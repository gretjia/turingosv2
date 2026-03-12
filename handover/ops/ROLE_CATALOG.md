# TuringOSv2 Role Catalog

Status: Active
Scope: Role registry, trigger rules, packet format, and ownership guidance for repository-level multi-agent work

## 1. Role Registry

| Role | Stable id | Engine | Write authority | Primary use |
| --- | --- | --- | --- | --- |
| Commander | `commander` | primary Codex session | Yes | scope lock, integration, handover, git decisions |
| Plan Agent | `turingos_plan` | Codex child role | No | bounded change map, risks, acceptance gates |
| Coder Agent | `turingos_coder` | Codex child role | Yes, assigned files only | general implementation outside Groundhog-specialized Rust work |
| Rust Systems Architect | `turingos_rust_systems_architect` | Codex child role | No | Rust crate decomposition, state types, commit-engine structure |
| Rust Coder | `turingos_rust_coder` | Codex child role | Yes, assigned files only | Groundhog Rust implementation |
| OS Systems Planner | `turingos_os_system_planner` | Codex child role | No | Unix, OS, CLI, config, filesystem, capability review |
| Git Historian | `turingos_git_historian` | Codex child role | No | provenance, commit slicing, sealed-file handling, migration sequencing |
| Karpathy Editor | `turingos_karpathy_editor` | Codex child role | Only if explicitly assigned | simplification and code-aesthetic review |
| Quality Auditor | `turingos_quality_auditor` | Codex child role | No | regression, tests, rollback, CLI contract |
| Research Agent | `turingos_researcher` | Codex child role | No | official-docs and primary-source research |
| Runtime Watcher | `turingos_runtime_watcher` | Codex child role | No | long-running test, demo, or benchmark monitoring |
| Goal Poller | `turingos_goal_poller` | Codex child role | No | target-rung supervision, progress polling, and stop/advance decisions for benchmark loops |
| Turing-AGI Architect Auditor | `turing_agi_architect_auditor` | Gemini CLI | No | top-level AGI-direction audit from `bible/`; for Groundhog, mission policy pins it to `Gemini 3.1 Pro Preview` |
| Groundhog Math Constitution Custodian | `groundhog_math_constitution_custodian` | Gemini CLI pinned to `Gemini 3.1 Pro Preview` | No | theorem-preservation audit for the sealed constitution |
| Groundhog Formal Methods Auditor | `groundhog_formal_methods_auditor` | Gemini CLI pinned to `Gemini 3.1 Pro Preview` | No | invariant and proof-obligation audit |
| Groundhog Turing Machine Theorist | `groundhog_turing_machine_theorist` | Gemini CLI pinned to `Gemini 3.1 Pro Preview` | No | tape/head/register and discrete-machine semantics |
| Groundhog Recursive Math Auditor | `groundhog_recursive_math_auditor` | Gemini CLI pinned to `Gemini 3.1 Pro Preview` | No | end-of-stage recursive mathematical closure audit |
| Recursive Software Auditor | `turingos_recursive_software_auditor` | Codex child role | No | end-of-stage recursive software-engineering closure audit |

`Architecture Auditor` remains retired. General architecture oversight is split between the Turing-AGI Architect Auditor and the OS Systems Planner. Mission Groundhog adds math-specialist Gemini roles plus Rust/git/Karpathy execution specialists.

## 2. Trigger Map

- Commander: every mission, no exception.
- Plan Agent: use when scope spans multiple files, multiple phases, or multiple child roles.
- Coder Agent: use for non-Rust implementation tasks once scope and ownership are locked.
- Rust Systems Architect: required for Groundhog Rust crate design, state-model decomposition, or commit-engine structure.
- Rust Coder: use for approved Groundhog Rust implementation once architecture and ownership are locked.
- OS Systems Planner: required for changes touching CLI, tooling, filesystem semantics, config, package layout, capability model, runtime boundary, recovery behavior, or Unix operational shape.
- Git Historian: add when commit slicing, provenance protection, sealed-file handling, or migration sequencing matter.
- Karpathy Editor: add after correctness is established when code simplification, naming, or abstraction pressure needs a dedicated pass.
- Turing-AGI Architect Auditor: required for changes touching `bible/`, `README.md`, `turingos/runtime.py`, `turingos/agents/`, `turingos/scheduler.py`, `turingos/signals.py`, `turingos/verifiers.py`, `turingos/oracle.py`, or any mission framed as architecture, memory, agent interface, verifier, or AGI-direction work.
- Groundhog Math Constitution Custodian: required for any Groundhog change touching the sealed constitution, state model, commit/abort semantics, or black-box/white-box privilege boundary.
- Groundhog Formal Methods Auditor: required for Groundhog proof, invariant, or transition-safety questions.
- Groundhog Turing Machine Theorist: required when theorem language must be mapped to tape/head/register semantics.
- Groundhog Recursive Math Auditor: required at Groundhog stage close.
- Recursive Software Auditor: required at Groundhog stage close.
- Quality Auditor: required after any non-trivial implementation, CLI behavior change, or shared-library change.
- Research Agent: add when behavior depends on unstable external APIs, official docs, or model/tooling facts.
- Runtime Watcher: add only after a long-running process is launched.
- Goal Poller: add when a benchmark or validation loop has an explicit quantitative target or gate ladder and the Commander needs agent-level polling without introducing a resident watchdog process; its verdict basis must be a compressed summary of historical success/failure evidence, not expert preference alone.

## 3. Default Routing

