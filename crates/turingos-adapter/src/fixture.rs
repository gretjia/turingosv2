use turingos_core::ReadView;

use crate::{AdapterOutcome, IntentAdapter};

pub(crate) struct FixtureAdapter<F> {
    plan: F,
}

impl<F> FixtureAdapter<F> {
    pub(crate) fn new(plan: F) -> Self {
        Self { plan }
    }
}

impl<QState, F> IntentAdapter<QState> for FixtureAdapter<F>
where
    F: Fn(&ReadView<QState>) -> AdapterOutcome<QState>,
{
    fn propose(&self, view: &ReadView<QState>) -> AdapterOutcome<QState> {
        (self.plan)(view)
    }
}

#[cfg(test)]
mod tests {
    use turingos_core::{Head, IntentEnvelope, ReadView, StepObservation, WriteMode};

    use crate::{AdapterError, AdapterOutcome, IntentAdapter};

    use super::FixtureAdapter;

    fn propose_through_boundary(
        adapter: &dyn IntentAdapter<u8>,
        view: &ReadView<u8>,
    ) -> AdapterOutcome<u8> {
        adapter.propose(view)
    }

    #[test]
    fn fixture_adapter_builds_intent_from_read_view_only() {
        let adapter = FixtureAdapter::new(|view: &ReadView<u8>| AdapterOutcome::Intent {
            observation: StepObservation {
                provenance: "fixture".to_owned(),
            },
            intent: IntentEnvelope {
                proposed_register: view.register + 1,
                action_payload: view.current_content.clone(),
                proposed_head: Head::new(format!("{}:next", view.head.path())),
                write_mode: WriteMode::Overwrite,
                write_content: view.current_content.clone(),
                halt: false,
                notes: "fixture".to_owned(),
            },
        });
        let view = ReadView {
            register: 7_u8,
            head: Head::new("root/input"),
            current_content: Some("payload".to_owned()),
        };

        match adapter.propose(&view) {
            AdapterOutcome::Intent {
                observation,
                intent,
            } => {
                assert_eq!(intent.proposed_register, 8_u8);
                assert_eq!(intent.action_payload.as_deref(), Some("payload"));
                assert_eq!(intent.proposed_head.path(), "root/input:next");
                assert_eq!(intent.write_content.as_deref(), Some("payload"));
                assert_eq!(intent.notes, "fixture");
                assert_eq!(observation.provenance, "fixture");
            }
            _ => panic!("expected intent"),
        }
    }

    #[test]
    fn fixture_adapter_remains_usable_through_provider_neutral_trait_object() {
        let adapter: Box<dyn IntentAdapter<u8>> =
            Box::new(FixtureAdapter::new(|view: &ReadView<u8>| {
                AdapterOutcome::Intent {
                    observation: StepObservation::default(),
                    intent: IntentEnvelope::keep(view.register, view.head.clone()),
                }
            }));
        let view = ReadView {
            register: 3_u8,
            head: Head::new("root/current"),
            current_content: None,
        };

        match propose_through_boundary(adapter.as_ref(), &view) {
            AdapterOutcome::Intent {
                observation,
                intent,
            } => {
                assert_eq!(intent.proposed_register, 3_u8);
                assert_eq!(intent.proposed_head.path(), "root/current");
                assert_eq!(intent.write_mode, WriteMode::Keep);
                assert_eq!(intent.action_payload, None);
                assert_eq!(intent.write_content, None);
                assert_eq!(observation, StepObservation::default());
            }
            _ => panic!("expected intent"),
        }
    }

    #[test]
    fn fixture_adapter_can_surface_provider_neutral_error_without_faking_intent() {
        let adapter: Box<dyn IntentAdapter<u8>> =
            Box::new(FixtureAdapter::new(|_view: &ReadView<u8>| {
                AdapterOutcome::Fault {
                    observation: StepObservation {
                        provenance: "fixture".to_owned(),
                    },
                    error: AdapterError::Unavailable {
                        detail: "timeout".to_owned(),
                    },
                }
            }));
        let view = ReadView {
            register: 3_u8,
            head: Head::new("root/current"),
            current_content: None,
        };

        match propose_through_boundary(adapter.as_ref(), &view) {
            AdapterOutcome::Fault { observation, error } => {
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
            _ => panic!("expected fault"),
        }
    }
}
