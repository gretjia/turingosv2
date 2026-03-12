use crate::{RunBasis, RunDelta, RunExport};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunPulse {
    current: RunBasis,
    delta: Option<RunDelta>,
}

impl RunPulse {
    pub fn current(&self) -> &RunBasis {
        &self.current
    }

    pub fn delta(&self) -> Option<RunDelta> {
        self.delta
    }

    fn from_export<QState>(current: &RunExport<QState>) -> Self {
        Self {
            current: current.basis(),
            delta: None,
        }
    }

    fn from_export_after_basis<QState>(current: &RunExport<QState>, previous: &RunBasis) -> Self {
        Self {
            current: current.basis(),
            delta: Some(RunDelta::between_basis(previous, current.summary())),
        }
    }
}

impl<QState> RunExport<QState> {
    pub fn pulse(&self) -> RunPulse {
        RunPulse::from_export(self)
    }

    pub fn pulse_after_basis(&self, previous: &RunBasis) -> RunPulse {
        RunPulse::from_export_after_basis(self, previous)
    }
}

#[cfg(test)]
mod tests {
    use turingos_adapter::AdapterError;
    use turingos_core::{
        CommitRecord, Head, StepIndex, StepObservation, TapeState, TraceHash, UniverseSnapshot,
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

    #[test]
    fn current_only_pulse_has_no_delta() {
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

        let pulse = current.pulse();

        assert!(pulse.delta().is_none());
        assert_eq!(pulse.current(), &current.basis());
    }

    #[test]
    fn pulse_after_basis_derives_exact_delta() {
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

        let pulse = current.pulse_after_basis(&previous.basis());

        assert!(pulse.delta().is_some());
        assert!(pulse.delta().is_some_and(|delta| delta.has_any_change()));
        assert_eq!(pulse.current().stop_class(), RunSummaryStopClass::Halted);
        assert_eq!(pulse.current().halt_status(), Some(&HaltStatus::Success));
        assert_eq!(
            pulse.delta(),
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
    fn pulse_after_basis_keeps_current_basis_authoritative() {
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

        let pulse = current.pulse_after_basis(&previous.basis());

        assert_eq!(pulse.current(), &current.basis());
        assert_eq!(pulse.current().halt_status(), Some(&HaltStatus::Success));
        assert_eq!(
            pulse
                .delta()
                .expect("delta should exist")
                .halt_status_changed,
            true
        );
    }

    #[test]
    fn pulse_after_basis_can_represent_no_change() {
        let current = RunExport {
            attempted_steps: 2,
            committed_steps: vec![committed_step(0, "first")],
            terminal_snapshot: snapshot(1, false),
            stop: RunExportStop::StepBudgetExhausted,
        };

        let pulse = current.pulse_after_basis(&current.basis());

        assert!(pulse.delta().is_some());
        assert!(!pulse.delta().is_some_and(|delta| delta.has_any_change()));
        assert_eq!(
            pulse.delta(),
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
    fn pulse_after_basis_matches_sample_after_basis() {
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

        let pulse = current.pulse_after_basis(&previous.basis());
        let sample = current.sample_after(previous.summary());

        assert_eq!(pulse.current(), &current.basis());
        assert_eq!(pulse.current(), &sample.current().basis());
        assert_eq!(pulse.delta(), sample.delta());
    }
}
