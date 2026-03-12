use turingos_adapter::AdapterError;
use turingos_core::{AbortRecord, StepObservation, UniverseSnapshot};

use crate::run::{CommittedStep, HaltStatus, RunOutcome, RunStop};

#[derive(Clone, Copy, Debug)]
pub struct RunReport<'a, QState> {
    outcome: &'a RunOutcome<QState>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunReportStop<'a, QState> {
    Halted {
        status: &'a HaltStatus,
    },
    Abort {
        observation: &'a StepObservation,
        abort: &'a AbortRecord<QState>,
    },
    AdapterFault {
        observation: &'a StepObservation,
        error: &'a AdapterError,
    },
    StepBudgetExhausted,
}

impl<'a, QState> RunReport<'a, QState> {
    pub fn attempted_steps(&self) -> usize {
        self.outcome.attempted_steps
    }

    pub fn committed_steps(&self) -> &'a [CommittedStep] {
        self.outcome.committed_steps.as_slice()
    }

    pub fn terminal_snapshot(&self) -> &'a UniverseSnapshot<QState> {
        match &self.outcome.stop {
            RunStop::Halted { final_snapshot, .. } => final_snapshot,
            RunStop::Abort {
                preserved_snapshot, ..
            } => preserved_snapshot,
            RunStop::AdapterFault {
                preserved_snapshot, ..
            } => preserved_snapshot,
            RunStop::StepBudgetExhausted { last_snapshot } => last_snapshot,
        }
    }

    pub fn stop(&self) -> RunReportStop<'a, QState> {
        match &self.outcome.stop {
            RunStop::Halted { status, .. } => RunReportStop::Halted { status },
            RunStop::Abort {
                observation, abort, ..
            } => RunReportStop::Abort { observation, abort },
            RunStop::AdapterFault {
                observation, error, ..
            } => RunReportStop::AdapterFault { observation, error },
            RunStop::StepBudgetExhausted { .. } => RunReportStop::StepBudgetExhausted,
        }
    }
}

impl<'a, QState> From<&'a RunOutcome<QState>> for RunReport<'a, QState> {
    fn from(outcome: &'a RunOutcome<QState>) -> Self {
        Self { outcome }
    }
}

impl<QState> RunOutcome<QState> {
    pub fn report(&self) -> RunReport<'_, QState> {
        RunReport::from(self)
    }
}

#[cfg(test)]
mod tests {
    use turingos_adapter::AdapterError;
    use turingos_core::{
        AbortRecord, CommitRecord, Head, IntentEnvelope, RejectRecord, StepIndex, StepObservation,
        TapeState, TraceHash, UniverseSnapshot, WriteMode,
    };

    use crate::{
        report::{RunReport, RunReportStop},
        run::{CommittedStep, HaltStatus, RunOutcome, RunStop},
    };

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
            proposed_register: 7,
            action_payload: None,
            proposed_head: Head::new("root/reject.txt"),
            write_mode: WriteMode::Overwrite,
            write_content: Some("payload".to_owned()),
            halt: false,
            notes: "reject".to_owned(),
        }
    }

    #[test]
    fn borrowed_report_projects_halted_outcome_without_copying_truth() {
        let outcome = RunOutcome {
            attempted_steps: 2,
            committed_steps: vec![committed_step(0, "first"), committed_step(1, "second")],
            stop: RunStop::Halted {
                final_snapshot: snapshot(2, true),
                status: HaltStatus::Success,
            },
        };
        let report: RunReport<'_, u8> = (&outcome).into();

        assert_eq!(report.attempted_steps(), 2);
        assert_eq!(report.committed_steps(), outcome.committed_steps.as_slice());
        assert_eq!(report.terminal_snapshot(), &snapshot(2, true));
        assert_eq!(
            report.stop(),
            RunReportStop::Halted {
                status: &HaltStatus::Success,
            }
        );
        assert_eq!(outcome.committed_steps.len(), 2);
        assert_eq!(outcome.attempted_steps, 2);
    }

    #[test]
    fn borrowed_report_distinguishes_halted_without_success() {
        let outcome = RunOutcome {
            attempted_steps: 0,
            committed_steps: Vec::new(),
            stop: RunStop::Halted {
                final_snapshot: snapshot(7, true),
                status: HaltStatus::WithoutSuccess,
            },
        };
        let report = outcome.report();

        assert_eq!(report.committed_steps(), &[]);
        assert_eq!(report.terminal_snapshot(), &snapshot(7, true));
        assert_eq!(
            report.stop(),
            RunReportStop::Halted {
                status: &HaltStatus::WithoutSuccess,
            }
        );
    }

    #[test]
    fn borrowed_report_preserves_abort_snapshot_and_reject_record() {
        let abort = AbortRecord {
            reject: RejectRecord {
                step: StepIndex(3),
                reasons: vec!["bad".to_owned()],
            },
            rejected_intent: rejected_intent(),
        };
        let outcome = RunOutcome {
            attempted_steps: 4,
            committed_steps: vec![committed_step(0, "first")],
            stop: RunStop::Abort {
                preserved_snapshot: snapshot(3, false),
                observation: StepObservation {
                    provenance: "fixture-abort".to_owned(),
                },
                abort: abort.clone(),
            },
        };
        let report = outcome.report();

        assert_eq!(report.terminal_snapshot(), &snapshot(3, false));
        assert_eq!(
            report.stop(),
            RunReportStop::Abort {
                observation: &StepObservation {
                    provenance: "fixture-abort".to_owned(),
                },
                abort: &abort,
            }
        );
    }

    #[test]
    fn borrowed_report_preserves_adapter_fault_without_reclassification() {
        let error = AdapterError::Unavailable {
            detail: "timeout".to_owned(),
        };
        let outcome = RunOutcome {
            attempted_steps: 3,
            committed_steps: vec![committed_step(0, "first")],
            stop: RunStop::AdapterFault {
                preserved_snapshot: snapshot(1, false),
                observation: StepObservation {
                    provenance: "fixture-fault".to_owned(),
                },
                error: error.clone(),
            },
        };
        let report = outcome.report();

        assert_eq!(report.terminal_snapshot(), &snapshot(1, false));
        assert_eq!(
            report.stop(),
            RunReportStop::AdapterFault {
                observation: &StepObservation {
                    provenance: "fixture-fault".to_owned(),
                },
                error: &error,
            }
        );
    }

    #[test]
    fn borrowed_report_preserves_budget_stop_without_inventing_policy() {
        let outcome = RunOutcome {
            attempted_steps: 5,
            committed_steps: vec![committed_step(0, "first"), committed_step(1, "second")],
            stop: RunStop::StepBudgetExhausted {
                last_snapshot: snapshot(2, false),
            },
        };
        let report = outcome.report();

        assert_eq!(report.attempted_steps(), 5);
        assert_eq!(report.committed_steps(), outcome.committed_steps.as_slice());
        assert_eq!(report.terminal_snapshot(), &snapshot(2, false));
        assert_eq!(report.stop(), RunReportStop::StepBudgetExhausted);
    }
}
