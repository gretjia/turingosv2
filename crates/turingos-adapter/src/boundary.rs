use turingos_core::{IntentEnvelope, ReadView, StepObservation};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdapterError {
    MalformedOutput { detail: String },
    Unavailable { detail: String },
}

impl std::fmt::Display for AdapterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MalformedOutput { detail } => {
                write!(f, "adapter returned malformed output: {detail}")
            }
            Self::Unavailable { detail } => write!(f, "adapter unavailable: {detail}"),
        }
    }
}

impl std::error::Error for AdapterError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdapterOutcome<QState> {
    Intent {
        observation: StepObservation,
        intent: IntentEnvelope<QState>,
    },
    Fault {
        observation: StepObservation,
        error: AdapterError,
    },
}

pub trait IntentAdapter<QState> {
    fn propose(&self, view: &ReadView<QState>) -> AdapterOutcome<QState>;
}
