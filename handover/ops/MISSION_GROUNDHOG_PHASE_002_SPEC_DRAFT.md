# Mission Groundhog Phase 2 Spec Draft

Status: Completed and frozen with PASS
Mission: Mission Groundhog
Phase: 2
Date: 2026-03-09 UTC

## 0. Phase State

- Phase 2 is now completed and frozen.
- Rust toolchain preflight was satisfied on 2026-03-09 UTC:
  - `cargo 1.94.0`
  - `rustc 1.94.0`
- The minimal Cargo/core/kernel file slice is now opened and compiles locally.
- Phase 2 closed only after it:
  - embodied the approved Rust roots
  - passed the required audit ring
  - written `handover/ops/MISSION_GROUNDHOG_PHASE_002_CLOSEOUT.md`
  - written one append-only entry matching `handover/ai-direct/entries/*_groundhog_phase002_closeout.md`

## 1. Objective

- Open the minimal Rust embodiment slice approved by Phase 1.
- Embody the canonical white-box core types and the kernel skeleton for read, predicate, commit, abort, and trace.
- Keep adapters, CLI, scheduler, workload ports, and benchmark migration closed.

## 2. Canonical Authority

- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
- `handover/ops/ACTIVE_MISSION_CHARTER.md`
- `handover/ops/MISSION_GROUNDHOG_PROJECT_SPEC.md`
- `handover/ops/MISSION_GROUNDHOG_EXECUTION_LOOP.md`
- `handover/ops/MISSION_GROUNDHOG_IMPLEMENTATION_GUARDRAILS.md`
- `handover/ops/MISSION_GROUNDHOG_TEAM.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_001_SPEC_DRAFT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_001_ARCHITECTURE_PACKET.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_001_CLOSEOUT.md`

## 3. In Scope Files

Writable in this phase:

- `Cargo.toml`
- `Cargo.lock`
- `crates/turingos-core/Cargo.toml`
- `crates/turingos-core/src/lib.rs`
- `crates/turingos-core/src/ids.rs`
- `crates/turingos-core/src/snapshot.rs`
- `crates/turingos-core/src/intent.rs`
- `crates/turingos-core/src/predicate.rs`
- `crates/turingos-core/src/outcome.rs`
- `crates/turingos-core/src/error.rs`
- `crates/turingos-kernel/Cargo.toml`
- `crates/turingos-kernel/src/lib.rs`
- `crates/turingos-kernel/src/config.rs`
- `crates/turingos-kernel/src/contracts.rs`
- `crates/turingos-kernel/src/read.rs`
- `crates/turingos-kernel/src/predicate_gate.rs`
- `crates/turingos-kernel/src/commit.rs`
- `crates/turingos-kernel/src/engine.rs`
- `crates/turingos-kernel/src/trace.rs`
- `handover/ops/MISSION_GROUNDHOG_PHASE_002_SPEC_DRAFT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_002_CLOSEOUT.md`
- one new append-only entry matching `handover/ai-direct/entries/*_groundhog_phase002_closeout.md`
- `handover/ai-direct/LATEST.md`
- `handover/BOARD.md`

Generated-metadata exception:

- `Cargo.lock` is permitted as generated workspace metadata for the opened Rust scaffold.
- This is the only Phase 2 exception to the exact Phase 1 source-file opening slice.
- No extra Rust source files are opened by that exception.

Read-only context:

- `turingos/`
- `tests/`
- `benchmarks/`
- `handover/ops/MISSION_GROUNDHOG_PHASE_001_ARCHITECTURE_PACKET.md`
- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`

## 4. Out of Scope Files

- `turingos/`
- `tests/` outside any inline or module-local Rust tests introduced within the in-scope Rust files
- `benchmarks/`
- `crates/turingos-task/`
- any adapter crate or adapter home
- any CLI crate or CLI implementation
- any scheduler module
- any benchmark migration code
- any constitution file

## 5. Surface Touch Matrix

- kernel: yes
- pluggable adapters: no
- tools: no
- CLI: no
- validation: yes

## 6. Kernel/Plugin Boundary Implications

- Phase 2 may embody only the white-box side of the architecture.
- `ReadView -> IntentEnvelope` remains the only future plugin seam, but no plugin crate opens in this phase.
- Phase 2 must not add hidden repair logic or a public constructor for the gate-pass capability.
- Phase 2 must not widen the public API to freeze the adapter seam too early.

## 7. Required Roles

Execution lanes:

- `turingos_plan`
- `groundhog_math_constitution_custodian`
- `groundhog_formal_methods_auditor`
- `groundhog_turing_machine_theorist`
- `turingos_rust_systems_architect`
- `turingos_rust_coder`
- `turingos_git_historian`

Phase-close recursive auditors:

- `groundhog_recursive_math_auditor`
- `turingos_recursive_software_auditor`

Phase-close auditors:

- `turingos_quality_auditor`
- `turing_agi_architect_auditor`

## 8. Deliverables

- minimal Cargo workspace scaffold
- `turingos-core` types for:
  - `UniverseSnapshot<QState>`
  - `ReadView`
  - `IntentEnvelope`
  - predicate verdict types
  - `CommitOutcome`
  - `CommitRecord`
  - `RejectRecord`
- `turingos-kernel` skeleton modules for:
  - `config`
  - `contracts`
  - `read`
  - `predicate_gate`
  - `commit`
  - `engine`
  - `trace`
- one Phase 2 closeout packet

## 9. Acceptance Criteria

- the opened Rust source file set matches the Phase 1 approved opening slice, with only the recorded `Cargo.lock` metadata exception
- `UniverseSnapshot` is embodied with a workload-generic `QState`
- `ReadView` and `IntentEnvelope` are embodied without direct future-state authority
- gate-pass capability cannot be forged outside the kernel
- commit and abort outcomes are represented explicitly
- abort path remains zero-mutation in the type and ownership plan
- the phase does not open adapters, CLI, scheduler, or benchmark migration

## 10. Required Audits

- constitution audit
- formal methods audit
- Turing-machine semantics audit
- Rust systems architecture audit
- git/provenance audit
- quality audit
- recursive audit pair
- AGI direction audit

## 11. Required Tests And Validation

- preflight:
  - `cargo --version`
  - `rustc --version`
- formatting and compile/test gates once toolchain exists:
  - `cargo fmt --check`
  - `cargo test`
- repo guard:
  - `./.venv/bin/python -m pytest -q`

## 12. Constraints

- no watchdog or supervisor daemon in any language
- no adapter or model-provider integration
- no CLI embodiment
- no scheduler embodiment
- no workload-port embodiment
- no constitution edits

## 13. Open Risks

- Rust root naming must remain sharp enough to avoid reopening architecture questions
- later phases may still need a separate task crate or adapter home, but Phase 2 must not pre-freeze that wider boundary
- Phase 2 has local tests passing, but it has not yet been through the full Groundhog audit ring
