# Mission Groundhog Phase 9 Spec Draft

Status: Completed and frozen after Phase 9 PASS
Mission: Mission Groundhog
Phase: 9
Date: 2026-03-10 UTC

## 0. Phase State

- Phase 9 is now the active bounded stage.
- Phase 8 is closed and frozen by `handover/ops/MISSION_GROUNDHOG_PHASE_008_CLOSEOUT.md`.
- Phase 9 may not claim completion until it has:
  - opened only the bounded run-report surface below
  - passed the required audit ring
  - written `handover/ops/MISSION_GROUNDHOG_PHASE_009_CLOSEOUT.md`
  - written one append-only entry matching `handover/ai-direct/entries/*_groundhog_phase009_closeout.md`

## 1. Objective

- Open the first canonical read-only run-report surface above the Phase 8 bounded run loop.
- Reuse `RunOutcome`, `CommittedStep`, and `RunStop` rather than re-deriving kernel history.
- Produce one provider-neutral packet that later CLI, benchmark, and polling-agent layers can consume without entering kernel theorem authority.

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
- `handover/ops/MISSION_GROUNDHOG_PHASE_008_CLOSEOUT.md`

## 3. In Scope Files

Writable in this phase:

- `Cargo.toml`
- `Cargo.lock`
- `crates/turingos-kernel/src/lib.rs`
- `crates/turingos-kernel/src/run.rs`
- `crates/turingos-kernel/src/report.rs`
- `handover/ops/MISSION_GROUNDHOG_PHASE_009_SPEC_DRAFT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_009_CLOSEOUT.md`
- one new append-only entry matching `handover/ai-direct/entries/*_groundhog_phase009_closeout.md`
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
- any serialization or storage backend
- any benchmark migration code
- any constitution file

## 5. Surface Touch Matrix

- kernel: yes
- pluggable adapters: no
- tools: no
- CLI: no
- validation: yes

## 6. Kernel/Plugin Boundary Implications

- Phase 9 may project a completed `RunOutcome` into one read-only report packet.
- The phase may:
  - define one canonical report shape
  - normalize committed history and terminal stop class for read-side consumers
  - provide provider-neutral helper construction for that report
- The phase must not:
  - mutate kernel state during report construction
  - add scheduler policy
  - add retry policy
  - add provider orchestration
  - add daemon or watchdog logic

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

- one canonical read-only run-report surface above `RunOutcome`
- one explicit report shape covering:
  - attempted step count
  - committed-history slice
  - terminal stop classification
  - terminal snapshot or preserved snapshot facts as appropriate
- tests proving:
  - report construction does not invent uncommitted history
  - report construction preserves stop classification
  - report construction stays read-only and policy-free
- one Phase 9 closeout packet

## 9. Acceptance Criteria

- the report surface is read-only and white-box auditable
- run history remains sourced only from Phase 8 committed history
- no scheduler, retry, CLI, provider, or daemon logic is introduced
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

- A report packet can accidentally become a second source of truth if it re-derives kernel facts instead of projecting them.
- Over-eager normalization can smuggle policy into what should remain a read-only summary layer.
- The next pressure after report normalization may be replay or benchmark integration, so Phase 9 must not widen into those layers prematurely.