- Small task: Commander -> Coder -> Quality Auditor
- Medium task: Commander -> Plan -> one or more Coder children -> OS Systems Planner -> Quality Auditor
- Architecture task: Commander -> Plan -> Turing-AGI Architect Auditor + OS Systems Planner -> Coder children -> Turing-AGI Architect Auditor + OS Systems Planner + Quality Auditor
- Mission Groundhog constitution-sensitive task: Commander -> Plan -> Groundhog Math Constitution Custodian + Groundhog Turing Machine Theorist + OS Systems Planner -> Groundhog Formal Methods Auditor -> Turing-AGI Architect Auditor
- Mission Groundhog Rust architecture task: Commander -> Plan -> Groundhog Math Constitution Custodian + Groundhog Formal Methods Auditor + Rust Systems Architect + OS Systems Planner + Git Historian
- Mission Groundhog implementation task: Commander -> Plan -> Rust Systems Architect -> one or more Rust Coder children -> Karpathy Editor -> Quality Auditor -> Groundhog Recursive Math Auditor + Recursive Software Auditor
- External-fact-sensitive task: add Research Agent before implementation
- Long-running validation: add Runtime Watcher after launch
- Target-driven benchmark loop: Commander -> Goal Poller -> Runtime Watcher while live runs are active -> Quality Auditor before any gate advance that changes code or run policy

## 4. Common Packet Fields

Every delegated role packet should include:

1. mission name or mission id
2. role id and engine
3. bounded objective
4. canonical spec
5. in-scope paths or evidence, including relevant historical success/failure corpus when the task is a debug, promotion, or rerun decision
6. out-of-scope paths or forbidden actions
7. allowed actions
8. required output format
9. test, runtime, or evidence constraints
10. stop conditions

## 5. Common Output Envelope

- Proven facts
- Inferences
- Open risks
- Evidence basis when the role is recommending a debug, promotion, rerun, or advance decision

Additional role-specific requirements:

- Plan Agent: in-scope files, out-of-scope files, acceptance gates, risks, open assumptions
- Coder and Rust Coder: exact files changed
- Rust Systems Architect: crate map, API boundaries, and migration risks
- Git Historian: provenance findings and commit-slicing guidance
- Karpathy Editor: simplification findings and naming/API cleanup recommendations
- Turing-AGI Architect Auditor: `AGI verdict` plus drift findings
- Groundhog Math Constitution Custodian: constitution-preservation verdict plus theorem-preservation findings
- Groundhog Formal Methods Auditor: formal-consistency verdict plus invariant findings
- Groundhog Turing Machine Theorist: machine-semantics map plus ambiguity warnings
- Groundhog Recursive Math Auditor: recursive math verdict plus missed contradiction or drift findings
- Recursive Software Auditor: recursive software verdict plus unresolved cross-audit tensions
- OS Systems Planner: `OS verdict` plus boundary and Linux risk findings
- Quality Auditor: `PASS`, `PASS WITH FIXES`, or `BLOCK`
- Research Agent: sources consulted
- Runtime Watcher: current health, latest progress, ETA if possible
- Goal Poller: current rung, current ceiling, compressed success/failure basis, advance/hold/stop verdict, next recommended action, and anti-hardcoding risk check

## 6. Recommended Ownership Slices

Use these file-group slices when parallel delegation is needed.

| Slice | Typical files |
| --- | --- |
| Constitution and formal canon | `bible/`, `handover/ops/MISSION_GROUNDHOG_*`, related proof notes |
| Python reference kernel | `turingos/runtime.py`, `turingos/models.py`, `turingos/fs.py`, `turingos/ledger.py` |
| White-box control layer | `turingos/signals.py`, `turingos/broadcast.py`, `turingos/masking.py`, `turingos/scheduler.py`, `turingos/verifiers.py`, `turingos/oracle.py` |
| Worker and task surface | `turingos/agents/`, `turingos/tasks/`, `examples/` |
| Rust embodiment | `Cargo.toml`, `rust/`, `crates/`, `src/`, or future Groundhog Rust roots |
| OS, CLI, and tooling | `turingos/cli.py`, `turingos/tools/`, `pyproject.toml` |
| Validation | `tests/`, replay fixtures, benchmark harnesses |
| Governance and handover | `AGENTS.md`, `handover/`, `.codex/` |

Default rule:

- Commander owns `handover/` and git decisions unless the mission explicitly assigns documentation-only work to a child.

## 7. Spec Files

- `handover/ops/roles/COMMANDER.md`
- `handover/ops/roles/PLAN_AGENT.md`
- `handover/ops/roles/CODER_AGENT.md`
- `handover/ops/roles/RUST_SYSTEMS_ARCHITECT.md`
- `handover/ops/roles/RUST_CODER.md`
- `handover/ops/roles/OS_SYSTEMS_PLANNER.md`
- `handover/ops/roles/GIT_HISTORIAN.md`
- `handover/ops/roles/KARPATHY_EDITOR.md`
- `handover/ops/roles/TURING_AGI_ARCHITECT_AUDITOR.md`
- `handover/ops/roles/GROUNDHOG_MATH_CONSTITUTION_CUSTODIAN.md`
- `handover/ops/roles/GROUNDHOG_FORMAL_METHODS_AUDITOR.md`
- `handover/ops/roles/GROUNDHOG_TURING_MACHINE_THEORIST.md`
- `handover/ops/roles/GROUNDHOG_RECURSIVE_MATH_AUDITOR.md`
- `handover/ops/roles/RECURSIVE_SOFTWARE_AUDITOR.md`
- `handover/ops/roles/QUALITY_AUDITOR.md`
- `handover/ops/roles/RESEARCH_AGENT.md`
- `handover/ops/roles/RUNTIME_WATCHER.md`
- `handover/ops/roles/GOAL_POLLER.md`
