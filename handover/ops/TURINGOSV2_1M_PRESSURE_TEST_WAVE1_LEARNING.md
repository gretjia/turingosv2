# TuringOSv2 1M Pressure Test Wave 1 Learning

Status: Active learning record
Date: 2026-03-10 UTC
Scope: first approved current-level smoke wave under the draft 1M pressure-test spec

## 1. Run Identity

- Run name: `mission002_current_level_smoke_20260310_1049UTC`
- Artifact root: `benchmarks/mission002/mission002_current_level_smoke_20260310_1049UTC/`
- Spec authority:
  - `handover/ops/TURINGOSV2_1M_PRESSURE_TEST_SPEC_DRAFT.md`
  - `handover/ops/MISSION_002_1M_TEST_SPEC.md`
  - `handover/ops/MISSION_002_WHITEBOX_RULES.md`

## 2. Preflight Facts

- Source-of-truth runner: `omega-vm`
- Model host: `zephrymac-studio`
- Local forwarded ports used for this run:
  - `127.0.0.1:18082 -> 127.0.0.1:8080`
  - `127.0.0.1:18083 -> 127.0.0.1:8081`
- Declared models:
  - `qwen3.5-27b-instruct-q4_k_m.gguf`
  - `Qwen3.5-9B-Q4_K_M.gguf`
- Declared prompt version: `mission002.prompt.v1`
- Declared generator version: `mission002.parity.v1`
- Declared repo revision: `d9f0357e0ce9d2843a06d2a3d25ee342b2dfce85`
- `windows1` did not participate in this first wave.

## 3. Primary Result

All three official runtime lanes failed on case `000001`, so the smoke ladder correctly stopped at the first rung.

- `turingosv2_planner27_worker9`
  - `consecutive_pass_before_first_fail = 0`
  - first fail: step `3`
  - failure class: semantic
  - reason: `wrong_register`
- `turingosv2_single_27b`
  - `consecutive_pass_before_first_fail = 0`
  - first fail: step `3`
  - failure class: semantic
  - reason: `wrong_register`
- `turingosv2_single_9b`
  - `consecutive_pass_before_first_fail = 0`
  - first fail: step `2`
  - failure class: semantic
  - reasons:
    - `wrong_path`
    - `wrong_write_mode`
    - `wrong_write_content`
    - `wrong_register`
    - `write_protected`

## 4. Headline Interpretation

- The current headline runtime score for `turingosv2` remains `0`.
- The dual lane did not outperform the single `27B` lane on this first case.
- The selected-agent trace for the dual lane shows `planner27` carried the failing path; the `worker9` lane did not rescue the run.
- The `9B` worker lane still breaks earlier and in a more structurally obvious way than the `27B` lane.

## 5. Raw Evidence Pointers

- Run manifest:
  - `benchmarks/mission002/mission002_current_level_smoke_20260310_1049UTC/run_manifest.json`
- Overall summary:
  - `benchmarks/mission002/mission002_current_level_smoke_20260310_1049UTC/overall_summary.json`
- Dual lane first fail:
  - `benchmarks/mission002/mission002_current_level_smoke_20260310_1049UTC/turingosv2_planner27_worker9/case_000001/case_result.json`
  - `benchmarks/mission002/mission002_current_level_smoke_20260310_1049UTC/turingosv2_planner27_worker9/case_000001/ledger_full.jsonl`
- Single 27B first fail:
  - `benchmarks/mission002/mission002_current_level_smoke_20260310_1049UTC/turingosv2_single_27b/case_000001/case_result.json`
  - `benchmarks/mission002/mission002_current_level_smoke_20260310_1049UTC/turingosv2_single_27b/case_000001/ledger_full.jsonl`
- Single 9B first fail:
  - `benchmarks/mission002/mission002_current_level_smoke_20260310_1049UTC/turingosv2_single_9b/case_000001/case_result.json`
  - `benchmarks/mission002/mission002_current_level_smoke_20260310_1049UTC/turingosv2_single_9b/case_000001/ledger_full.jsonl`

## 6. What Improved Relative To The Old Baseline Capture

- The manifest now records:
  - prompt version
  - topology host alias
  - remote model targets
  - `llama.cpp` launch flags
  - repo revision
  - whether `windows1` participated
- Non-pass cases now preserve `ledger_full.jsonl` in addition to `ledger_tail`.
- The current run therefore has a cleaner topology and provenance chain than the earlier Mission 002 smoke corpus.

## 7. Immediate Engineering Signal

- This run does not justify moving to deeper smoke rungs or staircase gates yet.
- The next bottleneck is still proposal-contract fidelity inside the current parity runtime, not infrastructure reachability.
- The most valuable upgrade targets remain:
  - scan-phase register discipline
  - `.ls` path normalization
  - write-protection obedience
  - planner/worker role separation that actually changes selected behavior under the same public interface

## 8. Constraints Reconfirmed

- No kernel changes were made for this run.
- No standalone direct-model baseline was rerun.
- The benchmark remained inside the existing Mission 002 non-kernel surfaces.

## 9. Next Step

- Use these Wave 1 artifacts as the authority corpus for the next prompt/adapter improvement pass.
- Re-run the same first rung before attempting `3` or `10` again.
