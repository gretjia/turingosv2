use turingos_core::{StepIndex, UniverseSnapshot};
use turingos_kernel::{CommittedStep, HaltStatus};

use crate::{RunExport, RunExportStop};

#[derive(Debug)]
pub struct RunSummary<'a, QState> {
    export: &'a RunExport<QState>,
}

impl<QState> Copy for RunSummary<'_, QState> {}

impl<QState> Clone for RunSummary<'_, QState> {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunSummaryStopClass {
    Halted,
    Abort,
    AdapterFault,
    StepBudgetExhausted,
}

impl<'a, QState> RunSummary<'a, QState> {
    pub fn attempted_steps(&self) -> usize {
        self.export.attempted_steps
    }

    pub fn committed_step_count(&self) -> usize {
        self.export.committed_steps.len()
    }

    pub fn last_committed_step(&self) -> Option<&'a CommittedStep> {
        self.export.committed_steps.last()
    }

    pub fn terminal_snapshot(&self) -> &'a UniverseSnapshot<QState> {
        &self.export.terminal_snapshot
    }

    pub fn terminal_step(&self) -> StepIndex {
        self.export.terminal_snapshot.step()
    }

    pub fn stop_class(&self) -> RunSummaryStopClass {
        match &self.export.stop {
            RunExportStop::Halted { .. } => RunSummaryStopClass::Halted,
            RunExportStop::Abort { .. } => RunSummaryStopClass::Abort,
            RunExportStop::AdapterFault { .. } => RunSummaryStopClass::AdapterFault,
            RunExportStop::StepBudgetExhausted => RunSummaryStopClass::StepBudgetExhausted,
        }
    }

    pub fn halt_status(&self) -> Option<&'a HaltStatus> {
        match &self.export.stop {
            RunExportStop::Halted { status } => Some(status),
            RunExportStop::Abort { .. }
            | RunExportStop::AdapterFault { .. }
            | RunExportStop::StepBudgetExhausted => None,
        }
    }

    pub fn is_success(&self) -> bool {
        self.halt_status() == Some(&HaltStatus::Success)
    }
}

impl<'a, QState> From<&'a RunExport<QState>> for RunSummary<'a, QState> {
    fn from(export: &'a RunExport<QState>) -> Self {
        Self { export }
    }
}

impl<QState> RunExport<QState> {
    pub fn summary(&self) -> RunSummary<'_, QState> {
        RunSummary::from(self)
    }
}

#[cfg(test)]
mod tests {
    use std::ptr;

    use turingos_adapter::AdapterError;
    use turingos_core::{
        AbortRecord, CommitRecord, Head, IntentEnvelope, RejectRecord, StepIndex, StepObservation,
        TapeState, TraceHash, UniverseSnapshot, WriteMode,
    };
    use turingos_kernel::{CommittedStep, HaltStatus};

    use crate::{RunExport, RunExportStop, RunSummaryStopClass};

    fn snapshot(step: u64, _halted: bool) -> UniverseSnapshot<u8> {
        UniverseSnapshot::new(
            step as u8,
            Head::new(format!("root/{step}.txt")),
            TapeState::default(),
            TraceHash::new(format!("trace-{step}")),
            StepIndex(step),
        )
    }

    fn committed_step(step: u64, note: &str) -> CommittedStep {
        CommittedStep {
            observation: StepObservation {
                provenance: format!("fixture-{step}"),
            },
            record: CommitRecord {
                from_step: StepIndex(step),
                to_step: StepIndex(step + 1),
                next_head: Head::new(format!("root/next-{step}.txt")),
                next_ledger_tip: TraceHash::new(format!("ledger-{step}")),
                halt_requested: false,
                notes: note.to_owned(),
            },
        }
    }

    fn rejected_intent() -> IntentEnvelope<u8> {
        IntentEnvelope {
            proposed_register: 9,
            action_payload: None,
            proposed_head: Head::new("root/reject.txt"),
            write_mode: WriteMode::Overwrite,
            write_content: Some("payload".to_owned()),
            halt: false,
            notes: "reject".to_owned(),
        }
    }

