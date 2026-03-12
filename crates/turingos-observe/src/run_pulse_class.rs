use turingos_kernel::HaltStatus;

use crate::{CountDelta, RunPulse, StepDelta};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunPulseTransitionClass {
    CurrentOnly,
    Unchanged,
    Progressed,
    Regressed,
    TerminalReclassified,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunPulseTerminalClass {
    HaltedSuccess,
    HaltedWithoutSuccess,
    Abort,
    AdapterFault,
    StepBudgetExhausted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RunPulseClass {
    transition: RunPulseTransitionClass,
    terminal: RunPulseTerminalClass,
}

impl RunPulseClass {
    pub fn transition(&self) -> RunPulseTransitionClass {
        self.transition
    }

    pub fn terminal(&self) -> RunPulseTerminalClass {
        self.terminal
    }
}

impl From<&RunPulse> for RunPulseClass {
    fn from(pulse: &RunPulse) -> Self {
        let transition = match pulse.delta() {
            None => RunPulseTransitionClass::CurrentOnly,
            Some(delta) if !delta.has_any_change() => RunPulseTransitionClass::Unchanged,
            Some(delta)
                if matches!(delta.attempted_steps, CountDelta::Decreased { .. })
                    || matches!(delta.committed_step_count, CountDelta::Decreased { .. })
                    || matches!(delta.terminal_step, StepDelta::Decreased { .. }) =>
            {
                RunPulseTransitionClass::Regressed
            }
            Some(delta)
                if matches!(delta.attempted_steps, CountDelta::Increased { .. })
                    || matches!(delta.committed_step_count, CountDelta::Increased { .. })
                    || matches!(delta.terminal_step, StepDelta::Increased { .. }) =>
            {
                RunPulseTransitionClass::Progressed
            }
            Some(_) => RunPulseTransitionClass::TerminalReclassified,
        };

        let terminal = match pulse.current().halt_status() {
            Some(HaltStatus::Success) => RunPulseTerminalClass::HaltedSuccess,
            Some(HaltStatus::WithoutSuccess) => RunPulseTerminalClass::HaltedWithoutSuccess,
            None => match pulse.current().stop_class() {
                crate::RunSummaryStopClass::Abort => RunPulseTerminalClass::Abort,
                crate::RunSummaryStopClass::AdapterFault => RunPulseTerminalClass::AdapterFault,
                crate::RunSummaryStopClass::StepBudgetExhausted => {
                    RunPulseTerminalClass::StepBudgetExhausted
                }
                crate::RunSummaryStopClass::Halted => {
                    unreachable!("halted stop class must carry halt status")
                }
            },
        };

        Self {
            transition,
            terminal,
        }
    }
}

impl RunPulse {
    pub fn class(&self) -> RunPulseClass {
        RunPulseClass::from(self)
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
        RunExport, RunExportStop, RunPulseTerminalClass, RunPulseTransitionClass,
        RunSummaryStopClass,
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
    fn current_only_pulse_classifies_without_transition() {
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

        let class = current.pulse().class();

        assert_eq!(class.transition(), RunPulseTransitionClass::CurrentOnly);
        assert_eq!(class.terminal(), RunPulseTerminalClass::AdapterFault);
    }

    #[test]
    fn unchanged_pulse_class_is_exact() {
        let current = RunExport {
            attempted_steps: 2,
            committed_steps: vec![committed_step(0, "first")],
            terminal_snapshot: snapshot(1, false),
            stop: RunExportStop::StepBudgetExhausted,
        };

        let class = current.pulse_after_basis(&current.basis()).class();

        assert_eq!(class.transition(), RunPulseTransitionClass::Unchanged);
        assert_eq!(class.terminal(), RunPulseTerminalClass::StepBudgetExhausted);
    }

    #[test]
    fn progressed_pulse_class_is_exact() {
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

        let class = current.pulse_after_basis(&previous.basis()).class();

        assert_eq!(class.transition(), RunPulseTransitionClass::Progressed);
        assert_eq!(class.terminal(), RunPulseTerminalClass::HaltedSuccess);
    }

    #[test]
    fn regressed_pulse_class_is_exact() {
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

        let class = current.pulse_after_basis(&previous.basis()).class();

        assert_eq!(class.transition(), RunPulseTransitionClass::Regressed);
        assert_eq!(class.terminal(), RunPulseTerminalClass::StepBudgetExhausted);
    }

    #[test]
    fn attempted_step_regression_classifies_as_regressed() {
        let previous = RunExport {
            attempted_steps: 5,
            committed_steps: vec![committed_step(0, "first")],
            terminal_snapshot: snapshot(1, false),
            stop: RunExportStop::StepBudgetExhausted,
        };
        let current = RunExport {
            attempted_steps: 2,
            committed_steps: vec![committed_step(0, "first")],
            terminal_snapshot: snapshot(1, false),
            stop: RunExportStop::StepBudgetExhausted,
        };

        let class = current.pulse_after_basis(&previous.basis()).class();

        assert_eq!(class.transition(), RunPulseTransitionClass::Regressed);
    }

    #[test]
    fn committed_step_regression_classifies_as_regressed() {
        let previous = RunExport {
            attempted_steps: 2,
            committed_steps: vec![committed_step(0, "first"), committed_step(1, "second")],
            terminal_snapshot: snapshot(2, false),
            stop: RunExportStop::StepBudgetExhausted,
        };
        let current = RunExport {
            attempted_steps: 2,
            committed_steps: vec![committed_step(0, "first")],
            terminal_snapshot: snapshot(2, false),
            stop: RunExportStop::StepBudgetExhausted,
        };

        let class = current.pulse_after_basis(&previous.basis()).class();

        assert_eq!(class.transition(), RunPulseTransitionClass::Regressed);
    }

    #[test]
    fn terminal_step_regression_classifies_as_regressed() {
        let previous = RunExport {
            attempted_steps: 2,
            committed_steps: vec![committed_step(0, "first")],
            terminal_snapshot: snapshot(3, false),
            stop: RunExportStop::StepBudgetExhausted,
        };
        let current = RunExport {
            attempted_steps: 2,
            committed_steps: vec![committed_step(0, "first")],
            terminal_snapshot: snapshot(1, false),
            stop: RunExportStop::StepBudgetExhausted,
        };

        let class = current.pulse_after_basis(&previous.basis()).class();

        assert_eq!(class.transition(), RunPulseTransitionClass::Regressed);
    }

    #[test]
    fn mixed_sign_delta_prefers_regressed_classification() {
        let previous = RunExport {
            attempted_steps: 2,
            committed_steps: vec![committed_step(0, "first"), committed_step(1, "second")],
            terminal_snapshot: snapshot(2, false),
            stop: RunExportStop::StepBudgetExhausted,
        };
        let current = RunExport {
            attempted_steps: 5,
            committed_steps: vec![committed_step(0, "first")],
            terminal_snapshot: snapshot(3, false),
            stop: RunExportStop::StepBudgetExhausted,
        };

        let class = current.pulse_after_basis(&previous.basis()).class();

        assert_eq!(class.transition(), RunPulseTransitionClass::Regressed);
    }

    #[test]
    fn halt_status_only_change_is_terminal_reclassification() {
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

        let class = current.pulse_after_basis(&previous.basis()).class();

        assert_eq!(
            class.transition(),
            RunPulseTransitionClass::TerminalReclassified
        );
        assert_eq!(class.terminal(), RunPulseTerminalClass::HaltedSuccess);
    }

    #[test]
    fn stop_class_only_change_is_terminal_reclassification() {
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
                error: AdapterError::Unavailable {
                    detail: "timeout".to_owned(),
                },
            },
        };

        let class = current.pulse_after_basis(&previous.basis()).class();

        assert_eq!(
            class.transition(),
            RunPulseTransitionClass::TerminalReclassified
        );
        assert_eq!(class.terminal(), RunPulseTerminalClass::AdapterFault);
    }

    #[test]
    fn abort_and_fault_terminal_classes_stay_faithful_to_pulse_facts() {
        let abort_export = RunExport {
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
        let fault_export = RunExport {
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

        assert_eq!(
            abort_export.summary().stop_class(),
            RunSummaryStopClass::Abort
        );
        assert_eq!(
            fault_export.summary().stop_class(),
            RunSummaryStopClass::AdapterFault
        );
        assert_eq!(
            abort_export.pulse().class().terminal(),
            RunPulseTerminalClass::Abort
        );
        assert_eq!(
            fault_export.pulse().class().terminal(),
            RunPulseTerminalClass::AdapterFault
        );
    }

    #[test]
    fn classifier_does_not_invent_rate_or_retry_policy() {
        let current = RunExport {
            attempted_steps: 7,
            committed_steps: vec![committed_step(0, "first"), committed_step(1, "second")],
            terminal_snapshot: snapshot(2, false),
            stop: RunExportStop::StepBudgetExhausted,
        };

        let class = current.pulse().class();

        assert_eq!(class.terminal(), RunPulseTerminalClass::StepBudgetExhausted);
        assert_eq!(class.transition(), RunPulseTransitionClass::CurrentOnly);
    }
}
