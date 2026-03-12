use turingos_adapter::AdapterError;
use turingos_core::{AbortRecord, StepObservation, UniverseSnapshot};
use turingos_kernel::{CommittedStep, HaltStatus, RunReport, RunReportStop};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunExport<QState> {
    pub attempted_steps: usize,
    pub committed_steps: Vec<CommittedStep>,
    pub terminal_snapshot: UniverseSnapshot<QState>,
    pub stop: RunExportStop<QState>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunExportStop<QState> {
    Halted {
        status: HaltStatus,
    },
    Abort {
        observation: StepObservation,
        abort: AbortRecord<QState>,
    },
    AdapterFault {
        observation: StepObservation,
        error: AdapterError,
    },
    StepBudgetExhausted,
}

impl<QState: Clone> From<RunReport<'_, QState>> for RunExport<QState> {
    fn from(report: RunReport<'_, QState>) -> Self {
        let stop = match report.stop() {
            RunReportStop::Halted { status } => RunExportStop::Halted {
                status: status.clone(),
            },
            RunReportStop::Abort { observation, abort } => RunExportStop::Abort {
                observation: observation.clone(),
                abort: abort.clone(),
            },
            RunReportStop::AdapterFault { observation, error } => RunExportStop::AdapterFault {
                observation: observation.clone(),
                error: error.clone(),
            },
            RunReportStop::StepBudgetExhausted => RunExportStop::StepBudgetExhausted,
        };

        Self {
            attempted_steps: report.attempted_steps(),
            committed_steps: report.committed_steps().to_vec(),
            terminal_snapshot: report.terminal_snapshot().clone(),
            stop,
        }
    }
}

#[cfg(test)]
mod tests {
    use turingos_adapter::AdapterError;
    use turingos_core::{
        AbortRecord, CommitRecord, Head, IntentEnvelope, RejectRecord, StepIndex, StepObservation,
        TapeState, TraceHash, UniverseSnapshot, WriteMode,
    };
    use turingos_kernel::{CommittedStep, HaltStatus, RunOutcome, RunStop};

    use crate::run_export::{RunExport, RunExportStop};

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
    fn export_from_borrowed_report_preserves_halted_history_and_terminal_snapshot() {
        let outcome = RunOutcome {
            attempted_steps: 2,
            committed_steps: vec![committed_step(0, "first"), committed_step(1, "second")],
            stop: RunStop::Halted {
                final_snapshot: snapshot(2, true),
                status: HaltStatus::Success,
            },
        };

        let export = RunExport::from(outcome.report());

        assert_eq!(export.attempted_steps, 2);
        assert_eq!(export.committed_steps, outcome.committed_steps);
        assert_eq!(export.terminal_snapshot, snapshot(2, true));
        assert_eq!(
            export.stop,
            RunExportStop::Halted {
                status: HaltStatus::Success,
            }
        );
    }

    #[test]
    fn export_from_borrowed_report_preserves_halted_without_success() {
        let outcome = RunOutcome {
            attempted_steps: 0,
            committed_steps: Vec::new(),
            stop: RunStop::Halted {
                final_snapshot: snapshot(7, true),
                status: HaltStatus::WithoutSuccess,
            },
        };

        let export = RunExport::from(outcome.report());

        assert_eq!(export.attempted_steps, 0);
        assert_eq!(export.committed_steps, outcome.committed_steps);
        assert_eq!(export.terminal_snapshot, snapshot(7, true));
        assert_eq!(
            export.stop,
            RunExportStop::Halted {
                status: HaltStatus::WithoutSuccess,
            }
        );
    }

    #[test]
    fn export_from_borrowed_report_preserves_abort_facts() {
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

        let export = RunExport::from(outcome.report());

        assert_eq!(export.attempted_steps, outcome.attempted_steps);
        assert_eq!(export.committed_steps, outcome.committed_steps);
        assert_eq!(export.terminal_snapshot, snapshot(3, false));
        assert_eq!(
            export.stop,
            RunExportStop::Abort {
                observation: StepObservation {
                    provenance: "fixture-abort".to_owned(),
                },
                abort,
            }
        );
    }

    #[test]
    fn export_preserves_adapter_fault_without_repair_logic() {
        let error = AdapterError::MalformedOutput {
            detail: "missing field".to_owned(),
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

        let export = RunExport::from(outcome.report());

        assert_eq!(export.attempted_steps, outcome.attempted_steps);
        assert_eq!(export.committed_steps, outcome.committed_steps);
        assert_eq!(export.terminal_snapshot, snapshot(1, false));
        assert_eq!(
            export.stop,
            RunExportStop::AdapterFault {
                observation: StepObservation {
                    provenance: "fixture-fault".to_owned(),
                },
                error,
            }
        );
    }

    #[test]
    fn export_preserves_step_budget_exhaustion_without_extra_policy() {
        let outcome = RunOutcome {
            attempted_steps: 5,
            committed_steps: vec![committed_step(0, "first"), committed_step(1, "second")],
            stop: RunStop::StepBudgetExhausted {
                last_snapshot: snapshot(2, false),
            },
        };

        let export = RunExport::from(outcome.report());

        assert_eq!(export.attempted_steps, 5);
        assert_eq!(export.committed_steps, outcome.committed_steps);
        assert_eq!(export.terminal_snapshot, snapshot(2, false));
        assert_eq!(export.stop, RunExportStop::StepBudgetExhausted);
    }
}
