use turingos_adapter::AdapterError;
use turingos_core::{CommitOutcome, StepIndex, StepObservation};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StepDriverOutcome<QState> {
    Applied {
        observation: StepObservation,
        outcome: CommitOutcome<QState>,
    },
    AdapterFault {
        preserved_step: StepIndex,
        observation: StepObservation,
        error: AdapterError,
    },
}
