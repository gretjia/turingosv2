# Mission Groundhog Phase 19 Spec Draft

Status: Active
Mission: Mission Groundhog
Phase: 19
Date: 2026-03-10 UTC

## 0. Phase State

- Phase 19 is now the active bounded stage.
- Phase 18 is closed and frozen by `handover/ops/MISSION_GROUNDHOG_PHASE_018_CLOSEOUT.md`.
- Phase 19 may not claim completion until it has:
  - added one real cross-crate integration harness for the current Rust stack
  - passed the required audit ring
  - written `handover/ops/MISSION_GROUNDHOG_PHASE_019_CLOSEOUT.md`
  - written one append-only entry matching `handover/ai-direct/entries/*_groundhog_phase019_*.md`

## 1. Objective

- Prove that the current Rust embodiment can be exercised as one bounded integration path rather than only as disconnected unit slices.
- Add the first canonical test-only harness that pressures:
  - `KernelEngine::run(...)`
  - `RunReport`
  - `RunExport`
  - `RunSummary`
  - `RunPulseFrame`
- Keep the phase strictly test-first and bounded. Do not port the Python parity task yet and do not introduce provider, CLI, daemon, or scheduler embodiment.

## 2. Canonical Authority

- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
- `handover/ops/ACTIVE_MISSION_CHARTER.md`
- `handover/ops/MISSION_GROUNDHOG_PROJECT_SPEC.md`
- `handover/ops/MISSION_GROUNDHOG_EXECUTION_LOOP.md`
- `handover/ops/MISSION_GROUNDHOG_IMPLEMENTATION_GUARDRAILS.md`
- `handover/ops/GEMINI_GROUNDHOG_OPERATING_POLICY.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_017_CLOSEOUT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_018_CLOSEOUT.md`

## 3. In Scope Files

Writable in this phase:

- `crates/turingos-observe/Cargo.toml`
- `crates/turingos-observe/tests/run_cycle_fixture.rs`
- `handover/ops/MISSION_GROUNDHOG_PHASE_019_SPEC_DRAFT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_019_CLOSEOUT.md`
- one new append-only entry matching `handover/ai-direct/entries/*_groundhog_phase019_*.md`
- `handover/ai-direct/LATEST.md`
- `handover/BOARD.md`

Read-only context:

- `crates/turingos-core/`
- `crates/turingos-adapter/`
- `crates/turingos-kernel/`
- `crates/turingos-task/`
- `crates/turingos-observe/src/`
- `turingos/tasks/parity.py`
- `tests/test_parity_runtime.py`
- `tests/conftest.py`
- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`

## 4. Out of Scope Files

- `turingos/`
- any real provider implementation
- any network client or server integration
- any CLI crate or CLI implementation
- any scheduler module
- any daemon or supervisor logic
- any serialization or storage backend
- any parity-port implementation in Rust
- any constitution file

## 5. Surface Touch Matrix

- kernel: no product-code edits
- observer crate: yes
- pluggable adapters: no product-code edits
- tools: no
- CLI: no
- validation: yes

## 6. Kernel/Plugin Boundary Implications

- Phase 19 may add only a test harness around the existing public Rust seams.
- The phase may:
  - instantiate a bounded fixture task contract in test code
  - instantiate bounded deterministic and failure adapters in test code
  - drive a real `KernelEngine::run(...)` cycle
  - project the resulting run into observer surfaces
- The phase must not:
  - widen kernel authority
  - add product convenience wrappers just for tests
  - backflow observer surfaces into kernel truth
  - import parity runtime behavior into Rust without an explicit later stage

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

- one canonical cross-crate integration test file
- one successful bounded run fixture
- one bounded failure-path fixture
- tests proving:
  - run history survives projection through `RunReport` and `RunExport`
  - observer surfaces stay faithful to real run results rather than synthetic hand-built exports
  - no extra daemon, retry, or scheduler policy enters through the integration harness
- one Phase 19 closeout packet

## 9. Acceptance Criteria

- the integration harness uses the current public Rust seams only
- at least one real run reaches `RunPulseFrame` through actual kernel execution rather than a synthetic `RunExport` literal
- at least one non-success stop class is exercised through the real run path
- no new product API is introduced unless required to express the bounded harness cleanly
- the resulting harness is compact and legible enough to serve as the base for future parity or real-project scenarios

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
- `/home/zephryj/.cargo/bin/cargo test -p turingos-observe`
- `/home/zephryj/.cargo/bin/cargo test`
- `./.venv/bin/python -m pytest -q`

## 12. Constraints

- no watchdog or supervisor daemon in any language
- runtime observation must continue to be handled by polling agents rather than background daemon logic
- no real provider or network integration
- no CLI embodiment
- no scheduler embodiment
- no parity-port embodiment
- no constitution edits

## 13. Open Risks

- A test-only harness can still bloat if it tries to act like a product fixture library.
- If the harness is too synthetic, it will not materially advance readiness for real `turingos` integration testing.
- If the harness is too ambitious, it will accidentally become a partial parity port, which this phase forbids.
