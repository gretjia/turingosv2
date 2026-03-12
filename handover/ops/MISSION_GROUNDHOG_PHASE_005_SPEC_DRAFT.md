# Mission Groundhog Phase 5 Spec Draft

Status: Completed and frozen with PASS
Mission: Mission Groundhog
Phase: 5
Date: 2026-03-09 UTC

## 0. Phase State

- Phase 5 was the bounded active stage for this cycle and is now closed and frozen.
- Phase 4 is closed and frozen by `handover/ops/MISSION_GROUNDHOG_PHASE_004_CLOSEOUT.md`.
- Phase 5 may not claim completion until it has:
  - opened only the bounded theorem-cycle surface below
  - passed the required audit ring
  - written `handover/ops/MISSION_GROUNDHOG_PHASE_005_CLOSEOUT.md`
  - written one append-only entry matching `handover/ai-direct/entries/*_groundhog_phase005_closeout.md`

## 1. Objective

- Embody the first theorem-bearing single-step driver path for Groundhog.
- Connect read projection, adapter proposal, predicate gate, and commit/abort into one bounded white-box cycle.
- Prove the cycle with fixture adapters only, while keeping all real provider integration, CLI, scheduler, and benchmark integration closed.

## 2. Canonical Authority

- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
- `handover/ops/ACTIVE_MISSION_CHARTER.md`
- `handover/ops/MISSION_GROUNDHOG_PROJECT_SPEC.md`
- `handover/ops/MISSION_GROUNDHOG_EXECUTION_LOOP.md`
- `handover/ops/MISSION_GROUNDHOG_IMPLEMENTATION_GUARDRAILS.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_001_ARCHITECTURE_PACKET.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_002_CLOSEOUT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_003_CLOSEOUT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_004_SPEC_DRAFT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_004_CLOSEOUT.md`

## 3. In Scope Files

Writable in this phase:

- `Cargo.toml`
- `Cargo.lock`
- `crates/turingos-core/src/outcome.rs`
- `crates/turingos-kernel/Cargo.toml`
- `crates/turingos-kernel/src/lib.rs`
- `crates/turingos-kernel/src/engine.rs`
- `crates/turingos-kernel/src/read.rs`
- `crates/turingos-kernel/src/driver.rs`
- `crates/turingos-kernel/src/trace.rs`
- `crates/turingos-adapter/src/lib.rs`
- `crates/turingos-adapter/src/boundary.rs`
- `crates/turingos-adapter/src/fixture.rs`
- `handover/ops/MISSION_GROUNDHOG_PHASE_005_SPEC_DRAFT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_005_CLOSEOUT.md`
- one new append-only entry matching `handover/ai-direct/entries/*_groundhog_phase005_closeout.md`
- `handover/ai-direct/LATEST.md`
- `handover/BOARD.md`

Read-only context:

- `crates/turingos-core/`
- `crates/turingos-task/`
- `tests/`
- `benchmarks/`
- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`

## 4. Out of Scope Files

- `turingos/`
- any real model-provider integration
- any network client or server integration
- any CLI crate or CLI implementation
- any scheduler module
- any benchmark migration code
- any constitution file

## 5. Surface Touch Matrix

- kernel: yes
- pluggable adapters: yes
- tools: no
- CLI: no
- validation: yes

## 6. Theorem-Cycle Boundary Implications

- Phase 5 may open only one bounded single-step cycle surface.
- The cycle may:
  - project a `ReadView` from one snapshot
  - ask an `IntentAdapter` for `AdapterResult`
  - route success into predicate evaluation and commit/abort
  - route adapter-local failure into an explicit non-commit outcome
- Phase 5 must not yet open:
  - real provider transport
  - CLI
  - scheduler embodiment
  - benchmark migration

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

- one bounded single-step driver surface in `turingos-kernel`
- tests proving:
  - successful one-step cycle
  - adapter-local failure stays outside commit authority
  - fixture adapter can drive the cycle without real provider integration
- one Phase 5 closeout packet

## 9. Acceptance Criteria

- one bounded single-step theorem cycle exists
- adapter-local failure does not require fake intents or implicit commit authority
- the cycle stays within current white-box and black-box boundaries
- no real provider integration is opened
- no CLI, scheduler, or benchmark migration surface is opened

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
- no real provider or network integration
- no CLI embodiment
- no scheduler embodiment
- no benchmark migration embodiment
- no constitution edits

## 13. Open Risks

- The first theorem-cycle embodiment can accidentally leak adapter failure into commit semantics if the outcome shape is vague
- Kernel and adapter crates can start to duplicate provenance or error authority if the seam is widened carelessly
- A bounded step driver can still accidentally become a disguised scheduler if it absorbs retry or orchestration policy
