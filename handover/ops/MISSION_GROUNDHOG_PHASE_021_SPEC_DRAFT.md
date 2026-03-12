# Mission Groundhog Phase 21 Spec Draft

Status: Active
Mission: Mission Groundhog
Phase: 21
Date: 2026-03-10 UTC

## 0. Phase State

- Phase 21 is now the active bounded stage.
- Phase 20 is closed and frozen by `handover/ops/MISSION_GROUNDHOG_PHASE_020_CLOSEOUT.md`.
- Phase 21 may not claim completion until it has:
  - projected one deterministic parity golden run through the observer surfaces
  - added one bounded parity-shaped observer acceptance test
  - passed the required audit ring
  - written `handover/ops/MISSION_GROUNDHOG_PHASE_021_CLOSEOUT.md`
  - written one append-only entry matching `handover/ai-direct/entries/*_groundhog_phase021_*.md`

## 1. Objective

- Advance from kernel-only deterministic parity replay to the first parity-shaped observer acceptance harness.
- Reuse the same frozen deterministic `alpha` parity lane proved in Phase 20.
- Add one Rust integration test that projects the resulting real run through:
  - `RunReport`
  - `RunExport`
  - `RunSummary`
  - `RunPulseFrame`
- Turn that projected parity lane into the first canonical acceptance artifact above the kernel cycle, without widening product authority.

## 2. Canonical Authority

- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
- `handover/ops/ACTIVE_MISSION_CHARTER.md`
- `handover/ops/MISSION_GROUNDHOG_PROJECT_SPEC.md`
- `handover/ops/MISSION_GROUNDHOG_EXECUTION_LOOP.md`
- `handover/ops/MISSION_GROUNDHOG_IMPLEMENTATION_GUARDRAILS.md`
- `handover/ops/GEMINI_GROUNDHOG_OPERATING_POLICY.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_020_CLOSEOUT.md`
- `crates/turingos-kernel/tests/fixtures/parity_deterministic.json`
- `crates/turingos-kernel/tests/parity_golden_replay.rs`
- `tests/test_parity_runtime.py`
- `turingos/tasks/parity.py`
- `turingos/runtime.py`

## 3. In Scope Files

Writable in this phase:

- `crates/turingos-observe/Cargo.toml`
- `crates/test-support/parity_golden.rs`
- `crates/turingos-kernel/tests/parity_golden_replay.rs`
- `crates/turingos-observe/tests/parity_projection_fixture.rs`
- `handover/ops/MISSION_GROUNDHOG_PHASE_021_SPEC_DRAFT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_021_CLOSEOUT.md`
- one new append-only entry matching `handover/ai-direct/entries/*_groundhog_phase021_*.md`
- `handover/ai-direct/LATEST.md`
- `handover/BOARD.md`

Read-only context:

- `crates/turingos-kernel/tests/fixtures/parity_deterministic.json`
- `crates/turingos-kernel/src/`
- `crates/turingos-observe/src/`
- `crates/turingos-task/src/replay.rs`
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

- kernel: read-only validation only
- observer crate: yes, test-only parity acceptance harness
- task crate: no product-code edits
- pluggable adapters: no product-code edits
- tools: no
- CLI: no
- validation: yes

## 6. Kernel/Plugin Boundary Implications

- Phase 21 may add only one observer-side acceptance harness around the already-frozen deterministic parity lane.
- The phase may:
  - reuse the frozen Phase 20 parity golden
  - run it through the current Rust kernel
  - project the resulting real run through the current observer surfaces
  - assert parity-shaped acceptance facts on those projections
- The phase must not:
  - create a second product truth packet
  - promote parity into product Rust code
  - widen kernel authority
  - backflow observer packets into kernel truth

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

- one bounded parity-shaped observer acceptance test
- tests proving:
  - the same deterministic parity lane projects faithfully through `RunReport`
  - the same deterministic parity lane projects faithfully through `RunExport`
  - the same deterministic parity lane projects faithfully through `RunSummary`
  - the same deterministic parity lane projects faithfully through `RunPulseFrame`
  - no new truth packet or daemon logic appears in the observer layer
- one Phase 21 closeout packet

## 9. Acceptance Criteria

- the Phase 21 test reuses the Phase 20 frozen parity lane rather than inventing a second parity scenario
- the observer acceptance harness uses current public seams only
- the test proves at least:
  - attempted step count
  - committed step count
  - terminal step
  - halt class
  - success classification
  - final result visibility through observer projections
- no product parity implementation is introduced
- the resulting lane is small enough to serve as the first parity-shaped observer acceptance artifact rather than a new embodied subsystem

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

- `/home/zephryj/.cargo/bin/cargo fmt --check`
- `/home/zephryj/.cargo/bin/cargo test -p turingos-observe --test parity_projection_fixture`
- `/home/zephryj/.cargo/bin/cargo test`
- `./.venv/bin/python -m pytest -q`

## 12. Constraints

- no watchdog or supervisor daemon in any language
- no real provider or network integration
- no CLI embodiment
- no scheduler embodiment
- no full parity-port embodiment
- no constitution edits

## 13. Open Risks

- If the observer acceptance test duplicates too much Phase 20 harness logic, it can become wrapper inflation instead of a new acceptance artifact.
- If the observer assertions are too thin, the phase will not actually reduce the remaining gap to real `turingos` test readiness.
- If the phase invents a second frozen parity truth rather than reusing Phase 20, it will create drift instead of readiness.
