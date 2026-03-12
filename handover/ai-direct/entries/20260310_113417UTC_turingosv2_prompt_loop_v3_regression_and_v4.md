# Handover Entry

## Summary

- Mission: `turingosv2` 1M pressure-test prompt/debug loop
- Date: 2026-03-10 UTC
- Owner: Codex Commander

## What Changed

- Completed the `mission002.prompt.v3` headline dual-only rerun for case `000001`.
- Preserved the `v3` regression artifact:
  - `v3` did not improve the ceiling
  - it failed earlier at step `7`
  - failure class: the compacted step-`7` numeric-file transition regressed, with one agent writing parity too early and the other corrupting `pending.bit`
- Replaced `mission002.prompt.v3` with `mission002.prompt.v4`.
- `v4` is intentionally shorter than `v3`:
  - keep the generic FILE-entry invariant needed for the old step-`12` omission
  - remove the extra long scan example
  - add two short numeric-file invariants to protect the `scan -> apply_pending` boundary
- Updated `tests/test_mission002_prompting.py` to match `mission002.prompt.v4`.
- Started a new dual-only `v4` rerun of case `000001`.

## Evidence

- `v3` artifact root:
  - `benchmarks/mission002/mission002_dual_case1_promptv3_20260310_1127UTC/`
- `v4` active artifact root:
  - `benchmarks/mission002/mission002_dual_case1_promptv4_20260310_1134UTC/`
- Files changed:
  - `turingos/agents/mission002_prompting.py`
  - `tests/test_mission002_prompting.py`
- Tests:
  - `./.venv/bin/python -m pytest -q tests/test_mission002_prompting.py tests/test_mission002_runner.py tests/test_llama_openai_agent.py tests/test_mission002_cases.py`

## Open Risks

- The active `v4` rerun was still in progress when this entry was written.
- The dual-lane benchmark still shows meaningful model variance even under the fixed kernel and topology, so each prompt change must be judged by preserved artifacts rather than intuition alone.

## Next Step

- Let the active `v4` rerun complete, inspect whether it passes both the old step-`7` and old step-`12` ceilings, then continue the non-kernel loop toward the first-stage target of `100` consecutive passing cases.
