use turingos_core::StepIndex;
use turingos_kernel::HaltStatus;

use crate::{RunExport, RunSummary, RunSummaryStopClass};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunBasis {
    attempted_steps: usize,
    committed_step_count: usize,
    terminal_step: StepIndex,
    stop_class: RunSummaryStopClass,
    halt_status: Option<HaltStatus>,
}

impl RunBasis {
    pub fn attempted_steps(&self) -> usize {
        self.attempted_steps
    }

    pub fn committed_step_count(&self) -> usize {
        self.committed_step_count
    }

    pub fn terminal_step(&self) -> StepIndex {
        self.terminal_step
    }

    pub fn stop_class(&self) -> RunSummaryStopClass {
        self.stop_class
    }

    pub fn halt_status(&self) -> Option<&HaltStatus> {
        self.halt_status.as_ref()
    }
}

impl<QState> From<RunSummary<'_, QState>> for RunBasis {
    fn from(summary: RunSummary<'_, QState>) -> Self {
        Self {
            attempted_steps: summary.attempted_steps(),
            committed_step_count: summary.committed_step_count(),
            terminal_step: summary.terminal_step(),
            stop_class: summary.stop_class(),
            halt_status: summary.halt_status().cloned(),
        }
    }
}

impl<QState> RunSummary<'_, QState> {
    pub fn basis(&self) -> RunBasis {
        RunBasis::from(*self)
    }
}

impl<QState> RunExport<QState> {
    pub fn basis(&self) -> RunBasis {
        self.summary().basis()
    }
}

#[cfg(test)]
mod tests {
    use turingos_adapter::AdapterError;
    use turingos_core::{
        CommitRecord, Head, StepIndex, StepObservation, TapeState, TraceHash, UniverseSnapshot,
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

    #[test]
    fn basis_captures_only_delta_relevant_facts() {
        let export = RunExport {
            attempted_steps: 5,
            committed_steps: vec![committed_step(0, "first"), committed_step(1, "second")],
            terminal_snapshot: snapshot(2, true),
            stop: RunExportStop::Halted {
                status: HaltStatus::Success,
            },
        };

        let basis = export.basis();

        assert_eq!(basis.attempted_steps(), 5);
        assert_eq!(basis.committed_step_count(), 2);
        assert_eq!(basis.terminal_step(), StepIndex(2));
        assert_eq!(basis.stop_class(), RunSummaryStopClass::Halted);
        assert_eq!(basis.halt_status(), Some(&HaltStatus::Success));
    }

    #[test]
    fn export_basis_matches_summary_basis_exactly() {
        let export = RunExport {
            attempted_steps: 2,
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

        assert_eq!(export.basis(), export.summary().basis());
    }
}
