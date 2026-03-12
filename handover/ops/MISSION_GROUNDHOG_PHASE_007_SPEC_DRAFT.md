# Mission Groundhog Phase 7 Spec Draft

Status: Completed and frozen after Phase 7 PASS on 2026-03-10 UTC
Mission: Mission Groundhog
Phase: 7
Date: 2026-03-10 UTC

## 0. Phase State

- Phase 7 is now the active bounded stage.
- Phase 6 is closed and frozen by `handover/ops/MISSION_GROUNDHOG_PHASE_006_CLOSEOUT.md`.
- Phase 7 may not claim completion until it has:
  - opened only the bounded observation/report simplification surface below
  - passed the required audit ring
  - written `handover/ops/MISSION_GROUNDHOG_PHASE_007_CLOSEOUT.md`
  - written one append-only entry matching `handover/ai-direct/entries/*_groundhog_phase007_closeout.md`

## 1. Objective

- Simplify the Phase 6 observation/report seam without changing theorem authority.
- Reduce wrapper depth between the adapter boundary and the kernel driver outcome.
- Improve compactness and readability while keeping provider provenance observational only.

## 2. Canonical Authority

- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
- `handover/ops/ACTIVE_MISSION_CHARTER.md`
- `handover/ops/MISSION_GROUNDHOG_PROJECT_SPEC.md`
- `handover/ops/MISSION_GROUNDHOG_EXECUTION_LOOP.md`
- `handover/ops/MISSION_GROUNDHOG_IMPLEMENTATION_GUARDRAILS.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_001_ARCHITECTURE_PACKET.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_005_CLOSEOUT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_006_CLOSEOUT.md`

## 3. In Scope Files

Writable in this phase:

- `Cargo.toml`
- `Cargo.lock`
- `crates/turingos-core/src/lib.rs`
- `crates/turingos-core/src/observation.rs`
- `crates/turingos-adapter/src/lib.rs`
- `crates/turingos-adapter/src/boundary.rs`
- `crates/turingos-adapter/src/fixture.rs`
- `crates/turingos-kernel/src/lib.rs`
- `crates/turingos-kernel/src/driver.rs`
- `crates/turingos-kernel/src/engine.rs`
- `crates/turingos-kernel/src/predicate_gate.rs`
- `handover/ops/MISSION_GROUNDHOG_PHASE_007_SPEC_DRAFT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_007_CLOSEOUT.md`
- one new append-only entry matching `handover/ai-direct/entries/*_groundhog_phase007_closeout.md`
- `handover/ai-direct/LATEST.md`
- `handover/BOARD.md`

Read-only context:

- `crates/turingos-core/src/intent.rs`
- `crates/turingos-kernel/src/{commit.rs,trace.rs}`
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

- Phase 7 may simplify observation/report wrappers, but it may not move observation metadata back into kernel truth.
- The phase may:
  - flatten one wrapper layer
  - reduce observation constructor ceremony
  - reduce redundant report shapes
  - relax internal `Sized` assumptions that would otherwise prevent the public kernel seam from accepting provider-neutral trait-object adapters or contracts
- The phase must not:
  - change commit or reject semantics
  - reopen live provider provenance inside the active intent path
  - introduce provider transport, retry, orchestration, or scheduler policy

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

- one thinner observation/report seam between adapter and kernel driver
- one simplified observation API shape
- tests proving:
  - provider provenance remains observational only
  - simplification did not change commit or reject truth
  - no scheduler or provider policy was introduced
- one Phase 7 closeout packet

## 9. Acceptance Criteria

- observation/report wrapper depth is reduced
- theorem authority remains unchanged
- provider provenance remains outside the active kernel intent path
- the resulting API is more compact and readable than the Phase 6 shape

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

- Simplification can accidentally reintroduce hidden authority mixing if wrapper removal is careless
- Too much eagerness to beautify can widen scope beyond the current bounded seam
- The next real provider phase will still need disciplined staging after this cleanup
