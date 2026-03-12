# Handover Entry

## Summary

- Date: 2026-03-10 UTC
- Scope: first approved live current-level smoke wave for the draft `turingosv2` 1M pressure test
- Owner: Codex Commander

## What Changed

- Added non-kernel manifest improvements to the Mission 002 benchmark runner so the run now records:
  - prompt version
  - topology metadata
  - remote model targets
  - launch flags
  - repo revision
  - `windows1` participation flag
- Added non-pass `ledger_full.jsonl` capture.
- Preflighted the Mac-hosted `llama.cpp` services for 27B and 9B and created a fresh local-forward topology on `18082/18083`.
- Executed the first smoke wave for:
  - `turingosv2_planner27_worker9`
  - `turingosv2_single_27b`
  - `turingosv2_single_9b`
- Stopped scored progression at the first rung because all three runtime lanes failed on case `000001`.
- Wrote the resulting learning record to `handover/ops/TURINGOSV2_1M_PRESSURE_TEST_WAVE1_LEARNING.md`.

## Evidence

- Run dir:
  - `benchmarks/mission002/mission002_current_level_smoke_20260310_1049UTC/`
- Key raw files:
  - `benchmarks/mission002/mission002_current_level_smoke_20260310_1049UTC/run_manifest.json`
  - `benchmarks/mission002/mission002_current_level_smoke_20260310_1049UTC/overall_summary.json`
  - `benchmarks/mission002/mission002_current_level_smoke_20260310_1049UTC/turingosv2_planner27_worker9/case_000001/case_result.json`
  - `benchmarks/mission002/mission002_current_level_smoke_20260310_1049UTC/turingosv2_single_27b/case_000001/case_result.json`
  - `benchmarks/mission002/mission002_current_level_smoke_20260310_1049UTC/turingosv2_single_9b/case_000001/case_result.json`
- Updated docs:
  - `handover/ops/TURINGOSV2_1M_PRESSURE_TEST_SPEC_DRAFT.md`
  - `handover/ops/TURINGOSV2_1M_PRESSURE_TEST_WAVE1_LEARNING.md`
- Validation:
  - `./.venv/bin/python -m pytest -q tests/test_mission002_runner.py tests/test_llama_openai_agent.py tests/test_mission002_cases.py`

## Open Risks

- The current runtime headline score is still `0`.
- The dual lane did not yet demonstrate effective heterogeneous rescue under the current prompt contract.
- Longer ladders are not justified until the same case-1 failure family is improved and re-tested.

## Next Step

- Treat this Wave 1 corpus as the authority dataset for the next non-kernel prompt/adapter improvement pass, then rerun case `000001` before reopening the `3` and `10` gates.
