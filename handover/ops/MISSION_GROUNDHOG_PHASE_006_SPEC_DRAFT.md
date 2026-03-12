# Mission Groundhog Phase 6 Spec Draft

Status: Completed and frozen with PASS
Mission: Mission Groundhog
Phase: 6
Date: 2026-03-10 UTC

## 0. Phase State

- Phase 6 was the bounded active stage for this cycle and is now closed and frozen.
- Phase 5 is closed and frozen by `handover/ops/MISSION_GROUNDHOG_PHASE_005_CLOSEOUT.md`.
- Phase 6 may not claim completion until it has:
  - opened only the bounded observation/report surface below
  - passed the required audit ring
  - written `handover/ops/MISSION_GROUNDHOG_PHASE_006_CLOSEOUT.md`
  - written one append-only entry matching `handover/ai-direct/entries/*_groundhog_phase006_closeout.md`

## 1. Objective

- Open one bounded observation/report surface above the Phase 5 theorem cycle.
- Preserve adapter provenance, adapter-fault evidence, and step-local observational metadata outside kernel authority.
- Keep the kernel fact path mathematically narrow while preparing later provider integration to land without polluting ledger or predicate authority.

## 2. Canonical Authority

- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
- `handover/ops/ACTIVE_MISSION_CHARTER.md`
- `handover/ops/MISSION_GROUNDHOG_PROJECT_SPEC.md`
- `handover/ops/MISSION_GROUNDHOG_EXECUTION_LOOP.md`
- `handover/ops/MISSION_GROUNDHOG_IMPLEMENTATION_GUARDRAILS.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_001_ARCHITECTURE_PACKET.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_002_CLOSEOUT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_003_CLOSEOUT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_004_CLOSEOUT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_005_CLOSEOUT.md`

## 3. In Scope Files

Writable in this phase:

- `Cargo.toml`
- `Cargo.lock`
- `crates/turingos-core/src/lib.rs`
- `crates/turingos-core/src/intent.rs`
- one new bounded observation/report file under `crates/turingos-core/src/`
- `crates/turingos-kernel/src/lib.rs`
- `crates/turingos-kernel/src/driver.rs`
- `crates/turingos-kernel/src/engine.rs`
- `crates/turingos-kernel/src/commit.rs`
- `crates/turingos-kernel/src/predicate_gate.rs`
- `crates/turingos-kernel/src/trace.rs`
- `crates/turingos-adapter/src/lib.rs`
- `crates/turingos-adapter/src/boundary.rs`
- `crates/turingos-adapter/src/fixture.rs`
- `crates/turingos-task/src/replay.rs`
- `handover/ops/MISSION_GROUNDHOG_PHASE_006_SPEC_DRAFT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_006_CLOSEOUT.md`
- one new append-only entry matching `handover/ai-direct/entries/*_groundhog_phase006_closeout.md`
- `handover/ai-direct/LATEST.md`
- `handover/BOARD.md`

Read-only context:

- `crates/turingos-core/`
- `crates/turingos-kernel/src/engine.rs`
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

## 6. Kernel/Plugin Boundary Implications

- Phase 6 may open only one observation-side surface around a single step outcome.
- The new surface may preserve:
  - adapter provenance
  - adapter-fault detail
  - step-local observation fields useful for later provider integration
- The new surface must not:
  - modify commit or reject authority
  - alter predicate behavior
  - turn observation metadata into ledger-tip material
  - introduce retry, orchestration, or scheduler policy
- Provider identity must remain observational in Phase 6, not constitutive of kernel truth.

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

- one bounded observation/report type for single-step execution
- one driver-level outcome or wrapper that can carry observational metadata without changing kernel authority
- one adapter boundary refinement that can preserve observation-side metadata without turning it into commit truth
- extraction of live provider provenance from the active `IntentEnvelope` path so provenance becomes observation-only
- tests proving:
  - provider provenance stays observational
  - adapter faults can be preserved for reporting without becoming commit facts
  - the phase does not open scheduler or provider transport policy
- one Phase 6 closeout packet

## 9. Acceptance Criteria

- Phase 6 preserves observational metadata outside kernel authority
- no new provider, CLI, scheduler, or benchmark surfaces are opened
- commit and reject truth remain defined only by the current kernel and task contracts
- the new observation/report shape is compact, explicit, and not duplicated across kernel and adapter crates
- live provider provenance no longer resides in the active intent path

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

- Observation metadata can still become shadow authority if it is allowed to influence kernel truth indirectly
- The report surface can bloat if it starts absorbing transport, retry, or orchestration policy
- Future provider integration may still pressure the current `IntentEnvelope` shape if phase boundaries are not kept sharp
