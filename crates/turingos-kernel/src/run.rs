#![cfg_attr(not(test), allow(dead_code))]

use std::fmt::Debug;

use turingos_adapter::{AdapterError, IntentAdapter};
use turingos_core::{AbortRecord, CommitRecord, StepObservation, UniverseSnapshot};

use crate::{driver::StepDriverOutcome, task::TaskContract, KernelEngine};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunOutcome<QState> {
    pub attempted_steps: usize,
    pub committed_steps: Vec<CommittedStep>,
    pub stop: RunStop<QState>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommittedStep {
    pub observation: StepObservation,
    pub record: CommitRecord,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HaltStatus {
    Success,
    WithoutSuccess,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunStop<QState> {
    Halted {
        final_snapshot: UniverseSnapshot<QState>,
        status: HaltStatus,
    },
    Abort {
        preserved_snapshot: UniverseSnapshot<QState>,
        observation: StepObservation,
        abort: AbortRecord<QState>,
    },
    AdapterFault {
        preserved_snapshot: UniverseSnapshot<QState>,
        observation: StepObservation,
        error: AdapterError,
    },
    StepBudgetExhausted {
        last_snapshot: UniverseSnapshot<QState>,
    },
}

impl KernelEngine {
    pub fn run<QState: Clone + Debug>(
        &self,
        initial_snapshot: UniverseSnapshot<QState>,
        adapter: &dyn IntentAdapter<QState>,
        contract: &dyn TaskContract<QState>,
        max_steps: usize,
    ) -> RunOutcome<QState> {
        if contract.is_success(initial_snapshot.world()) {
            return RunOutcome {
                attempted_steps: 0,
                committed_steps: Vec::new(),
                stop: RunStop::Halted {
                    final_snapshot: initial_snapshot,
                    status: HaltStatus::Success,
                },
            };
        }

        let mut snapshot = initial_snapshot;
        let mut attempted_steps = 0;
        let mut committed_steps = Vec::new();

        while attempted_steps < max_steps {
            attempted_steps += 1;

            match self.step(&snapshot, adapter, contract) {
                StepDriverOutcome::Applied {
                    observation,
                    outcome,
                } => match outcome {
                    turingos_core::CommitOutcome::Commit { next, record } => {
                        let halt_requested = record.halt_requested;
                        committed_steps.push(CommittedStep {
                            observation,
                            record,
                        });

                        if halt_requested {
                            let status = if contract.is_success(next.world()) {
                                HaltStatus::Success
                            } else {
                                HaltStatus::WithoutSuccess
                            };
                            return RunOutcome {
                                attempted_steps,
                                committed_steps,
                                stop: RunStop::Halted {
                                    final_snapshot: next,
                                    status,
                                },
                            };
                        }

                        snapshot = next;
                    }
                    turingos_core::CommitOutcome::Abort { abort } => {
                        return RunOutcome {
                            attempted_steps,
                            committed_steps,
                            stop: RunStop::Abort {
                                preserved_snapshot: snapshot,
                                observation,
                                abort,
                            },
                        };
                    }
                },
                StepDriverOutcome::AdapterFault {
                    observation, error, ..
                } => {
                    return RunOutcome {
                        attempted_steps,
                        committed_steps,
                        stop: RunStop::AdapterFault {
                            preserved_snapshot: snapshot,
                            observation,
                            error,
                        },
                    };
                }
            }
        }

        RunOutcome {
            attempted_steps,
            committed_steps,
            stop: RunStop::StepBudgetExhausted {
                last_snapshot: snapshot,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use turingos_adapter::{AdapterError, AdapterOutcome, IntentAdapter};
    use turingos_core::{
        Head, IntentEnvelope, PredicateVerdict, ReadView, StepIndex, StepObservation, TapeState,
        TraceHash, UniverseSnapshot, WorldState, WriteMode,
    };

    use crate::{
        config::KernelConfig,
        run::{HaltStatus, RunStop},
        task::TaskContract,
        KernelEngine,
    };

    struct SuccessOnHalt;
    struct NeverSuccess;
    struct Rejects;
    struct RejectOverwriteOnly;
    struct PanicsIfEvaluated;
    struct HaltCommitAdapter;
    struct KeepRunningAdapter;
    struct RejectingIntentAdapter;
    struct FaultAdapter;
    struct PanicsIfProposed;
    struct CommitThenFaultAdapter {
        calls: Cell<u8>,
    }
    struct CommitThenRejectAdapter {
        calls: Cell<u8>,
    }

    impl TaskContract<u8> for SuccessOnHalt {
        fn evaluate(
            &self,
            _world: &WorldState<u8>,
            _intent: &IntentEnvelope<u8>,
        ) -> PredicateVerdict {
            PredicateVerdict::pass()
        }

        fn is_success(&self, world: &WorldState<u8>) -> bool {
            *world.register() == 2
        }
    }

    impl TaskContract<u8> for NeverSuccess {
        fn evaluate(
            &self,
            _world: &WorldState<u8>,
            _intent: &IntentEnvelope<u8>,
        ) -> PredicateVerdict {
            PredicateVerdict::pass()
        }

        fn is_success(&self, _world: &WorldState<u8>) -> bool {
            false
        }
    }

    impl TaskContract<u8> for Rejects {
        fn evaluate(
            &self,
            _world: &WorldState<u8>,
            _intent: &IntentEnvelope<u8>,
        ) -> PredicateVerdict {
            PredicateVerdict::fail(vec!["bad".to_owned()])
        }

        fn is_success(&self, _world: &WorldState<u8>) -> bool {
            false
        }
    }

    impl TaskContract<u8> for RejectOverwriteOnly {
        fn evaluate(
            &self,
            _world: &WorldState<u8>,
            intent: &IntentEnvelope<u8>,
        ) -> PredicateVerdict {
            match intent.write_mode {
                WriteMode::Overwrite => PredicateVerdict::fail(vec!["bad".to_owned()]),
                WriteMode::Keep => PredicateVerdict::pass(),
            }
        }

        fn is_success(&self, _world: &WorldState<u8>) -> bool {
            false
        }
    }

    impl TaskContract<u8> for PanicsIfEvaluated {
        fn evaluate(
            &self,
            _world: &WorldState<u8>,
            _intent: &IntentEnvelope<u8>,
        ) -> PredicateVerdict {
            panic!("adapter fault must not reach predicate evaluation")
        }

        fn is_success(&self, _world: &WorldState<u8>) -> bool {
            false
        }
    }

    impl IntentAdapter<u8> for HaltCommitAdapter {
        fn propose(&self, view: &ReadView<u8>) -> AdapterOutcome<u8> {
            AdapterOutcome::Intent {
                observation: StepObservation {
                    provenance: "fixture".to_owned(),
                },
                intent: IntentEnvelope {
                    proposed_register: view.register + 1,
                    action_payload: None,
                    proposed_head: Head::new("root/done.txt"),
                    write_mode: WriteMode::Keep,
                    write_content: None,
                    halt: true,
                    notes: "halt".to_owned(),
                },
            }
        }
    }

    impl IntentAdapter<u8> for KeepRunningAdapter {
        fn propose(&self, view: &ReadView<u8>) -> AdapterOutcome<u8> {
            AdapterOutcome::Intent {
                observation: StepObservation {
                    provenance: "fixture".to_owned(),
                },
                intent: IntentEnvelope {
                    proposed_register: view.register + 1,
                    action_payload: None,
                    proposed_head: view.head.clone(),
                    write_mode: WriteMode::Keep,
                    write_content: None,
                    halt: false,
                    notes: "keep-going".to_owned(),
                },
            }
        }
    }

    impl IntentAdapter<u8> for RejectingIntentAdapter {
        fn propose(&self, view: &ReadView<u8>) -> AdapterOutcome<u8> {
            AdapterOutcome::Intent {
                observation: StepObservation {
                    provenance: "fixture".to_owned(),
                },
                intent: IntentEnvelope {
                    proposed_register: view.register + 1,
                    action_payload: None,
                    proposed_head: Head::new("root/reject.txt"),
                    write_mode: WriteMode::Overwrite,
                    write_content: Some("payload".to_owned()),
                    halt: false,
                    notes: "reject".to_owned(),
                },
            }
        }
    }

    impl IntentAdapter<u8> for FaultAdapter {
        fn propose(&self, _view: &ReadView<u8>) -> AdapterOutcome<u8> {
            AdapterOutcome::Fault {
                observation: StepObservation {
                    provenance: "fixture".to_owned(),
                },
                error: AdapterError::Unavailable {
                    detail: "timeout".to_owned(),
                },
            }
        }
    }

    impl IntentAdapter<u8> for PanicsIfProposed {
        fn propose(&self, _view: &ReadView<u8>) -> AdapterOutcome<u8> {
            panic!("run must not call adapter when the initial snapshot is already halted")
        }
    }

    impl IntentAdapter<u8> for CommitThenFaultAdapter {
        fn propose(&self, view: &ReadView<u8>) -> AdapterOutcome<u8> {
            let call = self.calls.get();
            self.calls.set(call + 1);

            if call == 0 {
                AdapterOutcome::Intent {
                    observation: StepObservation {
                        provenance: "fixture".to_owned(),
                    },
                    intent: IntentEnvelope {
                        proposed_register: view.register + 1,
                        action_payload: None,
                        proposed_head: view.head.clone(),
                        write_mode: WriteMode::Keep,
                        write_content: None,
                        halt: false,
                        notes: "keep-going".to_owned(),
                    },
                }
            } else {
                AdapterOutcome::Fault {
                    observation: StepObservation {
                        provenance: "fixture".to_owned(),
                    },
                    error: AdapterError::Unavailable {
                        detail: "timeout".to_owned(),
                    },
                }
            }
        }
    }

    impl IntentAdapter<u8> for CommitThenRejectAdapter {
        fn propose(&self, view: &ReadView<u8>) -> AdapterOutcome<u8> {
            let call = self.calls.get();
            self.calls.set(call + 1);

            if call == 0 {
                AdapterOutcome::Intent {
                    observation: StepObservation {
                        provenance: "fixture".to_owned(),
                    },
                    intent: IntentEnvelope {
                        proposed_register: view.register + 1,
                        action_payload: None,
                        proposed_head: view.head.clone(),
                        write_mode: WriteMode::Keep,
                        write_content: None,
                        halt: false,
                        notes: "keep-going".to_owned(),
                    },
                }
            } else {
                AdapterOutcome::Intent {
                    observation: StepObservation {
                        provenance: "fixture".to_owned(),
                    },
                    intent: IntentEnvelope {
                        proposed_register: view.register + 1,
                        action_payload: None,
                        proposed_head: Head::new("root/reject.txt"),
                        write_mode: WriteMode::Overwrite,
                        write_content: Some("payload".to_owned()),
                        halt: false,
                        notes: "reject".to_owned(),
                    },
                }
            }
        }
    }

    fn seed_snapshot() -> UniverseSnapshot<u8> {
        UniverseSnapshot::new(
            1_u8,
            Head::new("root/current.txt"),
            TapeState::default(),
            TraceHash::genesis(),
            StepIndex(0),
        )
    }

    fn success_snapshot() -> UniverseSnapshot<u8> {
        UniverseSnapshot::new(
            2_u8,
            Head::new("root/done.txt"),
            TapeState::default(),
            TraceHash::genesis(),
            StepIndex(7),
        )
    }

    #[test]
    fn run_short_circuits_when_initial_snapshot_is_already_successful() {
        let engine = KernelEngine::new(KernelConfig::default());
        let outcome = engine.run(success_snapshot(), &PanicsIfProposed, &SuccessOnHalt, 3);

        assert_eq!(outcome.attempted_steps, 0);
        assert!(outcome.committed_steps.is_empty());
        match outcome.stop {
            RunStop::Halted {
                final_snapshot,
                status,
            } => {
                assert_eq!(status, HaltStatus::Success);
                assert_eq!(final_snapshot.step(), StepIndex(7));
                assert_eq!(final_snapshot.register(), &2_u8);
            }
            _ => panic!("expected already successful"),
        }
    }

    #[test]
    fn run_stops_on_halted_success() {
        let engine = KernelEngine::new(KernelConfig::default());
        let outcome = engine.run(seed_snapshot(), &HaltCommitAdapter, &SuccessOnHalt, 3);

        assert_eq!(outcome.attempted_steps, 1);
        assert_eq!(outcome.committed_steps.len(), 1);
        assert_eq!(
            outcome.committed_steps[0].observation,
            StepObservation {
                provenance: "fixture".to_owned(),
            }
        );
        assert_eq!(outcome.committed_steps[0].record.from_step, StepIndex(0));
        assert_eq!(outcome.committed_steps[0].record.to_step, StepIndex(1));
        assert_eq!(
            outcome.committed_steps[0].record.next_head,
            Head::new("root/done.txt")
        );
        assert_eq!(outcome.committed_steps[0].record.notes, "halt");
        match outcome.stop {
            RunStop::Halted {
                final_snapshot,
                status,
            } => {
                assert_eq!(status, HaltStatus::Success);
                assert_eq!(final_snapshot.step(), StepIndex(1));
                assert_eq!(final_snapshot.register(), &2_u8);
            }
            _ => panic!("expected halted success"),
        }
    }

    #[test]
    fn run_stops_on_halted_without_success() {
        let engine = KernelEngine::new(KernelConfig::default());
        let outcome = engine.run(seed_snapshot(), &HaltCommitAdapter, &NeverSuccess, 3);

        assert_eq!(outcome.attempted_steps, 1);
        assert_eq!(outcome.committed_steps.len(), 1);
        assert_eq!(outcome.committed_steps[0].record.to_step, StepIndex(1));
        match outcome.stop {
            RunStop::Halted {
                final_snapshot,
                status,
            } => {
                assert_eq!(status, HaltStatus::WithoutSuccess);
                assert_eq!(final_snapshot.step(), StepIndex(1));
                assert_eq!(final_snapshot.register(), &2_u8);
            }
            _ => panic!("expected halted without success"),
        }
    }

    #[test]
    fn run_stops_on_adapter_fault_without_predicate_evaluation() {
        let engine = KernelEngine::new(KernelConfig::default());
        let outcome = engine.run(seed_snapshot(), &FaultAdapter, &PanicsIfEvaluated, 3);

        assert_eq!(outcome.attempted_steps, 1);
        assert!(outcome.committed_steps.is_empty());
        match outcome.stop {
            RunStop::AdapterFault {
                preserved_snapshot,
                observation,
                error,
            } => {
                assert_eq!(preserved_snapshot.step(), StepIndex(0));
                assert_eq!(observation.provenance, "fixture");
                assert_eq!(
                    error,
                    AdapterError::Unavailable {
                        detail: "timeout".to_owned(),
                    }
                );
            }
            _ => panic!("expected adapter fault"),
        }
    }

    #[test]
    fn run_stops_on_abort_without_advancing_snapshot() {
        let engine = KernelEngine::new(KernelConfig::default());
        let outcome = engine.run(seed_snapshot(), &RejectingIntentAdapter, &Rejects, 3);

        assert_eq!(outcome.attempted_steps, 1);
        assert!(outcome.committed_steps.is_empty());
        match outcome.stop {
            RunStop::Abort {
                preserved_snapshot,
                observation,
                abort,
            } => {
                assert_eq!(preserved_snapshot.step(), StepIndex(0));
                assert_eq!(observation.provenance, "fixture");
                assert_eq!(abort.reject.step, StepIndex(0));
                assert!(abort.reject.reasons.iter().any(|reason| reason == "bad"));
            }
            _ => panic!("expected abort"),
        }
    }

    #[test]
    fn run_stops_on_step_budget_exhaustion() {
        let engine = KernelEngine::new(KernelConfig::default());
        let outcome = engine.run(seed_snapshot(), &KeepRunningAdapter, &NeverSuccess, 3);

        assert_eq!(outcome.attempted_steps, 3);
        assert_eq!(outcome.committed_steps.len(), 3);
        assert_eq!(outcome.committed_steps[0].record.from_step, StepIndex(0));
        assert_eq!(outcome.committed_steps[2].record.to_step, StepIndex(3));
        assert!(outcome
            .committed_steps
            .iter()
            .all(|step| step.observation.provenance == "fixture"));
        match outcome.stop {
            RunStop::StepBudgetExhausted { last_snapshot } => {
                assert_eq!(last_snapshot.step(), StepIndex(3));
                assert_eq!(last_snapshot.register(), &4_u8);
            }
            _ => panic!("expected budget exhaustion"),
        }
    }

    #[test]
    fn run_preserves_committed_history_before_adapter_fault() {
        let engine = KernelEngine::new(KernelConfig::default());
        let adapter = CommitThenFaultAdapter {
            calls: Cell::new(0),
        };
        let outcome = engine.run(seed_snapshot(), &adapter, &NeverSuccess, 3);

        assert_eq!(outcome.attempted_steps, 2);
        assert_eq!(outcome.committed_steps.len(), 1);
        assert_eq!(outcome.committed_steps[0].record.to_step, StepIndex(1));
        assert_eq!(
            outcome.committed_steps[0].observation.provenance,
            "fixture".to_owned()
        );
        match outcome.stop {
            RunStop::AdapterFault {
                preserved_snapshot,
                error,
                ..
            } => {
                assert_eq!(preserved_snapshot.step(), StepIndex(1));
                assert_eq!(preserved_snapshot.register(), &2_u8);
                assert_eq!(
                    error,
                    AdapterError::Unavailable {
                        detail: "timeout".to_owned(),
                    }
                );
            }
            _ => panic!("expected adapter fault"),
        }
    }

    #[test]
    fn run_preserves_committed_history_before_abort() {
        let engine = KernelEngine::new(KernelConfig::default());
        let adapter = CommitThenRejectAdapter {
            calls: Cell::new(0),
        };
        let outcome = engine.run(seed_snapshot(), &adapter, &RejectOverwriteOnly, 3);

        assert_eq!(outcome.attempted_steps, 2);
        assert_eq!(outcome.committed_steps.len(), 1);
        assert_eq!(outcome.committed_steps[0].record.to_step, StepIndex(1));
        match outcome.stop {
            RunStop::Abort {
                preserved_snapshot,
                abort,
                ..
            } => {
                assert_eq!(preserved_snapshot.step(), StepIndex(1));
                assert_eq!(preserved_snapshot.register(), &2_u8);
                assert_eq!(abort.reject.step, StepIndex(1));
                assert!(abort.reject.reasons.iter().any(|reason| reason == "bad"));
            }
            _ => panic!("expected abort"),
        }
    }

    #[test]
    fn run_stops_on_zero_step_budget_without_calling_adapter() {
        let engine = KernelEngine::new(KernelConfig::default());
        let outcome = engine.run(seed_snapshot(), &PanicsIfProposed, &NeverSuccess, 0);

        assert_eq!(outcome.attempted_steps, 0);
        assert!(outcome.committed_steps.is_empty());
        match outcome.stop {
            RunStop::StepBudgetExhausted { last_snapshot } => {
                assert_eq!(last_snapshot.step(), StepIndex(0));
                assert_eq!(last_snapshot.register(), &1_u8);
            }
            _ => panic!("expected budget exhaustion"),
        }
    }
}
