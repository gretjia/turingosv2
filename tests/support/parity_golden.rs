use std::collections::BTreeMap;

use serde::Deserialize;
use turingos_adapter::{AdapterOutcome, IntentAdapter};
use turingos_core::{
    Head, IntentEnvelope, PredicateVerdict, ReadView, StepIndex, StepObservation, TapeState,
    TraceHash, UniverseSnapshot, WorldState, WriteMode,
};
use turingos_kernel::TaskContract;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GoldenRegister {
    pub cursor: usize,
    pub phase: String,
    pub answer: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GoldenFixture {
    #[allow(dead_code)]
    pub source_python_task: String,
    #[allow(dead_code)]
    pub source_python_runtime: String,
    #[allow(dead_code)]
    pub source_python_test: String,
    #[allow(dead_code)]
    pub capture_basis: String,
    pub selected_agent: String,
    pub initial_head: String,
    pub initial_phase: String,
    pub initial_answer: Option<String>,
    pub initial_tape: BTreeMap<String, String>,
    pub expected_steps: usize,
    pub expected_result: String,
    pub history: Vec<GoldenStep>,
}

#[derive(Debug, Deserialize)]
pub struct GoldenStep {
    pub step: usize,
    pub current_phase: String,
    pub current_answer: Option<String>,
    pub current_path: String,
    pub current_content: String,
    pub next_path: String,
    pub write_mode: String,
    pub write_content: Option<String>,
    pub halt: bool,
    pub next_phase: String,
    pub next_answer: Option<String>,
}

impl GoldenStep {
    pub fn next_register(&self) -> GoldenRegister {
        GoldenRegister {
            cursor: self.step + 1,
            phase: self.next_phase.clone(),
            answer: self.next_answer.clone(),
        }
    }

    pub fn intent(&self) -> IntentEnvelope<GoldenRegister> {
        IntentEnvelope {
            proposed_register: self.next_register(),
            action_payload: None,
            proposed_head: Head::new(self.next_path.clone()),
            write_mode: match self.write_mode.as_str() {
                "keep" => WriteMode::Keep,
                "overwrite" => WriteMode::Overwrite,
                other => panic!("unknown write mode: {other}"),
            },
            write_content: self.write_content.clone(),
            halt: self.halt,
            notes: String::new(),
        }
    }
}

pub struct GoldenTask<'a> {
    pub fixture: &'a GoldenFixture,
}

pub struct GoldenAdapter<'a> {
    fixture: &'a GoldenFixture,
}

impl<'a> GoldenAdapter<'a> {
    pub fn new(fixture: &'a GoldenFixture) -> Self {
        Self { fixture }
    }
}

impl TaskContract<GoldenRegister> for GoldenTask<'_> {
    fn evaluate(
        &self,
        world: &WorldState<GoldenRegister>,
        intent: &IntentEnvelope<GoldenRegister>,
    ) -> PredicateVerdict {
        let step = world.register().cursor;
        let Some(expected) = self.fixture.history.get(step) else {
            return PredicateVerdict::fail(vec![format!("unexpected cursor {step}")]);
        };

        let mut reasons = Vec::new();
        if world.register().cursor != expected.step {
            reasons.push(format!(
                "cursor mismatch: expected {}, got {}",
                expected.step,
                world.register().cursor
            ));
        }
        if world.register().phase != expected.current_phase {
            reasons.push(format!(
                "phase mismatch: expected {}, got {}",
                expected.current_phase,
                world.register().phase
            ));
        }
        if world.register().answer != expected.current_answer {
            reasons.push("answer mismatch".to_owned());
        }
        if world.head().path() != expected.current_path {
            reasons.push(format!(
                "head mismatch: expected {}, got {}",
                expected.current_path,
                world.head().path()
            ));
        }
        if world.tape().read(world.head().path()) != Some(expected.current_content.as_str()) {
            reasons.push("current_content mismatch".to_owned());
        }
        if intent != &expected.intent() {
            reasons.push(format!("intent mismatch at step {}", expected.step));
        }

        if reasons.is_empty() {
            PredicateVerdict::pass()
        } else {
            PredicateVerdict::fail(reasons)
        }
    }

    fn is_success(&self, world: &WorldState<GoldenRegister>) -> bool {
        world.register().cursor == self.fixture.history.len()
            && world.register().phase == "halt"
            && world.tape().read("result.md") == Some(self.fixture.expected_result.as_str())
    }
}

impl IntentAdapter<GoldenRegister> for GoldenAdapter<'_> {
    fn propose(&self, view: &ReadView<GoldenRegister>) -> AdapterOutcome<GoldenRegister> {
        let step = view.register.cursor;
        let expected = self
            .fixture
            .history
            .get(step)
            .unwrap_or_else(|| panic!("no golden step for cursor {step}"));

        assert_eq!(view.register.phase, expected.current_phase);
        assert_eq!(view.register.answer, expected.current_answer);
        assert_eq!(view.head.path(), expected.current_path);
        assert_eq!(
            view.current_content.as_deref(),
            Some(expected.current_content.as_str())
        );

        AdapterOutcome::Intent {
            observation: StepObservation {
                provenance: format!("golden:{}", self.fixture.selected_agent),
            },
            intent: expected.intent(),
        }
    }
}

pub fn fixture() -> GoldenFixture {
    serde_json::from_str(include_str!(
        "../../crates/turingos-kernel/tests/fixtures/parity_deterministic.json"
    ))
    .expect("phase 20 frozen parity fixture must deserialize")
}

#[allow(dead_code)]
pub fn assert_phase20_provenance(fixture: &GoldenFixture) {
    assert_eq!(fixture.source_python_task, "turingos/tasks/parity.py");
    assert_eq!(fixture.source_python_runtime, "turingos/runtime.py");
    assert_eq!(fixture.source_python_test, "tests/test_parity_runtime.py");
    assert_eq!(
        fixture.capture_basis,
        "deterministic alpha path via TuringOSConfig(max_steps=64, explore_epsilon=0.0, random_seed=11) and DeterministicPolicyAgent(agent_id=\"alpha\", policy=task.expected_transition(...))"
    );
}

pub fn initial_snapshot(fixture: &GoldenFixture) -> UniverseSnapshot<GoldenRegister> {
    UniverseSnapshot::new(
        GoldenRegister {
            cursor: 0,
            phase: fixture.initial_phase.clone(),
            answer: fixture.initial_answer.clone(),
        },
        Head::new(fixture.initial_head.clone()),
        TapeState::new(fixture.initial_tape.clone()),
        TraceHash::genesis(),
        StepIndex(0),
    )
}
