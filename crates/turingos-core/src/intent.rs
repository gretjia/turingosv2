use crate::Head;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteMode {
    Keep,
    Overwrite,
}

/// WHITEBOX projected input:
/// `INPUT_t[WHITEBOX] := <q_t, ReadTool_WHITEBOX(tape_t, HEAD_t)>`.
///
/// This is the only theorem-bearing read surface a BLACKBOX adapter may see.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReadView<QState> {
    pub register: QState,
    pub head: Head,
    pub current_content: Option<String>,
}

/// BLACKBOX suspended intent package:
/// `OUTPUT_t[BLACKBOX] := <q_o, a_o>`.
///
/// This is not future world state. It is only a proposal emitted by
/// `Delta_BLACKBOX(INPUT_t[WHITEBOX])`.
/// The proposal remains outside the worldline until `Predicates_WHITEBOX`
/// admit it and `WriteTool_WHITEBOX` materializes the next snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntentEnvelope<QState> {
    pub proposed_register: QState,
    pub action_payload: Option<String>,
    pub proposed_head: Head,
    pub write_mode: WriteMode,
    pub write_content: Option<String>,
    pub halt: bool,
    pub notes: String,
}

impl<QState> IntentEnvelope<QState> {
    pub fn keep(proposed_register: QState, proposed_head: Head) -> Self {
        Self {
            proposed_register,
            action_payload: None,
            proposed_head,
            write_mode: WriteMode::Keep,
            write_content: None,
            halt: false,
            notes: String::new(),
        }
    }
}
