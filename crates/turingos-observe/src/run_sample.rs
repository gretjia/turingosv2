use crate::{RunDelta, RunExport, RunSummary};

#[derive(Debug)]
pub struct RunSample<'a, QState> {
    current: RunSummary<'a, QState>,
    delta: Option<RunDelta>,
}

impl<QState> Copy for RunSample<'_, QState> {}

impl<QState> Clone for RunSample<'_, QState> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, QState> RunSample<'a, QState> {
    pub fn current(&self) -> RunSummary<'a, QState> {
        self.current
    }

    pub fn delta(&self) -> Option<RunDelta> {
        self.delta
    }

    pub fn has_delta(&self) -> bool {
        self.delta.is_some()
    }

    pub fn has_any_change(&self) -> bool {
        self.delta.is_some_and(|delta| delta.has_any_change())
    }
}

impl<'a, QState> RunSample<'a, QState> {
    fn from_current(current: &'a RunExport<QState>) -> Self {
        Self {
            current: current.summary(),
            delta: None,
        }
    }

    fn from_previous(current: &'a RunExport<QState>, previous: RunSummary<'_, QState>) -> Self {
        Self {
            current: current.summary(),
            delta: Some(RunDelta::between(previous, current.summary())),
        }
    }

    #[cfg(test)]
    fn from_basis(current: &'a RunExport<QState>, previous: &crate::RunBasis) -> Self {
        Self {
            current: current.summary(),
            delta: Some(RunDelta::between_basis(previous, current.summary())),
        }
    }
}

impl<QState> RunExport<QState> {
    pub fn sample(&self) -> RunSample<'_, QState> {
        RunSample::from_current(self)
    }

    pub fn sample_after(&self, previous: RunSummary<'_, QState>) -> RunSample<'_, QState> {
        RunSample::from_previous(self, previous)
    }
}

#[cfg(test)]
mod tests {
    use std::ptr;

    use turingos_adapter::AdapterError;
    use turingos_core::{
        CommitRecord, Head, StepIndex, StepObservation, TapeState, TraceHash, UniverseSnapshot,
    };
    use turingos_kernel::{CommittedStep, HaltStatus};

    use crate::{
        CountDelta, RunDelta, RunExport, RunExportStop, RunSample, RunSummaryStopClass, StepDelta,
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

    #[test]
    fn current_only_sample_has_no_delta() {
        let current = RunExport {
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

        let sample = current.sample();

        assert!(!sample.has_delta());
        assert!(!sample.has_any_change());
        assert!(sample.delta().is_none());
        assert!(ptr::eq(
            sample.current().terminal_snapshot(),
            &current.terminal_snapshot
        ));
    }

    #[test]
    fn sample_after_previous_derives_exact_delta() {
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

        let sample = current.sample_after(previous.summary());

        assert!(sample.has_delta());
        assert!(sample.has_any_change());
        assert_eq!(sample.current().stop_class(), RunSummaryStopClass::Halted);
        assert_eq!(sample.current().halt_status(), Some(&HaltStatus::Success));
        assert_eq!(
            sample.delta(),
            Some(RunDelta {
                attempted_steps: CountDelta::Increased { by: 3 },
                committed_step_count: CountDelta::Increased { by: 1 },
                terminal_step: StepDelta::Increased { by: 1 },
                stop_class_changed: true,
                halt_status_changed: true,
            })
        );
    }

    #[test]
    fn sample_keeps_current_summary_authoritative_when_delta_exists() {
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

        let sample = current.sample_after(previous.summary());

        assert!(ptr::eq(
            sample.current().terminal_snapshot(),
            &current.terminal_snapshot
        ));
        assert_eq!(sample.current().halt_status(), Some(&HaltStatus::Success));
        assert_eq!(
            sample
                .delta()
                .expect("delta should exist")
                .halt_status_changed,
            true
        );
    }

    #[test]
    fn sample_after_previous_can_represent_no_change() {
        let current = RunExport {
            attempted_steps: 2,
            committed_steps: vec![committed_step(0, "first")],
            terminal_snapshot: snapshot(1, false),
            stop: RunExportStop::StepBudgetExhausted,
        };

        let sample = current.sample_after(current.summary());

        assert!(sample.has_delta());
        assert!(!sample.has_any_change());
        assert_eq!(
            sample.delta(),
            Some(RunDelta {
                attempted_steps: CountDelta::Unchanged,
                committed_step_count: CountDelta::Unchanged,
                terminal_step: StepDelta::Unchanged,
                stop_class_changed: false,
                halt_status_changed: false,
            })
        );
    }

    #[test]
    fn sample_after_basis_matches_sample_after_summary() {
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

        let from_summary = current.sample_after(previous.summary());
        let from_basis = RunSample::from_basis(&current, &previous.basis());

        assert_eq!(
            from_basis.current().stop_class(),
            from_summary.current().stop_class()
        );
        assert_eq!(
            from_basis.current().halt_status(),
            from_summary.current().halt_status()
        );
        assert_eq!(
            from_basis.current().terminal_step(),
            from_summary.current().terminal_step()
        );
        assert_eq!(from_basis.delta(), from_summary.delta());
    }

    #[test]
    fn sample_after_basis_can_represent_no_change() {
        let current = RunExport {
            attempted_steps: 2,
            committed_steps: vec![committed_step(0, "first")],
            terminal_snapshot: snapshot(1, false),
            stop: RunExportStop::StepBudgetExhausted,
        };

        let sample = RunSample::from_basis(&current, &current.basis());

        assert!(sample.has_delta());
        assert!(!sample.has_any_change());
        assert_eq!(
            sample.delta(),
            Some(RunDelta {
                attempted_steps: CountDelta::Unchanged,
                committed_step_count: CountDelta::Unchanged,
                terminal_step: StepDelta::Unchanged,
                stop_class_changed: false,
                halt_status_changed: false,
            })
        );
    }
}