    #[test]
    fn summary_projects_success_without_copying_export_truth() {
        let export = RunExport {
            attempted_steps: 2,
            committed_steps: vec![committed_step(0, "first"), committed_step(1, "second")],
            terminal_snapshot: snapshot(2, true),
            stop: RunExportStop::Halted {
                status: HaltStatus::Success,
            },
        };

        let summary = export.summary();

        assert_eq!(summary.attempted_steps(), 2);
        assert_eq!(summary.committed_step_count(), 2);
        assert_eq!(summary.terminal_step(), StepIndex(2));
        assert!(summary.is_success());
        assert_eq!(summary.stop_class(), RunSummaryStopClass::Halted);
        assert_eq!(summary.halt_status(), Some(&HaltStatus::Success));
        assert!(ptr::eq(
            summary.terminal_snapshot(),
            &export.terminal_snapshot
        ));
        assert!(ptr::eq(
            summary
                .last_committed_step()
                .expect("success summary should expose last commit"),
            export
                .committed_steps
                .last()
                .expect("export should retain last commit"),
        ));
    }

    #[test]
    fn summary_distinguishes_halted_without_success() {
        let export = RunExport {
            attempted_steps: 0,
            committed_steps: Vec::new(),
            terminal_snapshot: snapshot(7, true),
            stop: RunExportStop::Halted {
                status: HaltStatus::WithoutSuccess,
            },
        };

        let summary = export.summary();

        assert_eq!(summary.attempted_steps(), 0);
        assert_eq!(summary.committed_step_count(), 0);
        assert_eq!(summary.terminal_step(), StepIndex(7));
        assert!(!summary.is_success());
        assert_eq!(summary.stop_class(), RunSummaryStopClass::Halted);
        assert_eq!(summary.halt_status(), Some(&HaltStatus::WithoutSuccess));
        assert!(summary.last_committed_step().is_none());
    }

    #[test]
    fn summary_distinguishes_abort_without_reclassifying_it() {
        let export = RunExport {
            attempted_steps: 4,
            committed_steps: vec![committed_step(0, "first")],
            terminal_snapshot: snapshot(3, false),
            stop: RunExportStop::Abort {
                observation: StepObservation {
                    provenance: "fixture-abort".to_owned(),
                },
                abort: AbortRecord {
                    reject: RejectRecord {
                        step: StepIndex(3),
                        reasons: vec!["bad".to_owned()],
                    },
                    rejected_intent: rejected_intent(),
                },
            },
        };

        let summary = export.summary();

        assert_eq!(summary.attempted_steps(), 4);
        assert_eq!(summary.committed_step_count(), 1);
        assert_eq!(summary.terminal_step(), StepIndex(3));
        assert_eq!(summary.stop_class(), RunSummaryStopClass::Abort);
        assert_eq!(summary.halt_status(), None);
        assert!(!summary.is_success());
    }

    #[test]
    fn summary_distinguishes_adapter_fault_without_reclassifying_it() {
        let export = RunExport {
            attempted_steps: 3,
            committed_steps: vec![committed_step(0, "first")],
            terminal_snapshot: snapshot(1, false),
            stop: RunExportStop::AdapterFault {
                observation: StepObservation {
                    provenance: "fixture-fault".to_owned(),
                },
                error: AdapterError::Unavailable {
                    detail: "timeout".to_owned(),
                },
            },
        };

        let summary = export.summary();

        assert_eq!(summary.attempted_steps(), 3);
        assert_eq!(summary.committed_step_count(), 1);
        assert_eq!(summary.terminal_step(), StepIndex(1));
        assert_eq!(summary.stop_class(), RunSummaryStopClass::AdapterFault);
        assert_eq!(summary.halt_status(), None);
        assert!(!summary.is_success());
    }

    #[test]
    fn summary_distinguishes_step_budget_exhaustion_without_policy() {
        let export = RunExport {
            attempted_steps: 5,
            committed_steps: vec![committed_step(0, "first"), committed_step(1, "second")],
            terminal_snapshot: snapshot(2, false),
            stop: RunExportStop::StepBudgetExhausted,
        };

        let summary = export.summary();

        assert_eq!(summary.attempted_steps(), 5);
        assert_eq!(summary.committed_step_count(), 2);
        assert_eq!(summary.terminal_step(), StepIndex(2));
        assert_eq!(
            summary.stop_class(),
            RunSummaryStopClass::StepBudgetExhausted
        );
        assert_eq!(summary.halt_status(), None);
        assert!(!summary.is_success());
    }
}
