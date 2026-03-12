use crate::{Head, IntentEnvelope, StepIndex, TraceHash, UniverseSnapshot};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommitRecord {
    pub from_step: StepIndex,
    pub to_step: StepIndex,
    pub next_head: Head,
    pub next_ledger_tip: TraceHash,
    pub halt_requested: bool,
    pub notes: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RejectRecord {
    pub step: StepIndex,
    pub reasons: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AbortRecord<QState> {
    pub reject: RejectRecord,
    pub rejected_intent: IntentEnvelope<QState>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommitOutcome<QState> {
    Commit {
        next: UniverseSnapshot<QState>,
        record: CommitRecord,
    },
    Abort {
        abort: AbortRecord<QState>,
    },
}
