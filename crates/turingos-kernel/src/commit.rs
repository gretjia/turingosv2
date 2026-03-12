#![cfg_attr(not(test), allow(dead_code))]

use std::fmt::Debug;

use turingos_core::{CommitOutcome, IntentEnvelope, UniverseSnapshot, WriteMode};

use crate::predicate_gate::GatePass;
use crate::trace::build_commit_record;

pub(crate) fn commit_snapshot<QState: Clone + Debug>(
    snapshot: &UniverseSnapshot<QState>,
    intent: IntentEnvelope<QState>,
    _gate_pass: GatePass,
) -> CommitOutcome<QState> {
    let record = build_commit_record(snapshot, &intent);
    let tape = match (intent.write_mode, intent.write_content.as_ref()) {
        (WriteMode::Keep, _) => snapshot.tape().clone(),
        (WriteMode::Overwrite, Some(content)) => snapshot
            .tape()
            .with_write(snapshot.head().path(), content.clone()),
        (WriteMode::Overwrite, None) => {
            panic!("overwrite intent must carry write_content before commit")
        }
    };
    let next = UniverseSnapshot::new(
        intent.proposed_register,
        intent.proposed_head,
        tape,
        record.next_ledger_tip.clone(),
        snapshot.step().next(),
    );
    CommitOutcome::Commit { next, record }
}

#[cfg(test)]
mod tests {
    use turingos_core::{
        Head, IntentEnvelope, StepIndex, TapeState, TraceHash, UniverseSnapshot, WorldState,
        WriteMode,
    };

    use crate::{commit::commit_snapshot, predicate_gate::PredicateGate, task::TaskContract};

    #[test]
    fn commit_path_mints_new_snapshot() {
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
            write_mode: WriteMode::Keep,
            write_content: None,
            halt: false,
            notes: "commit".to_owned(),
        };
        let gate = PredicateGate;
        let pass = gate.evaluate(&snapshot, &intent, &AlwaysPass).unwrap();
        let outcome = commit_snapshot(&snapshot, intent, pass);
        match outcome {
            turingos_core::CommitOutcome::Commit { next, .. } => {
                assert_eq!(next.step(), StepIndex(1));
                assert_eq!(next.register(), &2_u8);
            }
            _ => panic!("expected commit"),
        }
        assert_eq!(snapshot.register(), &1_u8);
    }

    #[test]
    fn overwrite_materializes_tape_change_at_current_head() {
        let snapshot = UniverseSnapshot::new(
            1_u8,
            Head::new("root/file.txt"),
            TapeState::default().with_write("root/file.txt", "old"),
            TraceHash::genesis(),
            StepIndex(0),
        );
        let intent = IntentEnvelope {
            proposed_register: 2_u8,
            action_payload: None,
            proposed_head: Head::new("root/next.txt"),
            write_mode: WriteMode::Overwrite,
            write_content: Some("new".to_owned()),
            halt: false,
            notes: "commit".to_owned(),
        };
        let gate = PredicateGate;
        let pass = gate.evaluate(&snapshot, &intent, &AlwaysPass).unwrap();
        let outcome = commit_snapshot(&snapshot, intent, pass);
        match outcome {
            turingos_core::CommitOutcome::Commit { next, .. } => {
                assert_eq!(next.tape().read("root/file.txt"), Some("new"));
                assert_eq!(next.head().path(), "root/next.txt");
                assert_eq!(snapshot.tape().read("root/file.txt"), Some("old"));
            }
            _ => panic!("expected commit"),
        }
    }

    struct AlwaysPass;

    impl TaskContract<u8> for AlwaysPass {
        fn evaluate(
            &self,
            _world: &WorldState<u8>,
            _intent: &IntentEnvelope<u8>,
        ) -> turingos_core::PredicateVerdict {
            turingos_core::PredicateVerdict::pass()
        }

        fn is_success(&self, _world: &WorldState<u8>) -> bool {
            false
        }
    }
}
