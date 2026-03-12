# Mission Groundhog Phase 22 Spec Draft

Status: Active
Mission: Mission Groundhog
Phase: 22
Date: 2026-03-10 UTC

## 0. Phase State

- Phase 22 is now the active bounded stage.
- Phase 21 is closed and frozen by `handover/ops/MISSION_GROUNDHOG_PHASE_021_CLOSEOUT.md`.
- Phase 22 may not claim completion until it has:
  - defined one canonical repo-level parity formal test gate
  - implemented one bounded executable entrypoint for that gate
  - validated that gate locally
  - passed the required audit ring
  - written `handover/ops/MISSION_GROUNDHOG_PHASE_022_CLOSEOUT.md`
  - written one append-only entry matching `handover/ai-direct/entries/*_groundhog_phase022_*.md`

## 1. Objective

- Advance from parity replay and observer acceptance artifacts to one canonical repo-level formal test gate for real `turingos` project pressure.
- Join exactly three already-proven surfaces:
  - the Rust kernel replay anchor
  - the Rust observer acceptance harness
  - the current Python `turingos` parity runtime test surface
- Make that gate executable as one explicit bounded project command without introducing provider, daemon, or scheduler embodiment.

## 2. Canonical Authority

- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
- `handover/ops/ACTIVE_MISSION_CHARTER.md`
- `handover/ops/MISSION_GROUNDHOG_PROJECT_SPEC.md`
- `handover/ops/MISSION_GROUNDHOG_EXECUTION_LOOP.md`
- `handover/ops/MISSION_GROUNDHOG_IMPLEMENTATION_GUARDRAILS.md`
- `handover/ops/GEMINI_GROUNDHOG_OPERATING_POLICY.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_020_CLOSEOUT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_021_CLOSEOUT.md`
- `crates/turingos-kernel/tests/parity_golden_replay.rs`
- `crates/turingos-observe/tests/parity_projection_fixture.rs`
- `tests/test_parity_runtime.py`

## 3. In Scope Files

Writable in this phase:

- `scripts/groundhog_parity_formal_test.sh`
- `handover/ops/MISSION_GROUNDHOG_PHASE_022_SPEC_DRAFT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_022_CLOSEOUT.md`
- one new append-only entry matching `handover/ai-direct/entries/*_groundhog_phase022_*.md`
- `handover/ai-direct/LATEST.md`
- `handover/BOARD.md`

Read-only context:

- `crates/test-support/parity_golden.rs`
- `crates/turingos-kernel/tests/parity_golden_replay.rs`
- `crates/turingos-observe/tests/parity_projection_fixture.rs`
- `tests/test_parity_runtime.py`
- `turingos/tasks/parity.py`
- `turingos/runtime.py`

## 4. Out of Scope Files

- `turingos/`
- any product parity port in Rust
- any real provider implementation
- any network client or server integration
- any CLI crate or CLI implementation
- any daemon or supervisor logic
- any scheduler embodiment
- any constitution file

## 5. Surface Touch Matrix

- kernel: no
- observer crate: no
- task crate: no
- project test gate: yes
- tools/scripts: yes
- CLI: no
- validation: yes

## 6. Kernel/Plugin Boundary Implications

- Phase 22 may add only one repo-level formal test gate over already-proven bounded surfaces.
- The phase may:
  - sequence existing Rust and Python parity test commands
  - make the formal gate reproducible as one explicit entrypoint
  - document that gate in handover
- The phase must not:
  - add provider logic
  - add scheduler logic
  - add background polling or daemon behavior
  - smuggle new test truth into product code

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

- one canonical repo-level parity formal test gate
- one executable entrypoint for that gate
- validation proving the gate runs the bounded Rust and Python parity lanes in one place
- one Phase 22 closeout packet

## 9. Acceptance Criteria

- the gate reuses the already-proven parity lanes instead of inventing new ones
- the gate is one explicit bounded project entrypoint
- the gate includes at least:
  - `cargo test -p turingos-kernel --test parity_golden_replay`
  - `cargo test -p turingos-observe --test parity_projection_fixture`
  - `./.venv/bin/python -m pytest -q tests/test_parity_runtime.py`
- no provider, daemon, CLI, or scheduler embodiment is introduced
- the resulting gate is small enough to serve as the first real `turingos` formal test entrypoint

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
- `cargo test -p turingos-kernel --test parity_golden_replay`
- `cargo test -p turingos-observe --test parity_projection_fixture`
- `./.venv/bin/python -m pytest -q tests/test_parity_runtime.py`
- `bash scripts/groundhog_parity_formal_test.sh`

## 12. Constraints

- no watchdog or supervisor daemon in any language
- no real provider or network integration
- no CLI embodiment
- no scheduler embodiment
- no full parity-port embodiment
- no constitution edits

## 13. Open Risks

- If the gate widens into a general orchestrator, it will violate the bounded-stage rule.
- If the gate omits one of the already-proven parity lanes, it will fail to become a real readiness boundary.
- If the gate grows hidden environment assumptions, it will look ready without actually being reproducible.
