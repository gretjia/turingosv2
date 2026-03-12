use std::collections::BTreeMap;

use crate::{StepIndex, TraceHash};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Head {
    path: String,
}

impl Head {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TapeState {
    entries: BTreeMap<String, String>,
}

impl TapeState {
    pub fn new(entries: BTreeMap<String, String>) -> Self {
        Self { entries }
    }

    pub fn entries(&self) -> &BTreeMap<String, String> {
        &self.entries
    }

    pub fn read(&self, path: &str) -> Option<&str> {
        self.entries.get(path).map(String::as_str)
    }

    pub fn with_write(&self, path: impl Into<String>, content: impl Into<String>) -> Self {
        let mut entries = self.entries.clone();
        entries.insert(path.into(), content.into());
        Self { entries }
    }
}

/// Constitution-bearing world triple.
///
/// This is the WHITEBOX object that corresponds exactly to
/// `WORLD_t[WHITEBOX] := <q_t, HEAD_t, tape_t>`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldState<QState> {
    register: QState,
    head: Head,
    tape: TapeState,
}

impl<QState> WorldState<QState> {
    pub fn new(register: QState, head: Head, tape: TapeState) -> Self {
        Self {
            register,
            head,
            tape,
        }
    }

    pub fn register(&self) -> &QState {
        &self.register
    }

    pub fn head(&self) -> &Head {
        &self.head
    }

    pub fn tape(&self) -> &TapeState {
        &self.tape
    }
}

/// Whitebox witness metadata that tracks lineage outside the pure world triple.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnapshotWitness {
    ledger_tip: TraceHash,
    step: StepIndex,
}

impl SnapshotWitness {
    pub fn new(ledger_tip: TraceHash, step: StepIndex) -> Self {
        Self { ledger_tip, step }
    }

    pub fn ledger_tip(&self) -> &TraceHash {
        &self.ledger_tip
    }

    pub fn step(&self) -> StepIndex {
        self.step
    }
}

/// Current committed universe snapshot.
///
/// Constitutionally, the theorem-bearing world state is the WHITEBOX triple
/// `<q_t, HEAD_t, tape_t>`. This wrapper carries that pure world triple plus
/// witness metadata used by the implementation (`ledger_tip`, `step`).
/// Witness metadata does not widen BLACKBOX authority or become part of the
/// world triple itself.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UniverseSnapshot<QState> {
    world: WorldState<QState>,
    witness: SnapshotWitness,
}

impl<QState> UniverseSnapshot<QState> {
    pub fn new(
        register: QState,
        head: Head,
        tape: TapeState,
        ledger_tip: TraceHash,
        step: StepIndex,
    ) -> Self {
        Self::from_world(
            WorldState::new(register, head, tape),
            SnapshotWitness::new(ledger_tip, step),
        )
    }

    pub fn from_world(world: WorldState<QState>, witness: SnapshotWitness) -> Self {
        Self { world, witness }
    }

    pub fn world(&self) -> &WorldState<QState> {
        &self.world
    }

    pub fn witness(&self) -> &SnapshotWitness {
        &self.witness
    }

    pub fn register(&self) -> &QState {
        self.world.register()
    }

    pub fn head(&self) -> &Head {
        self.world.head()
    }

    pub fn tape(&self) -> &TapeState {
        self.world.tape()
    }

    pub fn ledger_tip(&self) -> &TraceHash {
        self.witness.ledger_tip()
    }

    pub fn step(&self) -> StepIndex {
        self.witness.step()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_can_advance_without_mutating_head() {
        let head = Head::new("root/.ls");
        let snapshot = UniverseSnapshot::new(
            1_u8,
            head.clone(),
            TapeState::default(),
            TraceHash::genesis(),
            StepIndex(0),
        );
        let next = UniverseSnapshot::new(
            2_u8,
            head.clone(),
            TapeState::default(),
            TraceHash::new("next"),
            snapshot.step().next(),
        );
        assert_eq!(snapshot.head(), &head);
        assert_eq!(next.step(), StepIndex(1));
    }

    #[test]
    fn tape_state_projects_written_copy_without_mutating_prior_state() {
        let tape = TapeState::default().with_write("root/.ls", "DIR a");
        let next = tape.with_write("root/.ls", "DIR b");
        assert_eq!(tape.read("root/.ls"), Some("DIR a"));
        assert_eq!(next.read("root/.ls"), Some("DIR b"));
    }

    #[test]
    fn world_state_is_a_pure_triple() {
        let world = WorldState::new(3_u8, Head::new("root/.ls"), TapeState::default());
        assert_eq!(world.register(), &3_u8);
        assert_eq!(world.head().path(), "root/.ls");
        assert_eq!(world.tape().read("root/.ls"), None);
    }

    #[test]
    fn universe_snapshot_wraps_world_plus_witness_metadata() {
        let world = WorldState::new(3_u8, Head::new("root/.ls"), TapeState::default());
        let snapshot = UniverseSnapshot::from_world(
            world.clone(),
            SnapshotWitness::new(TraceHash::new("tip"), StepIndex(4)),
        );
        assert_eq!(snapshot.world(), &world);
        assert_eq!(snapshot.step(), StepIndex(4));
    }
}
