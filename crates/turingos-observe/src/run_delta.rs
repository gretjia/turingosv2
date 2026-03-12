use crate::{RunBasis, RunSummary};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CountDelta {
    Decreased { by: usize },
    Unchanged,
    Increased { by: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepDelta {
    Decreased { by: u64 },
    Unchanged,
    Increased { by: u64 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RunDelta {
    pub attempted_steps: CountDelta,
    pub committed_step_count: CountDelta,
    pub terminal_step: StepDelta,
    pub stop_class_changed: bool,
    pub halt_status_changed: bool,
}

impl RunDelta {
    pub fn between<QState>(
        previous: RunSummary<'_, QState>,
        current: RunSummary<'_, QState>,
    ) -> Self {
        Self {
            attempted_steps: count_delta(previous.attempted_steps(), current.attempted_steps()),
            committed_step_count: count_delta(
                previous.committed_step_count(),
                current.committed_step_count(),
            ),
            terminal_step: step_delta(previous.terminal_step().0, current.terminal_step().0),
            stop_class_changed: previous.stop_class() != current.stop_class(),
            halt_status_changed: previous.halt_status() != current.halt_status(),
        }
    }

    pub fn between_basis<QState>(previous: &RunBasis, current: RunSummary<'_, QState>) -> Self {
        Self {
            attempted_steps: count_delta(previous.attempted_steps(), current.attempted_steps()),
            committed_step_count: count_delta(
                previous.committed_step_count(),
                current.committed_step_count(),
            ),
            terminal_step: step_delta(previous.terminal_step().0, current.terminal_step().0),
            stop_class_changed: previous.stop_class() != current.stop_class(),
            halt_status_changed: previous.halt_status() != current.halt_status(),
        }
    }

    pub fn has_any_change(&self) -> bool {
        self.attempted_steps != CountDelta::Unchanged
            || self.committed_step_count != CountDelta::Unchanged
            || self.terminal_step != StepDelta::Unchanged
            || self.stop_class_changed
            || self.halt_status_changed
    }
}

fn count_delta(previous: usize, current: usize) -> CountDelta {
    match current.cmp(&previous) {
        std::cmp::Ordering::Less => CountDelta::Decreased {
            by: previous - current,
        },
        std::cmp::Ordering::Equal => CountDelta::Unchanged,
        std::cmp::Ordering::Greater => CountDelta::Increased {
            by: current - previous,
        },
    }
}

fn step_delta(previous: u64, current: u64) -> StepDelta {
    match current.cmp(&previous) {
        std::cmp::Ordering::Less => StepDelta::Decreased {
            by: previous - current,
        },
        std::cmp::Ordering::Equal => StepDelta::Unchanged,
        std::cmp::Ordering::Greater => StepDelta::Increased {
            by: current - previous,
        },
    }
}

#[cfg(test)]
mod tests {
    use turingos_adapter::AdapterError;
    use turingos_core::{
        AbortRecord, CommitRecord, Head, IntentEnvelope, RejectRecord, StepIndex, StepObservation,
        TapeState, TraceHash, UniverseSnapshot, WriteMode,
    };
    use turingos_kernel::{CommittedStep, HaltStatus};

    use crate::{CountDelta, RunDelta, RunExport, RunExportStop, RunSummaryStopClass, StepDelta};

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
    fn delta_detects_no_change_exactly() {
        let previous = RunExport {
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
        let current = previous.clone();

        let delta = RunDelta::between(previous.summary(), current.summary());

        assert_eq!(delta.attempted_steps, CountDelta::Unchanged);
        assert_eq!(delta.committed_step_count, CountDelta::Unchanged);
        assert_eq!(delta.terminal_step, StepDelta::Unchanged);
        assert!(!delta.stop_class_changed);
        assert!(!delta.halt_status_changed);
        assert!(!delta.has_any_change());
    }

    #[test]
    fn delta_detects_forward_progress_without_status_change() {
        let previous = RunExport {
            attempted_steps: 2,
            committed_steps: vec![committed_step(0, "first")],
            terminal_snapshot: snapshot(1, false),
            stop: RunExportStop::StepBudgetExhausted,
        };
        let current = RunExport {
            attempted_steps: 5,
            committed_steps: vec![committed_step(0, "first"), committed_step(1, "second")],
            terminal_snapshot: snapshot(2, false),
            stop: RunExportStop::StepBudgetExhausted,
        };

        let delta = RunDelta::between(previous.summary(), current.summary());

        assert_eq!(delta.attempted_steps, CountDelta::Increased { by: 3 });
        assert_eq!(delta.committed_step_count, CountDelta::Increased { by: 1 });
        assert_eq!(delta.terminal_step, StepDelta::Increased { by: 1 });
        assert!(!delta.stop_class_changed);
        assert!(!delta.halt_status_changed);
        assert!(delta.has_any_change());
    }

    #[test]
    fn delta_detects_stop_class_transition() {
        let previous = RunExport {
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
        let current = RunExport {
            attempted_steps: 4,
            committed_steps: vec![committed_step(0, "first")],
            terminal_snapshot: snapshot(4, true),
            stop: RunExportStop::Halted {
                status: HaltStatus::Success,
            },
        };

        let delta = RunDelta::between(previous.summary(), current.summary());

        assert_eq!(previous.summary().stop_class(), RunSummaryStopClass::Abort);
        assert_eq!(current.summary().stop_class(), RunSummaryStopClass::Halted);
        assert!(delta.stop_class_changed);
        assert!(delta.halt_status_changed);
        assert_eq!(delta.terminal_step, StepDelta::Increased { by: 1 });
        assert!(delta.has_any_change());
    }

    #[test]
    fn delta_detects_halt_status_change_inside_same_stop_kind() {
        let previous = RunExport {
            attempted_steps: 1,
            committed_steps: vec![committed_step(0, "first")],
            terminal_snapshot: snapshot(1, true),
            stop: RunExportStop::Halted {
                status: HaltStatus::WithoutSuccess,
            },
        };
        let current = RunExport {
            attempted_steps: 1,
            committed_steps: vec![committed_step(0, "first")],
            terminal_snapshot: snapshot(1, true),
            stop: RunExportStop::Halted {
                status: HaltStatus::Success,
            },
        };

        let delta = RunDelta::between(previous.summary(), current.summary());

        assert_eq!(delta.attempted_steps, CountDelta::Unchanged);
        assert_eq!(delta.committed_step_count, CountDelta::Unchanged);
        assert_eq!(delta.terminal_step, StepDelta::Unchanged);
        assert!(!delta.stop_class_changed);
        assert!(delta.halt_status_changed);
        assert!(delta.has_any_change());
    }

    #[test]
    fn delta_detects_regression_exactly_without_policy() {
        let previous = RunExport {
            attempted_steps: 5,
            committed_steps: vec![committed_step(0, "first"), committed_step(1, "second")],
            terminal_snapshot: snapshot(2, false),
            stop: RunExportStop::StepBudgetExhausted,
        };
        let current = RunExport {
            attempted_steps: 3,
            committed_steps: vec![committed_step(0, "first")],
            terminal_snapshot: snapshot(1, false),
            stop: RunExportStop::AdapterFault {
                observation: StepObservation {
                    provenance: "fixture-fault".to_owned(),
                },
                error: AdapterError::MalformedOutput {
                    detail: "missing field".to_owned(),
                },
            },
        };

        let delta = RunDelta::between(previous.summary(), current.summary());

        assert_eq!(delta.attempted_steps, CountDelta::Decreased { by: 2 });
        assert_eq!(delta.committed_step_count, CountDelta::Decreased { by: 1 });
        assert_eq!(delta.terminal_step, StepDelta::Decreased { by: 1 });
        assert!(delta.stop_class_changed);
        assert!(!delta.halt_status_changed);
        assert!(delta.has_any_change());
    }

    #[test]
    fn delta_from_basis_matches_delta_from_previous_summary() {
        let previous = RunExport {
            attempted_steps: 2,
            committed_steps: vec![committed_step(0, "first")],
            terminal_snapshot: snapshot(1, false),
            stop: RunExportStop::StepBudgetExhausted,
        };
        let current = RunExport {
            attempted_steps: 5,
            committed_steps: vec![committed_step(0, "first"), committed_step(1, "second")],
            terminal_snapshot: snapshot(2, true),
            stop: RunExportStop::Halted {
                status: HaltStatus::Success,
            },
        };

        assert_eq!(
            RunDelta::between_basis(&previous.basis(), current.summary()),
            RunDelta::between(previous.summary(), current.summary()),
        );
    }

    #[test]
    fn delta_from_halted_basis_matches_delta_from_previous_summary() {
        let previous = RunExport {
            attempted_steps: 1,
            committed_steps: vec![committed_step(0, "first")],
            terminal_snapshot: snapshot(1, true),
            stop: RunExportStop::Halted {
                status: HaltStatus::WithoutSuccess,
            },
        };
        let current = RunExport {
            attempted_steps: 1,
            committed_steps: vec![committed_step(0, "first")],
            terminal_snapshot: snapshot(1, true),
            stop: RunExportStop::Halted {
                status: HaltStatus::Success,
            },
        };

        let from_basis = RunDelta::between_basis(&previous.basis(), current.summary());
        let from_summary = RunDelta::between(previous.summary(), current.summary());

        assert_eq!(from_basis, from_summary);
        assert_eq!(from_basis.attempted_steps, CountDelta::Unchanged);
        assert_eq!(from_basis.committed_step_count, CountDelta::Unchanged);
        assert_eq!(from_basis.terminal_step, StepDelta::Unchanged);
        assert!(!from_basis.stop_class_changed);
        assert!(from_basis.halt_status_changed);
    }
}
