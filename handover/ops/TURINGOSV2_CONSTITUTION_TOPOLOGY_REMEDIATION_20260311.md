# TuringOSv2 Constitution Topology Remediation

Status: Active remediation record
Date: 2026-03-11 UTC
Scope:
- follow-up to `handover/ops/TURINGOSV2_CONSTITUTION_TOPOLOGY_ALIGNMENT_AUDIT_20260311.md`

## Fixed High-Priority Items

### 1. Python invalid-proposal commit bypass is now sealed

Changed files:
- `turingos/runtime.py`
- `tests/test_masking_and_scheduler.py`

What changed:
- `TuringOSConfig` now rejects `abort_on_no_valid_proposal=False`.
- `TuringOSKernel.step_once()` now always aborts on any non-`hard_pass` proposal before `_apply(...)`.

Constitution effect:
- The Python runtime no longer permits a `∏p = 0` proposal to cross the predicate barrier and reach white-box commit.

### 2. Python black-box boundary no longer receives a live `MachineState`

Changed files:
- `turingos/models.py`
- `turingos/masking.py`
- `tests/test_masking_and_scheduler.py`
- `tests/test_mission002_prompting.py`
- `tests/test_llama_openai_agent.py`

What changed:
- Added `FrozenMachineState`.
- `MachineState.clone()` now deep-copies nested register state.
- `MachineState.frozen()` now produces a deep-copied snapshot.
- `MaskingPolicy.build_view(...)` now passes `state.frozen()` into `AgentView`.

Constitution effect:
- Python black-box agents now receive a sealed snapshot-style input fragment instead of a live mutable kernel state object.

### 3. Theorem-bearing black-box input is now reduced to the sealed pair

Changed files:
- `turingos/models.py`
- `turingos/masking.py`
- `turingos/runtime.py`
- `harnesses/mission002_py/mission002_prompting.py`
- `tests/test_masking_and_scheduler.py`
- `tests/test_mission002_prompting.py`
- `tests/test_llama_openai_agent.py`
- `crates/turingos-core/src/intent.rs`
- `crates/turingos-kernel/src/read.rs`
- `crates/turingos-kernel/src/engine.rs`
- `crates/turingos-task/src/contract.rs`
- `crates/turingos-task/src/replay.rs`
- `crates/turingos-adapter/src/fixture.rs`
- `crates/turingos-observe/tests/run_cycle_fixture.rs`

What changed:
- Python `AgentView` no longer carries `public_broadcasts`, `private_feedback`, or `price_hint`.
- Python masking and runtime projection now expose only frozen register state plus current content.
- Mission-002 prompt payloads now project only `agent_id`, `state`, and `current_content`.
- Rust `ReadView` no longer carries `public_notes`.
- Rust task contracts, replay fixtures, adapter fixtures, and observe tests were tightened to the same reduced read surface.

Constitution effect:
- The theorem-bearing black-box input surface now matches the sealed constitutional shape much more closely:
  - `q_t`
  - `rtool(tape_t, HEAD_t)`
- Broadcasts, feedback, price traces, and other overlays may still exist in the wider system as diagnostics, but they no longer cross the constitutional read boundary as black-box input.

## Remaining Open Drift

### 1. Python still represents `Q_t` in a split form

Still open:
- `MachineState`
- `WorkspaceFS`
- `AppendOnlyLedger`

Current interpretation:
- Functionally serviceable, but still less physically closed than the Rust `UniverseSnapshot` embodiment.

## Verification

- `./.venv/bin/python -m pytest -q`
- `./.venv/bin/python -m pytest -q tests/test_masking_and_scheduler.py tests/test_parity_runtime.py tests/test_mission002_prompting.py tests/test_llama_openai_agent.py`
- `/home/zephryj/.cargo/bin/cargo test -q`

## Recommended Next Step

- Keep the original alignment audit as the “discovery” record.
- Treat this remediation record as proof that the two constitution-level Python leaks and the widened theorem-bearing input surface have now been sealed.
- Handle the remaining split-`Q_t` question as the next explicit constitutional design decision.
