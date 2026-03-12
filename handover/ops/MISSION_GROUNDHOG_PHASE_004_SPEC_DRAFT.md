# Mission Groundhog Phase 4 Spec Draft

Status: Completed and frozen with PASS
Mission: Mission Groundhog
Phase: 4
Date: 2026-03-09 UTC

## 0. Phase State

- Phase 4 was the bounded active stage for this cycle and is now closed and frozen.
- Phase 3 is closed and frozen by `handover/ops/MISSION_GROUNDHOG_PHASE_003_CLOSEOUT.md`.
- Phase 4 may not claim completion until it has:
  - opened only the bounded adapter-boundary surface below
  - passed the required audit ring
  - written `handover/ops/MISSION_GROUNDHOG_PHASE_004_CLOSEOUT.md`
  - written one append-only entry matching `handover/ai-direct/entries/*_groundhog_phase004_closeout.md`

## 1. Objective

- Open the first provider-neutral black-box adapter boundary for Groundhog.
- Define a pluggable adapter crate that consumes `ReadView` and yields `IntentEnvelope` on success through an adapter-local fallible boundary.
- Prove the seam with fixture adapters only, while keeping all real model-provider, CLI, scheduler, and benchmark integration closed.

## 2. Canonical Authority

- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
- `handover/ops/ACTIVE_MISSION_CHARTER.md`
- `handover/ops/MISSION_GROUNDHOG_PROJECT_SPEC.md`
- `handover/ops/MISSION_GROUNDHOG_EXECUTION_LOOP.md`
- `handover/ops/MISSION_GROUNDHOG_IMPLEMENTATION_GUARDRAILS.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_001_ARCHITECTURE_PACKET.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_002_CLOSEOUT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_003_SPEC_DRAFT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_003_CLOSEOUT.md`

## 3. In Scope Files

Writable in this phase:

- `Cargo.toml`
- `Cargo.lock`
- `crates/turingos-adapter/Cargo.toml`
- `crates/turingos-adapter/src/lib.rs`
- `crates/turingos-adapter/src/boundary.rs`
- `crates/turingos-adapter/src/fixture.rs`
- `handover/ops/MISSION_GROUNDHOG_PHASE_004_SPEC_DRAFT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_004_CLOSEOUT.md`
- one new append-only entry matching `handover/ai-direct/entries/*_groundhog_phase004_closeout.md`
- `handover/ai-direct/LATEST.md`
- `handover/BOARD.md`

Read-only context:

- `crates/turingos-core/`
- `crates/turingos-kernel/`
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

- kernel: no
- pluggable adapters: yes
- tools: no
- CLI: no
- validation: yes

## 6. Kernel/Plugin Boundary Implications

- Phase 4 may open only the provider-neutral black-box adapter seam.
- Adapters may consume only `ReadView<QState>` and yield only:
  - `IntentEnvelope<QState>` on success
  - adapter-local boundary failure on provider or parse failure
- No adapter may receive:
  - mutable snapshot access
  - gate-pass authority
  - commit authority
  - task-policy authority
- Phase 4 must not yet open any real provider transport or prompt stack.

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

- `turingos-adapter` crate scaffold
- one provider-neutral adapter boundary trait
- one fixture adapter implementation for tests only
- inline Rust tests proving the seam stays inside `ReadView -> IntentEnvelope`
- one Phase 4 closeout packet

## 9. Acceptance Criteria

- `crates/turingos-adapter/` exists as the first black-box adapter boundary crate
- the adapter boundary consumes `ReadView` and yields only adapter-local failure or `IntentEnvelope`
- no real model-provider integration is opened
- no kernel authority is leaked into the adapter crate
- Phase 4 still keeps CLI, scheduler, and benchmark migration closed

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

- A provider-neutral adapter trait can still accidentally freeze prompt or transport assumptions too early
- fixture adapters must not quietly become semi-real providers
- the adapter crate must stay black-box boundary only, not a second task-policy home
