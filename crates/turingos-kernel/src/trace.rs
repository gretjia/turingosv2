#![cfg_attr(not(test), allow(dead_code))]

use std::fmt::Debug;

use turingos_core::{CommitRecord, IntentEnvelope, RejectRecord, TraceHash, UniverseSnapshot};

fn write_mode_label(mode: turingos_core::WriteMode) -> &'static str {
    match mode {
        turingos_core::WriteMode::Keep => "keep",
        turingos_core::WriteMode::Overwrite => "overwrite",
    }
}

fn fnv1a64(input: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in input {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

pub(crate) fn advance_trace(current: &TraceHash, material: impl AsRef<[u8]>) -> TraceHash {
    let mut bytes = current.as_str().as_bytes().to_vec();
    bytes.extend_from_slice(material.as_ref());
    TraceHash::new(format!("{:016x}", fnv1a64(&bytes)))
}

pub(crate) fn build_commit_record<QState: Debug>(
    snapshot: &UniverseSnapshot<QState>,
    intent: &IntentEnvelope<QState>,
) -> CommitRecord {
    let material = format!(
        "commit:{}:{}:{:?}:{}:{}:{}:{}:{}",
        snapshot.step().0,
        intent.proposed_head.path(),
        intent.proposed_register,
        write_mode_label(intent.write_mode),
        intent.write_content.as_deref().unwrap_or(""),
        intent.action_payload.as_deref().unwrap_or(""),
        intent.halt,
        intent.notes,
    );
    let next_tip = advance_trace(snapshot.ledger_tip(), material.into_bytes());
    CommitRecord {
        from_step: snapshot.step(),
        to_step: snapshot.step().next(),
        next_head: intent.proposed_head.clone(),
        next_ledger_tip: next_tip,
        halt_requested: intent.halt,
        notes: intent.notes.clone(),
    }
}

pub(crate) fn build_reject_record<QState>(
    snapshot: &UniverseSnapshot<QState>,
    reasons: Vec<String>,
) -> RejectRecord {
    RejectRecord {
        step: snapshot.step(),
        reasons,
    }
}

#[cfg(test)]
mod tests {
    use turingos_core::{
        Head, IntentEnvelope, StepIndex, TapeState, TraceHash, UniverseSnapshot, WriteMode,
    };

    use super::{build_commit_record, build_reject_record};

    #[test]
    fn reject_record_preserves_objective_timeline() {
        let snapshot = UniverseSnapshot::new(
            1_u8,
            Head::new("root"),
            TapeState::default(),
            TraceHash::genesis(),
            StepIndex(0),
        );
        let intent = IntentEnvelope {
            proposed_register: 2_u8,
            action_payload: Some("noop".to_owned()),
            proposed_head: Head::new("next"),
            write_mode: WriteMode::Keep,
            write_content: None,
            halt: false,
            notes: "ok".to_owned(),
        };
        let reject = build_reject_record(&snapshot, vec!["bad".to_owned()]);
        assert_eq!(reject.step, snapshot.step());
        assert_eq!(snapshot.ledger_tip(), &TraceHash::genesis());
        let commit = build_commit_record(&snapshot, &intent);
        assert_ne!(commit.next_ledger_tip, snapshot.ledger_tip().clone());
    }

    #[test]
    fn semantically_distinct_commits_advance_to_distinct_tips() {
        let snapshot = UniverseSnapshot::new(
            1_u8,
            Head::new("root"),
            TapeState::default(),
            TraceHash::genesis(),
            StepIndex(0),
        );
        let left = IntentEnvelope {
            proposed_register: 2_u8,
            action_payload: None,
            proposed_head: Head::new("next"),
            write_mode: WriteMode::Keep,
            write_content: None,
            halt: false,
            notes: "ok".to_owned(),
        };
        let right = IntentEnvelope {
            proposed_register: 2_u8,
            action_payload: None,
            proposed_head: Head::new("next"),
            write_mode: WriteMode::Overwrite,
            write_content: Some("hello".to_owned()),
            halt: false,
            notes: "ok".to_owned(),
        };
        assert_ne!(
            build_commit_record(&snapshot, &left).next_ledger_tip,
            build_commit_record(&snapshot, &right).next_ledger_tip
        );
    }

    #[test]
    fn identical_commit_material_mints_same_tip() {
        let snapshot = UniverseSnapshot::new(
            1_u8,
            Head::new("root"),
            TapeState::default(),
            TraceHash::genesis(),
            StepIndex(0),
        );
        let left = IntentEnvelope {
            proposed_register: 2_u8,
            action_payload: None,
            proposed_head: Head::new("next"),
            write_mode: WriteMode::Keep,
            write_content: None,
            halt: false,
            notes: "ok".to_owned(),
        };
        let right = IntentEnvelope {
            proposed_register: 2_u8,
            action_payload: None,
            proposed_head: Head::new("next"),
            write_mode: WriteMode::Keep,
            write_content: None,
            halt: false,
            notes: "ok".to_owned(),
        };

        assert_eq!(
            build_commit_record(&snapshot, &left).next_ledger_tip,
            build_commit_record(&snapshot, &right).next_ledger_tip
        );
    }
}
