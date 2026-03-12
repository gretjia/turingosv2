use turingos_core::{IntentEnvelope, PredicateVerdict, WorldState};

/// WHITEBOX admissibility contract.
///
/// A task contract does not generate proposals. It judges whether a BLACKBOX
/// proposal may be admitted across the WHITEBOX predicate barrier, and whether
/// a committed world satisfies success.
pub trait TaskContract<QState> {
    fn evaluate(
        &self,
        world: &WorldState<QState>,
        intent: &IntentEnvelope<QState>,
    ) -> PredicateVerdict;

    fn is_success(&self, world: &WorldState<QState>) -> bool;
}
