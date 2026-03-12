# Mission Groundhog Phase 8 Spec Draft

Status: Completed and frozen after Phase 8 PASS
Mission: Mission Groundhog
Phase: 8
Date: 2026-03-10 UTC

## 0. Phase State

- Phase 8 is now the active bounded stage.
- Phase 7 is closed and frozen by `handover/ops/MISSION_GROUNDHOG_PHASE_007_CLOSEOUT.md`.
- Phase 8 may not claim completion until it has:
  - opened only the bounded multi-step run-loop surface below
  - passed the required audit ring
  - written `handover/ops/MISSION_GROUNDHOG_PHASE_008_CLOSEOUT.md`
  - written one append-only entry matching `handover/ai-direct/entries/*_groundhog_phase008_closeout.md`

## 1. Objective

- Open the first bounded multi-step run loop above the Phase 7 single-step kernel seam.
- Reuse the existing theorem-safe `KernelEngine::step(...)` path rather than duplicating step logic.
- Define explicit run-stop classes without opening scheduler, provider transport, CLI, or watchdog logic.

## 2. Canonical Authority

- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
- `handover/ops/ACTIVE_MISSION_CHARTER.md`
- `handover/ops/MISSION_GROUNDHOG_PROJECT_SPEC.md`
- `handover/ops/MISSION_GROUNDHOG_EXECUTION_LOOP.md`
- `handover/ops/MISSION_GROUNDHOG_IMPLEMENTATION_GUARDRAILS.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_001_ARCHITECTURE_PACKET.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_005_CLOSEOUT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_006_CLOSEOUT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_007_CLOSEOUT.md`

## 3. In Scope Files

Writable in this phase:

- `Cargo.toml`
- `Cargo.lock`
- `crates/turingos-kernel/src/lib.rs`
- `crates/turingos-kernel/src/driver.rs`
- `crates/turingos-kernel/src/engine.rs`
- `crates/turingos-kernel/src/predicate_gate.rs`
- `crates/turingos-kernel/src/run.rs`
- `handover/ops/MISSION_GROUNDHOG_PHASE_008_SPEC_DRAFT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_008_CLOSEOUT.md`
- one new append-only entry matching `handover/ai-direct/entries/*_groundhog_phase008_closeout.md`
- `handover/ai-direct/LATEST.md`
- `handover/BOARD.md`

Read-only context:

- `crates/turingos-core/`
- `crates/turingos-adapter/`
- `crates/turingos-task/`
- `tests/`
- `benchmarks/`
- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`

## 4. Out of Scope Files

- `turingos/`
- any real model-provider integration
- any network client or server integration
- any scheduler module
- any CLI crate or CLI implementation
- any benchmark migration code
- any constitution file

## 5. Surface Touch Matrix

- kernel: yes
- pluggable adapters: no
- tools: no
- CLI: no
- validation: yes

## 6. Kernel/Plugin Boundary Implications

- Phase 8 may sequence repeated calls to `KernelEngine::step(...)`, but it may not duplicate or fork theorem authority.
- The phase may:
  - add one bounded run-loop surface
  - define explicit run-stop outcomes
  - carry observation metadata forward only as reporting-side evidence
- The phase must not:
  - add retry policy
  - add scheduler policy
  - add provider orchestration
  - add background watchdog or supervisor logic

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

- one bounded multi-step run-loop surface above `KernelEngine::step(...)`
- one explicit run outcome shape covering:
  - halted success
  - adapter fault
  - bounded step-budget exhaustion
- tests proving:
  - run loop reuses step semantics rather than reimplementing them
  - adapter faults stop the run without entering new policy layers
  - step-budget exhaustion is explicit and auditable
- one Phase 8 closeout packet

## 9. Acceptance Criteria

- the run loop is bounded and white-box auditable
- single-step theorem authority remains centralized in existing step/apply/gate code
- no scheduler or retry policy is introduced
- no watchdog or supervisor process is introduced
- the resulting API remains compact and legible

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
- runtime observation must continue to be handled by polling agents rather than background daemon logic
- no real provider or network integration
- no CLI embodiment
- no scheduler embodiment
- no benchmark migration embodiment
- no constitution edits

## 13. Open Risks

- A run loop can accidentally smuggle in scheduler policy if stop classes are not sharply defined.
- Repeating single-step execution can tempt duplication of theorem logic unless the phase keeps `KernelEngine::step(...)` as the sole entrypoint.
- Budget exhaustion can become a shadow retry policy if the stop semantics are underspecified.
