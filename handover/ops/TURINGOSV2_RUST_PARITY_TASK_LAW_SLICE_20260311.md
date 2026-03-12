# TuringOSv2 Rust Parity Task-Law Slice

Date: 2026-03-11 UTC
Status: Applied
Scope:
- first Rust port of the current parity white-box task law
- keep parity outside the generic kernel physics while removing Python as the sole owner of that law

## Objective

- Stop treating `turingos/tasks/parity.py` as the only repository embodiment of the current parity task truth.
- Move the parity admissibility law into a trusted Rust task-law layer.
- Preserve the constitutional separation:
  - kernel physics stay generic
  - task law stays white-box
  - benchmark/harness logic stays outside the theorem-bearing core

## Constitutional Basis

- Sealed source:
  - `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
- Derived explanatory authority:
  - `handover/ops/TURINGOSV2_CONSTITUTION_AI_FRIENDLY_REWRITE_20260311.md`
- Audited Rust-only boundary:
  - `handover/ops/TURINGOSV2_RUST_ONLY_BLUEPRINT_AUDITED_20260311.md`

## What Changed

- Added a new trusted Rust task-law crate:
  - `crates/turingos-parity/`
- Implemented:
  - `ParityPhase`
  - `ParityRegister`
  - `ParityTask`
  - `initial_world(...)`
  - `initial_snapshot(...)`
  - the parity law as:
    - `expected_intent(world)`
    - `projected_intent(read_view)`
    - `TaskContract::evaluate(...)`
    - `TaskContract::is_success(...)`
- Kept parity outside the generic kernel stage files:
  - no parity-specific logic was merged into `read.rs`, `predicate_gate.rs`, `commit.rs`, or `run.rs`
- Added Rust unit tests for:
  - `.ls` frontier ordering
  - `apply_pending` parity XOR behavior
  - `finalize -> write_result -> halt`
  - `wrong_path` rejection
  - success detection
  - a live Rust parity execution lane via `KernelEngine::run(...)`

## Why This Slice Is Safe

- It does not widen the BLACKBOX seam.
- It does not move benchmark orchestration into the kernel.
- It keeps the kernel generic while making the task law white-box and trusted.
- It matches the audited direction:
  - white-box task rules may be plugged in
  - generic kernel physics must remain small and explicit

## AgentOS Review

- Ramanujan line:
  - `PASS`
  - parity law belongs in a trusted Rust task-law crate/module outside the generic kernel physics
- Turing line:
  - pending live response at the time this slice was recorded
- Commander decision:
  - proceed with the separate trusted crate shape rather than hardcoding parity into kernel internals

## Validation

- `cd /home/zephryj/projects/turingosv2 && /home/zephryj/.cargo/bin/cargo test -p turingos-parity -q`
- `cd /home/zephryj/projects/turingosv2 && /home/zephryj/.cargo/bin/cargo test -q`
- `cd /home/zephryj/projects/turingosv2 && ./.venv/bin/python -m pytest -q`

## Remaining Work

- Python `turingos/tasks/parity.py` still remains as the live reference path for the Python runtime.
- Python `turingos/runtime.py` still drives the current Python parity demo and Python runtime tests.
- But Python runtime is no longer the only live repository proof that parity can execute to success:
  - the Rust parity crate now contains a deterministic adapter-driven live execution test over the Rust kernel.
- The next meaningful migration slice is not more shell deletion.
- The next meaningful migration slice is:
  - turn the new test-only Rust parity execution lane into an operator-facing lane
  - then retire Python parity/runtime truth from active execution
