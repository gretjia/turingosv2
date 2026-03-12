#![cfg_attr(not(test), allow(dead_code))]

use std::fmt::Debug;

use turingos_adapter::{AdapterOutcome, IntentAdapter};
use turingos_core::{CommitOutcome, IntentEnvelope, UniverseSnapshot};

use crate::{
    commit::commit_snapshot, config::KernelConfig, driver::StepDriverOutcome,
    predicate_gate::PredicateGate, read::project_read_view, task::TaskContract,
};

#[derive(Clone, Debug)]
pub struct KernelEngine {
    config: KernelConfig,
    gate: PredicateGate,
}

impl KernelEngine {
    pub fn new(config: KernelConfig) -> Self {
        Self {
            config,
            gate: PredicateGate,
        }
    }

    pub fn config(&self) -> &KernelConfig {
        &self.config
    }

    pub fn step<QState: Clone + Debug>(
        &self,
        snapshot: &UniverseSnapshot<QState>,
        adapter: &dyn IntentAdapter<QState>,
        contract: &dyn TaskContract<QState>,
    ) -> StepDriverOutcome<QState> {
        let view = project_read_view(snapshot);
        match adapter.propose(&view) {
            AdapterOutcome::Intent {
                observation,
                intent,
            } => StepDriverOutcome::Applied {
                observation,
                outcome: self.apply(snapshot, intent, contract),
            },
            AdapterOutcome::Fault { observation, error } => StepDriverOutcome::AdapterFault {
                preserved_step: snapshot.step(),
                observation,
                error,
            },
        }
    }

