# Handover Entry

## Summary

- Date: 2026-03-10 UTC
- Scope: draft `turingosv2` current-level 1M pressure-test project spec
- Owner: Codex Commander

## What Changed

- Read the historical `turingos` 1M baseline materials to extract only the measurement rules worth preserving.
- Read the current `turingosv2` Mission 002 benchmark, parity-case, prompt, and llama OpenAI adapter surfaces.
- Wrote a new draft project spec in `handover/ops/TURINGOSV2_1M_PRESSURE_TEST_SPEC_DRAFT.md`.
- Kept the benchmark boundary aligned with the existing runtime:
  - no kernel changes
  - no rerun of historical standalone model baselines
  - parity remains the benchmark object
  - raw first-fail evidence remains mandatory

## Evidence

- Historical references:
  - `/home/zephryj/projects/turingos/src/bench/million-baseline-compare.ts`
  - `/home/zephryj/projects/turingos/handover/audits/modelcompare/main_steps_baseline_20260228.md`
- Current benchmark surfaces:
  - `turingos/benchmarks/mission002.py`
  - `turingos/benchmarks/parity_cases.py`
  - `turingos/agents/llama_openai.py`
  - `turingos/agents/mission002_prompting.py`
  - `tests/test_mission002_runner.py`
  - `tests/test_llama_openai_agent.py`
  - `tests/test_mission002_cases.py`
- Existing authority and learning docs:
  - `handover/ops/MISSION_002_1M_TEST_SPEC.md`
  - `handover/ops/MISSION_002_WHITEBOX_RULES.md`
  - `handover/ops/MISSION_002_FAILURE_LEARNING.md`

## Open Risks

- The new project spec is drafted, not yet executed.
- The historical 1M discipline is clear, but the current endpoint state on the Mac must still be re-preflighted before the first official wave.
- The existing Mission 002 harness may need non-kernel execution and reporting extensions once the first longer gates move past `10`.

## Next Step

- Present the draft 1M pressure-test spec to the user, then execute the preflight and first smoke wave once approved.
