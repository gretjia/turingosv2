# Mission Groundhog Phase 3 Spec Draft

Status: Completed and frozen with PASS
Mission: Mission Groundhog
Phase: 3
Date: 2026-03-09 UTC

## 0. Phase State

- Phase 3 is now completed and frozen.
- Phase 2 is closed and frozen by `handover/ops/MISSION_GROUNDHOG_PHASE_002_CLOSEOUT.md`.
- Phase 3 may not claim completion until it has:
  - opened only the bounded task/replay surface below
  - passed the required audit ring
  - written `handover/ops/MISSION_GROUNDHOG_PHASE_003_CLOSEOUT.md`
  - written one append-only entry matching `handover/ai-direct/entries/*_groundhog_phase003_closeout.md`

## 1. Objective

- Open the first white-box task-policy crate for Groundhog.
- Move the temporary kernel-private workload seam out of the kernel and into a bounded `turingos-task` crate.
- Add replay-oriented task/trace fixture surfaces that can feed later validation without opening adapters, CLI, scheduler, or benchmark migration.

## 2. Canonical Authority

- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
- `handover/ops/ACTIVE_MISSION_CHARTER.md`
- `handover/ops/MISSION_GROUNDHOG_PROJECT_SPEC.md`
- `handover/ops/MISSION_GROUNDHOG_EXECUTION_LOOP.md`
- `handover/ops/MISSION_GROUNDHOG_IMPLEMENTATION_GUARDRAILS.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_001_ARCHITECTURE_PACKET.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_002_SPEC_DRAFT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_002_CLOSEOUT.md`

## 3. In Scope Files

Writable in this phase:

- `Cargo.toml`
- `Cargo.lock`
- `crates/turingos-task/Cargo.toml`
- `crates/turingos-task/src/lib.rs`
- `crates/turingos-task/src/contract.rs`
- `crates/turingos-task/src/replay.rs`
- `crates/turingos-kernel/src/engine.rs`
- `crates/turingos-kernel/src/lib.rs`
- `handover/ops/MISSION_GROUNDHOG_PHASE_003_SPEC_DRAFT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_003_CLOSEOUT.md`
- one new append-only entry matching `handover/ai-direct/entries/*_groundhog_phase003_closeout.md`
- `handover/ai-direct/LATEST.md`
- `handover/BOARD.md`

Read-only context:

- `crates/turingos-core/`
- `crates/turingos-kernel/src/read.rs`
- `crates/turingos-kernel/src/predicate_gate.rs`
- `crates/turingos-kernel/src/commit.rs`
- `tests/`
- `benchmarks/`
- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`

## 4. Out of Scope Files

- `turingos/`
- `benchmarks/`
- `tests/` outside inline or module-local Rust tests introduced in the newly opened files
- any adapter crate or adapter home
- any CLI crate or CLI implementation
- any scheduler module
- any real model-provider integration
- any benchmark migration code
- any constitution file

## 5. Surface Touch Matrix

- kernel: yes
- pluggable adapters: no
- tools: no
- CLI: no
- validation: yes

## 6. Kernel/Plugin Boundary Implications

- Phase 3 may open only white-box task policy, not black-box adapters.
- Task policy must stay side-effect-free:
  - task-local read expectations
  - task-local predicate evaluation
  - task-local success/replay semantics
- `ReadView -> IntentEnvelope` remains the only eventual plugin seam.
- Phase 3 must not freeze any adapter transport, prompt format, or provider model boundary.

## 7. Required Roles

Execution lanes:

- `turingos_plan`
- `groundhog_math_constitution_custodian`
- `groundhog_formal_methods_auditor`
- `groundhog_turing_machine_theorist`
- `turingos_rust_systems_architect`
- `turingos_rust_coder`
- `turingos_git_historian`

Phase-close auditors:

- `turingos_quality_auditor`
- `turing_agi_architect_auditor`

Phase-close recursive auditors:

- `groundhog_recursive_math_auditor`
- `turingos_recursive_software_auditor`

Advisory simplification lane:

- `turingos_karpathy_editor`

## 8. Deliverables

- `turingos-task` crate scaffold
- one explicit public white-box task contract
- one replay-oriented fixture surface for task/trace expectations
- kernel rewired to consume the task contract directly instead of the temporary kernel-private seam
- one Phase 3 closeout packet

## 9. Acceptance Criteria

- `crates/turingos-task/` exists and is the new home of the white-box workload contract
- the kernel no longer defines or aliases the authoritative task contract
- the opened task contract remains side-effect-free and white-box
- replay-oriented fixture surfaces exist without opening adapter or CLI concerns
- Phase 3 still keeps adapters, CLI, scheduler, and benchmark migration closed

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

- `cargo fmt --check`
- `cargo test`
- `./.venv/bin/python -m pytest -q`

## 12. Constraints

- no watchdog or supervisor daemon in any language
- no adapter or model-provider integration
- no CLI embodiment
- no scheduler embodiment
- no benchmark migration embodiment
- no constitution edits

## 13. Open Risks

- Opening the task crate can accidentally freeze task policy too early if replay semantics are mixed with adapter concerns
- the kernel/task split must not reintroduce a second route around the predicate gate
- replay fixtures must remain evidentiary rather than growing into a benchmark harness by stealth