    pub(crate) fn apply<QState: Clone + Debug>(
        &self,
        snapshot: &UniverseSnapshot<QState>,
        intent: IntentEnvelope<QState>,
        contract: &dyn TaskContract<QState>,
    ) -> CommitOutcome<QState> {
        match self.gate.evaluate(snapshot, &intent, contract) {
            Ok(pass) => commit_snapshot(snapshot, intent, pass),
            Err(reject) => CommitOutcome::Abort {
                abort: turingos_core::AbortRecord {
                    reject,
                    rejected_intent: intent,
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use turingos_adapter::{AdapterError, AdapterOutcome, IntentAdapter};
    use turingos_core::{
        CommitOutcome, Head, IntentEnvelope, PredicateVerdict, ReadView, StepIndex,
        StepObservation, TapeState, TraceHash, UniverseSnapshot, WorldState, WriteMode,
    };

    use crate::{
        config::KernelConfig, driver::StepDriverOutcome, engine::KernelEngine, task::TaskContract,
    };

    struct Rejects;
    struct AlwaysPass;
    struct PanicsIfEvaluated;
    struct CommitAdapter;
    struct FaultAdapter;
    struct MalformedAdapter;

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

    impl TaskContract<u8> for AlwaysPass {
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

    impl IntentAdapter<u8> for CommitAdapter {
        fn propose(&self, view: &ReadView<u8>) -> AdapterOutcome<u8> {
            AdapterOutcome::Intent {
                observation: StepObservation {
                    provenance: "fixture".to_owned(),
                },
                intent: IntentEnvelope {
                    proposed_register: view.register + 1,
                    action_payload: None,
                    proposed_head: Head::new("root/next.txt"),
                    write_mode: WriteMode::Overwrite,
                    write_content: Some(view.current_content.as_deref().unwrap_or("").to_owned()),
                    halt: false,
                    notes: "commit".to_owned(),
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

    impl IntentAdapter<u8> for MalformedAdapter {
        fn propose(&self, _view: &ReadView<u8>) -> AdapterOutcome<u8> {
            AdapterOutcome::Fault {
                observation: StepObservation {
                    provenance: "fixture".to_owned(),
                },
                error: AdapterError::MalformedOutput {
                    detail: "bad json".to_owned(),
                },
            }
        }
    }

    #[test]
    fn abort_path_preserves_step() {
        let engine = KernelEngine::new(KernelConfig::default());
        let snapshot = UniverseSnapshot::new(
            1_u8,
            Head::new("root"),
            TapeState::default(),
            TraceHash::genesis(),
            StepIndex(0),
        );
        let intent = IntentEnvelope {
            proposed_register: 2_u8,
            action_payload: None,
            proposed_head: Head::new("next"),
            write_mode: WriteMode::Overwrite,
            write_content: Some("hello".to_owned()),
            halt: false,
            notes: "reject".to_owned(),
        };
        let outcome = engine.apply(&snapshot, intent, &Rejects);
        match outcome {
            CommitOutcome::Abort { abort } => {
                assert_eq!(abort.reject.step, StepIndex(0));
                assert_eq!(abort.rejected_intent.notes, "reject");
            }
            _ => panic!("expected abort"),
        }
        assert_eq!(snapshot.ledger_tip(), &TraceHash::genesis());
    }

    #[test]
    fn step_projects_read_view_into_commit_cycle() {
        let engine = KernelEngine::new(KernelConfig::default());
        let snapshot = UniverseSnapshot::new(
            1_u8,
            Head::new("root/current.txt"),
            TapeState::default().with_write("root/current.txt", "old"),
            TraceHash::genesis(),
            StepIndex(0),
        );

        let outcome = engine.step(&snapshot, &CommitAdapter, &AlwaysPass);

        match outcome {
            StepDriverOutcome::Applied {
                observation,
                outcome,
            } => match outcome {
                CommitOutcome::Commit { next, .. } => {
                    assert_eq!(observation.provenance, "fixture");
                    assert_eq!(next.register(), &2_u8);
                    assert_eq!(next.head().path(), "root/next.txt");
                    assert_eq!(next.tape().read("root/current.txt"), Some("old"));
                    assert_eq!(snapshot.tape().read("root/current.txt"), Some("old"));
                }
                _ => panic!("expected committed step"),
            },
            _ => panic!("expected committed step"),
        }
    }

    #[test]
    fn step_returns_adapter_fault_without_entering_commit_path() {
        let engine = KernelEngine::new(KernelConfig::default());
        let snapshot = UniverseSnapshot::new(
            1_u8,
            Head::new("root/current.txt"),
            TapeState::default(),
            TraceHash::genesis(),
            StepIndex(0),
        );
        let outcome = engine.step(&snapshot, &FaultAdapter, &PanicsIfEvaluated);

        match outcome {
            StepDriverOutcome::AdapterFault {
                preserved_step,
                observation,
                error,
            } => {
                assert_eq!(preserved_step, StepIndex(0));
                assert_eq!(
                    observation,
                    StepObservation {
                        provenance: "fixture".to_owned(),
                    }
                );
                assert_eq!(
                    error,
                    AdapterError::Unavailable {
                        detail: "timeout".to_owned(),
                    }
                );
            }
            _ => panic!("expected adapter fault"),
        }
        assert_eq!(snapshot.step(), StepIndex(0));
        assert_eq!(snapshot.ledger_tip(), &TraceHash::genesis());
    }

    #[test]
    fn step_returns_malformed_output_fault_without_kernel_outcome() {
        let engine = KernelEngine::new(KernelConfig::default());
        let snapshot = UniverseSnapshot::new(
            1_u8,
            Head::new("root/current.txt"),
            TapeState::default(),
            TraceHash::genesis(),
            StepIndex(0),
        );
        let outcome = engine.step(&snapshot, &MalformedAdapter, &PanicsIfEvaluated);

        match outcome {
            StepDriverOutcome::AdapterFault {
                preserved_step,
                observation,
                error,
            } => {
                assert_eq!(preserved_step, StepIndex(0));
                assert_eq!(
                    observation,
                    StepObservation {
                        provenance: "fixture".to_owned(),
                    }
                );
                assert_eq!(
                    error,
                    AdapterError::MalformedOutput {
                        detail: "bad json".to_owned(),
                    }
                );
            }
            _ => panic!("expected malformed adapter fault"),
        }
        assert_eq!(snapshot.step(), StepIndex(0));
        assert_eq!(snapshot.ledger_tip(), &TraceHash::genesis());
    }

    #[test]
    fn step_can_abort_after_adapter_success() {
        let engine = KernelEngine::new(KernelConfig::default());
        let snapshot = UniverseSnapshot::new(
            1_u8,
            Head::new("root/current.txt"),
            TapeState::default().with_write("root/current.txt", "old"),
            TraceHash::genesis(),
            StepIndex(0),
        );

        let outcome = engine.step(&snapshot, &CommitAdapter, &Rejects);

        match outcome {
            StepDriverOutcome::Applied {
                observation,
                outcome,
            } => match outcome {
                CommitOutcome::Abort { abort } => {
                    assert_eq!(observation.provenance, "fixture");
                    assert_eq!(abort.reject.step, StepIndex(0));
                    assert!(abort.reject.reasons.iter().any(|reason| reason == "bad"));
                    assert_eq!(abort.rejected_intent.notes, "commit");
                }
                _ => panic!("expected abort"),
            },
            _ => panic!("expected abort"),
        }
        assert_eq!(snapshot.tape().read("root/current.txt"), Some("old"));
    }

    #[test]
    fn observation_provenance_stays_outside_commit_authority() {
        struct CommitAdapterWithProvenance(&'static str);

        impl IntentAdapter<u8> for CommitAdapterWithProvenance {
            fn propose(&self, view: &ReadView<u8>) -> AdapterOutcome<u8> {
                AdapterOutcome::Intent {
                    observation: StepObservation {
                        provenance: self.0.to_owned(),
                    },
                    intent: IntentEnvelope {
                        proposed_register: view.register + 1,
                        action_payload: None,
                        proposed_head: Head::new("root/next.txt"),
                        write_mode: WriteMode::Overwrite,
                        write_content: Some("same".to_owned()),
                        halt: false,
                        notes: "commit".to_owned(),
                    },
                }
            }
        }

        let engine = KernelEngine::new(KernelConfig::default());
        let snapshot = UniverseSnapshot::new(
            1_u8,
            Head::new("root/current.txt"),
            TapeState::default().with_write("root/current.txt", "old"),
            TraceHash::genesis(),
            StepIndex(0),
        );

        let left = engine.step(
            &snapshot,
            &CommitAdapterWithProvenance("model-a"),
            &AlwaysPass,
        );
        let right = engine.step(
            &snapshot,
            &CommitAdapterWithProvenance("model-b"),
            &AlwaysPass,
        );

        match (left, right) {
            (
                StepDriverOutcome::Applied {
                    observation: left_observation,
                    outcome:
                        CommitOutcome::Commit {
                            record: left_record,
                            ..
                        },
                },
                StepDriverOutcome::Applied {
                    observation: right_observation,
                    outcome:
                        CommitOutcome::Commit {
                            record: right_record,
                            ..
                        },
                },
            ) => {
                assert_eq!(left_record.next_ledger_tip, right_record.next_ledger_tip);
                assert_ne!(left_observation.provenance, right_observation.provenance);
            }
            _ => panic!("expected committed steps"),
        }
    }
}
