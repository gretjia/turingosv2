# Mission Groundhog Phase 20 Spec Draft

Status: Active
Mission: Mission Groundhog
Phase: 20
Date: 2026-03-10 UTC

## 0. Phase State

- Phase 20 is now the active bounded stage.
- Phase 19 is closed and frozen by `handover/ops/MISSION_GROUNDHOG_PHASE_019_CLOSEOUT.md`.
- Phase 20 may not claim completion until it has:
  - frozen one deterministic parity golden history from the Python reference
  - added one Rust golden-replay integration test against that history
  - passed the required audit ring
  - written `handover/ops/MISSION_GROUNDHOG_PHASE_020_CLOSEOUT.md`
  - written one append-only entry matching `handover/ai-direct/entries/*_groundhog_phase020_*.md`

## 1. Objective

- Advance from generic integration harnesses to the first deterministic parity golden-replay lane.
- Freeze one normalized deterministic `alpha` parity history from the current Python reference implementation.
- Add one Rust integration test that uses that golden to assert bounded kernel replay behavior without porting the full parity task into product code.

## 2. Canonical Authority

- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
- `handover/ops/ACTIVE_MISSION_CHARTER.md`
- `handover/ops/MISSION_GROUNDHOG_PROJECT_SPEC.md`
- `handover/ops/MISSION_GROUNDHOG_EXECUTION_LOOP.md`
- `handover/ops/MISSION_GROUNDHOG_IMPLEMENTATION_GUARDRAILS.md`
- `handover/ops/GEMINI_GROUNDHOG_OPERATING_POLICY.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_019_CLOSEOUT.md`
- `tests/test_parity_runtime.py`
- `tests/conftest.py`
- `turingos/tasks/parity.py`
- `turingos/runtime.py`

## 3. In Scope Files

Writable in this phase:

- `crates/turingos-kernel/Cargo.toml`
- `crates/turingos-kernel/tests/parity_golden_replay.rs`
- `crates/turingos-kernel/tests/fixtures/parity_deterministic.json`
- `handover/ops/MISSION_GROUNDHOG_PHASE_020_SPEC_DRAFT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_020_CLOSEOUT.md`
- one new append-only entry matching `handover/ai-direct/entries/*_groundhog_phase020_*.md`
- `handover/ai-direct/LATEST.md`
- `handover/BOARD.md`

Read-only context:

- `turingos/tasks/parity.py`
- `turingos/runtime.py`
- `turingos/agents/parity.py`
- `tests/test_parity_runtime.py`
- `tests/conftest.py`
- `examples/parity_workspace/ledger.jsonl`
- `examples/parity_demo_output.json`
- `crates/turingos-core/`
- `crates/turingos-adapter/`
- `crates/turingos-kernel/src/`
- `crates/turingos-observe/`
- `crates/turingos-task/`

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

- kernel: validation only plus test-only fixture support
- observer crate: no product-code edits
- pluggable adapters: no product-code edits
- tools: no
- CLI: no
- validation: yes

## 6. Kernel/Plugin Boundary Implications

- Phase 20 may add only a test-only golden-replay lane around the existing public seams.
- The phase may:
  - freeze one normalized deterministic parity history from the Python reference
  - define one test-local task/adapter pair that replays that history through the Rust kernel
  - assert normalized step history and terminal success against the frozen golden
- The phase must not:
  - port the full parity runtime or scheduler into Rust product code
  - make the golden fixture a shadow product spec beyond the bounded normalized lane
  - widen kernel authority or feed observer packets back into kernel truth

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

- one deterministic parity golden fixture in JSON
- one Rust golden-replay integration test
- tests proving:
  - the frozen normalized history is reproduced by the Rust replay harness
  - terminal success and final answer match the golden
  - the lane remains bounded and does not become a full parity port
- one Phase 20 closeout packet

## 9. Acceptance Criteria

- the fixture is frozen from the current Python deterministic `alpha` path and recorded as normalized history rather than raw Python runtime dumps
- the Rust golden-replay test uses the current public kernel seams only
- the test proves at least:
  - step count
  - path sequence
  - write-mode sequence
  - halt behavior
  - terminal success
  - final result
- no product parity implementation is introduced
- the resulting lane is small enough to serve as the first frozen cross-language replay anchor rather than a premature embodiment commitment

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
- `/home/zephryj/.cargo/bin/cargo test -p turingos-kernel --test parity_golden_replay`
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

- The normalized golden can become a shadow spec if it keeps too much Python-only detail or strips away too much causal structure.
- The replay lane can bloat if it starts reproducing runtime subsystems rather than asserting the bounded normalized history.
- If the golden fixture is captured from the wrong Python path, the test will lock in noise or scheduler detail instead of the deterministic parity baseline.
