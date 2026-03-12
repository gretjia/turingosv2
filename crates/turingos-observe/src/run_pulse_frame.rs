use crate::{RunBasis, RunExport, RunPulse, RunPulseClass};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunPulseFrame {
    pulse: RunPulse,
    class: RunPulseClass,
}

impl RunPulseFrame {
    pub fn pulse(&self) -> &RunPulse {
        &self.pulse
    }

    pub fn class(&self) -> RunPulseClass {
        self.class
    }

    pub fn basis(&self) -> &RunBasis {
        self.pulse.current()
    }

    fn from_pulse(pulse: RunPulse) -> Self {
        Self {
            class: pulse.class(),
            pulse,
        }
    }
}

impl<QState> RunExport<QState> {
    pub fn pulse_frame(&self) -> RunPulseFrame {
        RunPulseFrame::from_pulse(self.pulse())
    }

    pub fn pulse_frame_after_basis(&self, previous: &RunBasis) -> RunPulseFrame {
        RunPulseFrame::from_pulse(self.pulse_after_basis(previous))
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

    use crate::{
        CountDelta, RunExport, RunExportStop, RunPulseTerminalClass, RunPulseTransitionClass,
        StepDelta,
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
    fn current_only_pulse_frame_matches_canonical_split_path() {
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

        let frame = export.pulse_frame();
        let pulse = export.pulse();

        assert_eq!(frame.pulse(), &pulse);
        assert_eq!(frame.basis(), &export.basis());
        assert!(frame.pulse().delta().is_none());
        assert_eq!(frame.class(), pulse.class());
        assert_eq!(
            frame.class().transition(),
            RunPulseTransitionClass::CurrentOnly
        );
        assert_eq!(
            frame.class().terminal(),
            RunPulseTerminalClass::AdapterFault
        );
    }

    #[test]
    fn pulse_frame_after_basis_matches_canonical_split_path() {
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

        let frame = current.pulse_frame_after_basis(&previous.basis());
        let pulse = current.pulse_after_basis(&previous.basis());

        assert_eq!(frame.pulse(), &pulse);
        assert_eq!(frame.basis(), &current.basis());
        assert_eq!(frame.pulse().delta(), pulse.delta());
        assert_eq!(frame.class(), pulse.class());
        assert_eq!(
            frame.class().transition(),
            RunPulseTransitionClass::Progressed
        );
        assert_eq!(
            frame.class().terminal(),
            RunPulseTerminalClass::HaltedSuccess
        );
        assert_eq!(
            frame.pulse().delta(),
            Some(crate::RunDelta {
                attempted_steps: CountDelta::Increased { by: 3 },
                committed_step_count: CountDelta::Increased { by: 1 },
                terminal_step: StepDelta::Increased { by: 1 },
                stop_class_changed: true,
                halt_status_changed: true,
            })
        );
    }

    #[test]
    fn unchanged_pulse_frame_after_basis_matches_canonical_split_path() {
        let current = RunExport {
            attempted_steps: 2,
            committed_steps: vec![committed_step(0, "first")],
            terminal_snapshot: snapshot(1, false),
            stop: RunExportStop::StepBudgetExhausted,
        };

        let frame = current.pulse_frame_after_basis(&current.basis());
        let pulse = current.pulse_after_basis(&current.basis());

        assert_eq!(frame.pulse(), &pulse);
        assert_eq!(frame.basis(), &current.basis());
        assert_eq!(frame.class(), pulse.class());
        assert_eq!(
            frame.class().transition(),
            RunPulseTransitionClass::Unchanged
        );
    }

    #[test]
    fn pulse_frame_basis_is_exact_current_basis_without_parallel_truth() {
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

        let frame = current.pulse_frame_after_basis(&previous.basis());

        assert!(std::ptr::eq(frame.basis(), frame.pulse().current()));
        assert_eq!(frame.basis(), frame.pulse().current());
        assert_eq!(frame.basis(), &current.basis());
    }

    #[test]
    fn pulse_frame_preserves_regressed_classification_without_new_policy() {
        let previous = RunExport {
            attempted_steps: 5,
            committed_steps: vec![committed_step(0, "first"), committed_step(1, "second")],
            terminal_snapshot: snapshot(2, true),
            stop: RunExportStop::Halted {
                status: HaltStatus::Success,
            },
        };
        let current = RunExport {
            attempted_steps: 2,
            committed_steps: vec![committed_step(0, "first")],
            terminal_snapshot: snapshot(1, false),
            stop: RunExportStop::StepBudgetExhausted,
        };

        let frame = current.pulse_frame_after_basis(&previous.basis());

        assert_eq!(
            frame.class().transition(),
            RunPulseTransitionClass::Regressed
        );
        assert_eq!(
            frame.class().terminal(),
            RunPulseTerminalClass::StepBudgetExhausted
        );
    }

    #[test]
    fn pulse_frame_preserves_terminal_reclassification_without_new_policy() {
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
            terminal_snapshot: snapshot(3, false),
            stop: RunExportStop::AdapterFault {
                observation: StepObservation {
                    provenance: "fixture-fault".to_owned(),
                },
                error: AdapterError::MalformedOutput {
                    detail: "missing field".to_owned(),
                },
            },
        };

        let frame = current.pulse_frame_after_basis(&previous.basis());

        assert_eq!(
            frame.class().transition(),
            RunPulseTransitionClass::TerminalReclassified
        );
        assert_eq!(
            frame.class().terminal(),
            RunPulseTerminalClass::AdapterFault
        );
    }
}
