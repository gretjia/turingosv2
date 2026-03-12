#![cfg_attr(not(test), allow(dead_code))]

use turingos_core::{IntentEnvelope, RejectRecord, UniverseSnapshot};

use crate::{task::TaskContract, trace::build_reject_record};

#[derive(Clone, Debug, Default)]
pub struct PredicateGate;

#[derive(Clone, Copy, Debug)]
pub(crate) struct GatePass(());

impl PredicateGate {
    fn kernel_reasons<QState>(
        _snapshot: &UniverseSnapshot<QState>,
        intent: &IntentEnvelope<QState>,
    ) -> Vec<String> {
        let mut reasons = Vec::new();
        if matches!(intent.write_mode, turingos_core::WriteMode::Overwrite)
            && intent.write_content.is_none()
        {
            reasons.push("overwrite intent must carry write_content".to_owned());
        }
        reasons
    }

    pub(crate) fn evaluate<QState>(
        &self,
        snapshot: &UniverseSnapshot<QState>,
        intent: &IntentEnvelope<QState>,
        contract: &dyn TaskContract<QState>,
    ) -> Result<GatePass, RejectRecord> {
        let mut reasons = Self::kernel_reasons(snapshot, intent);
        reasons.extend_from_slice(contract.evaluate(snapshot.world(), intent).reasons());
        if reasons.is_empty() {
            return Ok(GatePass(()));
        }
        Err(build_reject_record(snapshot, reasons))
    }
}

#[cfg(test)]
mod tests {
    use turingos_core::{
        Head, IntentEnvelope, PredicateVerdict, StepIndex, TapeState, TraceHash, UniverseSnapshot,
        WorldState, WriteMode,
    };

    use crate::task::TaskContract;

    use super::PredicateGate;

    struct Rejects;

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

    #[test]
    fn gate_returns_reject_record_on_failure() {
        let gate = PredicateGate;
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
            notes: String::new(),
        };
        let reject = gate.evaluate(&snapshot, &intent, &Rejects);
        assert!(reject.is_err());
    }

    #[test]
    fn gate_rejects_overwrite_without_content_before_contract() {
        let gate = PredicateGate;
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
            write_content: None,
            halt: false,
            notes: String::new(),
        };
        let reject = gate.evaluate(&snapshot, &intent, &Rejects).unwrap_err();
        assert!(reject
            .reasons
            .iter()
            .any(|reason| reason == "overwrite intent must carry write_content"));
    }
}
