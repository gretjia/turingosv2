use std::path::Path;

use turingos_core::{
    Head, IntentEnvelope, PredicateVerdict, ReadView, StepIndex, TapeState, TraceHash,
    UniverseSnapshot, WorldState, WriteMode,
};
use turingos_kernel::TaskContract;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParityPhase {
    Boot,
    InitParity,
    Scan,
    ApplyPending,
    Finalize,
    WriteResult,
    Halt,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PendingBit {
    pub bit: u8,
    pub source: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParityFiles {
    pub parity: String,
    pub result: String,
    pub root_listing: String,
}

impl Default for ParityFiles {
    fn default() -> Self {
        Self {
            parity: "parity.md".to_owned(),
            result: "result.md".to_owned(),
            root_listing: ".ls".to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParityRegister {
    pub phase: ParityPhase,
    pub todo: Vec<String>,
    pub pending: Option<PendingBit>,
    pub answer: Option<String>,
    pub completed: Vec<String>,
    pub files: ParityFiles,
}

impl ParityRegister {
    pub fn initial() -> Self {
        Self {
            phase: ParityPhase::Boot,
            todo: Vec::new(),
            pending: None,
            answer: None,
            completed: Vec::new(),
            files: ParityFiles::default(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ParityTask;

impl ParityTask {
    pub fn initial_world(&self, tape: TapeState) -> WorldState<ParityRegister> {
        WorldState::new(ParityRegister::initial(), Head::new(".ls"), tape)
    }

    pub fn initial_snapshot(&self, tape: TapeState) -> UniverseSnapshot<ParityRegister> {
        UniverseSnapshot::new(
            ParityRegister::initial(),
            Head::new(".ls"),
            tape,
            TraceHash::genesis(),
            StepIndex(0),
        )
    }

    pub fn projected_intent(&self, view: &ReadView<ParityRegister>) -> IntentEnvelope<ParityRegister> {
        let world = WorldState::new(
            view.register.clone(),
            view.head.clone(),
            TapeState::default().with_write(view.head.path(), view.current_content.clone().unwrap_or_default()),
        );
        self.expected_intent(&world)
    }

    pub fn expected_intent(
        &self,
        world: &WorldState<ParityRegister>,
    ) -> IntentEnvelope<ParityRegister> {
        let mut reg = world.register().clone();
        let phase = reg.phase.clone();
        let current_path = world.head().path();
        let current_content = world.tape().read(current_path).unwrap_or("");
        let parity_path = reg.files.parity.clone();
        let result_path = reg.files.result.clone();

        match phase {
            ParityPhase::Boot => {
                reg.phase = ParityPhase::InitParity;
                IntentEnvelope::keep(reg, Head::new(parity_path))
            }
            ParityPhase::InitParity => {
                reg.phase = ParityPhase::Scan;
                reg.todo.clear();
                let root_listing = reg.files.root_listing.clone();
                IntentEnvelope {
                    proposed_register: reg,
                    action_payload: None,
                    proposed_head: Head::new(root_listing),
                    write_mode: WriteMode::Overwrite,
                    write_content: Some("0".to_owned()),
                    halt: false,
                    notes: String::new(),
                }
            }
            ParityPhase::Scan => {
                if current_path.ends_with(".ls") {
                    let (dirs, files) = parse_listing(current_content);
                    let base_dir = dir_of(current_path);
                    let discovered_files = files
                        .into_iter()
                        .filter(|name| name.ends_with(".md"))
                        .map(|name| join(base_dir.as_deref(), &name))
                        .collect::<Vec<_>>();
                    let discovered_dirs = dirs
                        .into_iter()
                        .map(|name| join(Some(&join(base_dir.as_deref(), &name)), ".ls"))
                        .collect::<Vec<_>>();
                    let remaining = reg
                        .todo
                        .iter()
                        .filter(|item| item.as_str() != current_path)
                        .cloned()
                        .collect::<Vec<_>>();
                    if !reg.completed.iter().any(|item| item == current_path) {
                        reg.completed.push(current_path.to_owned());
                    }
                    reg.todo = dedupe_preserve(
                        remaining
                            .into_iter()
                            .chain(discovered_files)
                            .chain(discovered_dirs)
                            .collect(),
                        &reg.completed,
                    );
                    if let Some(next) = reg.todo.first().cloned() {
                        return IntentEnvelope::keep(reg, Head::new(next));
                    }
                    reg.phase = ParityPhase::Finalize;
                    return IntentEnvelope::keep(reg, Head::new(parity_path));
                }

                if current_path.ends_with(".md")
                    && file_name(current_path) != reg.files.parity
                    && file_name(current_path) != reg.files.result
                {
                    let value = current_content.trim().parse::<i64>().unwrap_or(0);
                    let bit = (value & 1) as u8;
                    reg.todo = reg
                        .todo
                        .iter()
                        .filter(|item| item.as_str() != current_path)
                        .cloned()
                        .collect();
                    if !reg.completed.iter().any(|item| item == current_path) {
                        reg.completed.push(current_path.to_owned());
                    }
                    reg.pending = Some(PendingBit {
                        bit,
                        source: current_path.to_owned(),
                    });
                    reg.phase = ParityPhase::ApplyPending;
                    return IntentEnvelope::keep(reg, Head::new(parity_path));
                }

                if let Some(next) = reg.todo.first().cloned() {
                    return IntentEnvelope::keep(reg, Head::new(next));
                }
                reg.phase = ParityPhase::Finalize;
                IntentEnvelope::keep(reg, Head::new(parity_path))
            }
            ParityPhase::ApplyPending => {
                let current = current_content.trim().parse::<u8>().unwrap_or(0);
                let bit = reg.pending.as_ref().map(|pending| pending.bit).unwrap_or(0);
                let new_value = (current ^ bit).to_string();
                reg.pending = None;
                let next_head = if let Some(next) = reg.todo.first().cloned() {
                    reg.phase = ParityPhase::Scan;
                    next
                } else {
                    reg.phase = ParityPhase::Finalize;
                    parity_path
                };
                IntentEnvelope {
                    proposed_register: reg,
                    action_payload: None,
                    proposed_head: Head::new(next_head),
                    write_mode: WriteMode::Overwrite,
                    write_content: Some(new_value),
                    halt: false,
                    notes: String::new(),
                }
            }
            ParityPhase::Finalize => {
                if current_path != parity_path {
                    return IntentEnvelope::keep(reg, Head::new(parity_path));
                }
                let answer = if current_content.trim().parse::<u8>().unwrap_or(0) == 0 {
                    "even"
                } else {
                    "odd"
                };
                reg.answer = Some(answer.to_owned());
                reg.phase = ParityPhase::WriteResult;
                IntentEnvelope::keep(reg, Head::new(result_path))
            }
            ParityPhase::WriteResult => {
                if current_path != result_path {
                    return IntentEnvelope::keep(reg, Head::new(result_path));
                }
                let answer = reg.answer.clone().unwrap_or_else(|| self.expected_answer(world.tape()));
                reg.phase = ParityPhase::Halt;
                IntentEnvelope {
                    proposed_register: reg,
                    action_payload: None,
                    proposed_head: Head::new(result_path),
                    write_mode: WriteMode::Overwrite,
                    write_content: Some(answer),
                    halt: true,
                    notes: String::new(),
                }
            }
            ParityPhase::Halt => IntentEnvelope {
                proposed_register: reg,
                action_payload: None,
                proposed_head: Head::new(current_path.to_owned()),
                write_mode: WriteMode::Keep,
                write_content: None,
                halt: true,
                notes: String::new(),
            },
        }
    }

    pub fn expected_answer(&self, tape: &TapeState) -> String {
        let mut parity = 0_u8;
        let parity_name = ParityFiles::default().parity;
        let result_name = ParityFiles::default().result;
        for (path, content) in tape.entries() {
            if !path.ends_with(".md") {
                continue;
            }
            let name = file_name(path);
            if name == parity_name || name == result_name {
                continue;
            }
            if let Ok(value) = content.trim().parse::<i64>() {
                parity ^= (value & 1) as u8;
            }
        }
        if parity == 0 {
            "even".to_owned()
        } else {
            "odd".to_owned()
        }
    }
}

impl TaskContract<ParityRegister> for ParityTask {
    fn evaluate(
        &self,
        world: &WorldState<ParityRegister>,
        intent: &IntentEnvelope<ParityRegister>,
    ) -> PredicateVerdict {
        let expected = self.expected_intent(world);
        let mut reasons = Vec::new();

        if intent.proposed_head != expected.proposed_head {
            reasons.push(format!(
                "wrong_path: expected next_path={}, got {}",
                expected.proposed_head.path(),
                intent.proposed_head.path()
            ));
        }
        if intent.write_mode != expected.write_mode {
            reasons.push(format!(
                "wrong_write_mode: expected {:?}, got {:?}",
                expected.write_mode, intent.write_mode
            ));
        }
        if intent.write_content != expected.write_content {
            reasons.push(format!(
                "wrong_write_content: expected {:?}, got {:?}",
                expected.write_content, intent.write_content
            ));
        }
        if intent.halt != expected.halt {
            reasons.push(format!(
                "wrong_halt: expected {}, got {}",
                expected.halt, intent.halt
            ));
        }
        if intent.proposed_register != expected.proposed_register {
            reasons.push("wrong_register: proposed_register diverged from expected policy".to_owned());
        }

        if reasons.is_empty() {
            PredicateVerdict::pass()
        } else {
            PredicateVerdict::fail(reasons)
        }
    }

    fn is_success(&self, world: &WorldState<ParityRegister>) -> bool {
        matches!(world.register().phase, ParityPhase::Halt)
            && world.tape().read(&world.register().files.result)
                == Some(self.expected_answer(world.tape()).as_str())
    }
}

fn parse_listing(content: &str) -> (Vec<String>, Vec<String>) {
    let mut dirs = Vec::new();
    let mut files = Vec::new();
    for raw in content.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(name) = line.strip_prefix("DIR ") {
            dirs.push(name.to_owned());
        } else if let Some(name) = line.strip_prefix("FILE ") {
            files.push(name.to_owned());
        }
    }
    (dirs, files)
}

fn dir_of(path: &str) -> Option<String> {
    let parent = Path::new(path).parent()?.to_string_lossy().replace('\\', "/");
    if parent.is_empty() || parent == "." {
        None
    } else {
        Some(parent)
    }
}

fn join(base: Option<&str>, name: &str) -> String {
    match base {
        Some("") | None => name.to_owned(),
        Some(base) => format!("{base}/{name}"),
    }
}

fn file_name(path: &str) -> String {
    Path::new(path)
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default()
}

fn dedupe_preserve(items: Vec<String>, completed: &[String]) -> Vec<String> {
    let mut output = Vec::new();
    for item in items {
        if completed.iter().any(|done| done == &item) {
            continue;
        }
        if output.iter().any(|seen| seen == &item) {
            continue;
        }
        output.push(item);
    }
    output
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use turingos_adapter::{AdapterOutcome, IntentAdapter};
    use turingos_core::WorldState;
    use turingos_core::{ReadView, StepObservation};
    use turingos_kernel::{HaltStatus, KernelConfig, KernelEngine, RunStop};

    use super::*;

    fn sample_tape() -> TapeState {
        TapeState::new(BTreeMap::from([
            (".ls".to_owned(), "DIR dir\nFILE a.md".to_owned()),
            ("dir/.ls".to_owned(), "FILE b.md".to_owned()),
            ("a.md".to_owned(), "3".to_owned()),
            ("dir/b.md".to_owned(), "4".to_owned()),
            ("parity.md".to_owned(), "".to_owned()),
            ("result.md".to_owned(), "".to_owned()),
        ]))
    }

    fn base_register() -> ParityRegister {
        ParityRegister::initial()
    }

    struct DeterministicParityAdapter {
        task: ParityTask,
    }

    impl IntentAdapter<ParityRegister> for DeterministicParityAdapter {
        fn propose(&self, view: &ReadView<ParityRegister>) -> AdapterOutcome<ParityRegister> {
            AdapterOutcome::Intent {
                observation: StepObservation {
                    provenance: "deterministic-parity".to_owned(),
                },
                intent: self.task.projected_intent(view),
            }
        }
    }

    #[test]
    fn root_listing_scan_prefers_files_before_child_listings() {
        let task = ParityTask;
        let mut register = base_register();
        register.phase = ParityPhase::Scan;
        let world = WorldState::new(register, Head::new(".ls"), sample_tape());

        let intent = task.expected_intent(&world);

        assert_eq!(intent.proposed_head.path(), "a.md");
        assert_eq!(
            intent.proposed_register.todo,
            vec!["a.md".to_owned(), "dir/.ls".to_owned()]
        );
        assert_eq!(intent.proposed_register.completed, vec![".ls".to_owned()]);
        assert_eq!(intent.write_mode, WriteMode::Keep);
    }

    #[test]
    fn apply_pending_xors_parity_and_returns_to_next_todo() {
        let task = ParityTask;
        let mut register = base_register();
        register.phase = ParityPhase::ApplyPending;
        register.todo = vec!["dir/.ls".to_owned()];
        register.pending = Some(PendingBit {
            bit: 1,
            source: "a.md".to_owned(),
        });
        let tape = sample_tape().with_write("parity.md", "1");
        let world = WorldState::new(register, Head::new("parity.md"), tape);

        let intent = task.expected_intent(&world);

        assert_eq!(intent.proposed_head.path(), "dir/.ls");
        assert_eq!(intent.write_mode, WriteMode::Overwrite);
        assert_eq!(intent.write_content.as_deref(), Some("0"));
        assert_eq!(intent.proposed_register.phase, ParityPhase::Scan);
        assert_eq!(intent.proposed_register.pending, None);
    }

    #[test]
    fn finalize_and_write_result_halt_with_expected_answer() {
        let task = ParityTask;
        let mut register = base_register();
        register.phase = ParityPhase::Finalize;
        let tape = sample_tape().with_write("parity.md", "1");
        let world = WorldState::new(register, Head::new("parity.md"), tape);

        let finalize = task.expected_intent(&world);
        assert_eq!(finalize.proposed_head.path(), "result.md");
        assert_eq!(finalize.proposed_register.phase, ParityPhase::WriteResult);
        assert_eq!(finalize.proposed_register.answer.as_deref(), Some("odd"));

        let write_world = WorldState::new(
            finalize.proposed_register.clone(),
            Head::new("result.md"),
            sample_tape().with_write("parity.md", "1"),
        );
        let write = task.expected_intent(&write_world);
        assert_eq!(write.write_mode, WriteMode::Overwrite);
        assert_eq!(write.write_content.as_deref(), Some("odd"));
        assert!(write.halt);
        assert_eq!(write.proposed_register.phase, ParityPhase::Halt);
    }

    #[test]
    fn evaluate_rejects_wrong_path() {
        let task = ParityTask;
        let mut register = base_register();
        register.phase = ParityPhase::Scan;
        let world = WorldState::new(register, Head::new(".ls"), sample_tape());
        let mut wrong = task.expected_intent(&world);
        wrong.proposed_head = Head::new("parity.md");

        let verdict = task.evaluate(&world, &wrong);
        assert!(!verdict.passed());
        assert!(verdict.reasons().iter().any(|reason| reason.starts_with("wrong_path:")));
    }

    #[test]
    fn task_success_requires_halt_and_expected_result() {
        let task = ParityTask;
        let mut register = base_register();
        register.phase = ParityPhase::Halt;
        let world = WorldState::new(
            register.clone(),
            Head::new("result.md"),
            sample_tape().with_write("result.md", "odd"),
        );
        assert!(task.is_success(&world));

        let wrong = WorldState::new(
            register,
            Head::new("result.md"),
            sample_tape().with_write("result.md", "even"),
        );
        assert!(!task.is_success(&wrong));
    }

    #[test]
    fn live_rust_parity_lane_halts_successfully() {
        let task = ParityTask;
        let adapter = DeterministicParityAdapter { task: task.clone() };
        let engine = KernelEngine::new(KernelConfig::default());
        let initial = task.initial_snapshot(sample_tape());

        let outcome = engine.run(initial, &adapter, &task, 32);

        match outcome.stop {
            RunStop::Halted {
                final_snapshot,
                status,
            } => {
                assert_eq!(status, HaltStatus::Success);
                assert_eq!(
                    final_snapshot.tape().read("result.md"),
                    Some("odd")
                );
                assert!(task.is_success(final_snapshot.world()));
            }
            other => panic!("expected halted success, got {other:?}"),
        }
    }
}
