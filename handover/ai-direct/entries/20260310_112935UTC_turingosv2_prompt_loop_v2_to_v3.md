# Handover Entry

## Summary

- Mission: `turingosv2` 1M pressure-test prompt/debug loop
- Date: 2026-03-10 UTC
- Owner: Codex Commander

## What Changed

- Preserved the first approved live smoke wave as the fixed baseline for the current pressure-test cycle.
- Diagnosed the new `mission002.prompt.v2` headline failure as a generic `.ls` scan omission rather than a kernel or topology problem:
  - the headline dual lane now survives past the original step-`3` failure
  - it fails later at step `12`
  - failure class: when a `.ls` file contains a newly discovered `FILE *.md` entry and an older pending `todo[0]` remains first, both agents preserved the older pending item but omitted the newly discovered file from `todo`
- Updated the non-kernel mission prompt from `mission002.prompt.v2` to `mission002.prompt.v3`.
- Added one new generic invariant and one generic scan example:
  - discovered `FILE *.md` paths must still be appended to `todo`
  - this remains true even when `next_path` stays on an older pending item
- Added prompt regression coverage in `tests/test_mission002_prompting.py`.
- Stopped the stale multi-mode `v2` rerun after it had already produced enough headline evidence, then started a new dual-only `v3` rerun of case `000001`.

## Evidence

- Prior v2 headline rerun artifact:
  - `benchmarks/mission002/mission002_current_level_case1_promptv2_20260310_1112UTC/`
- New v3 rerun artifact root:
  - `benchmarks/mission002/mission002_dual_case1_promptv3_20260310_1127UTC/`
- Key failure evidence preserved:
  - `benchmarks/mission002/mission002_current_level_case1_promptv2_20260310_1112UTC/turingosv2_planner27_worker9/case_000001/case_result.json`
- Files changed:
  - `turingos/agents/mission002_prompting.py`
  - `tests/test_mission002_prompting.py`
- Tests:
  - `./.venv/bin/python -m pytest -q tests/test_mission002_prompting.py tests/test_mission002_runner.py tests/test_llama_openai_agent.py tests/test_mission002_cases.py`

## Open Risks

- The `v3` rerun was still in progress when this handover entry was written, so the new ceiling was not yet known.
- The 27B/9B Mac topology remains slow, so long response silence must still be treated carefully and not misclassified as a hang.

## Next Step

- Let the dual-only `v3` rerun continue until it either passes case `000001` or yields the next generic first-fail class, then continue the non-kernel loop toward the first-stage target of `100` consecutive passing cases.
