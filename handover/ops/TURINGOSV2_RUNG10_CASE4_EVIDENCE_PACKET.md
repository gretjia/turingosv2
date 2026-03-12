# TuringOSv2 Rung 10 Case 000004 Evidence Packet

Status: Active
Date: 2026-03-10 UTC
Scope: Evidence-only compression packet for the first official `rung 10` semantic failure

## 1. Authority

Primary sources:

- `benchmarks/mission002/mission002_dual_rung10_promptv9_mt768_20260310_1943UTC/turingosv2_planner27_worker9/summary.json`
- `benchmarks/mission002/mission002_dual_rung10_promptv9_mt768_20260310_1943UTC/turingosv2_planner27_worker9/case_000004/case_result.json`
- `benchmarks/mission002/mission002_dual_rung10_promptv9_mt768_20260310_1943UTC/turingosv2_planner27_worker9/case_000004/workspace/ledger.jsonl`
- `turingos/tasks/parity.py`
- `harnesses/mission002_py/mission002_prompting.py`

Interpretation rule:

- This packet compresses the recorded evidence only.
- Any proposed fix must stay within the current non-kernel benchmark surface.

## 2. Official Result

- Mode: `turingosv2_planner27_worker9`
- Official scored run: `mission002_dual_rung10_promptv9_mt768_20260310_1943UTC`
- Outcome:
  - `attempted = 4`
  - `passed = 3`
  - `failed = 1`
  - `first_non_pass_case = 4`
  - `consecutive_pass_before_first_fail = 3`
  - `status = FAIL`

Recorded reason:

- `no valid proposal at step 15: wrong_path:expected next_path=dir_01_31/dir_02_30/n_02_536.md, got parity.md; wrong_register:next_register diverged from expected policy`

## 3. Whitebox State At First Fail

Immediately before the failing proposal:

- `HEAD_t = dir_01_31/dir_02_30/.ls`
- `phase = scan`
- `pending = null`
- `todo = ["dir_01_31/dir_02_30/.ls"]`
- `parity.md = "1"`
- `result.md` absent

Relevant tape slice:

```text
.ls
DIR dir_00_11
DIR dir_01_31
FILE n_00_639.md
FILE n_04_130.md
FILE n_05_212.md

dir_00_11/.ls
FILE n_01_831.md

dir_01_31/.ls
DIR dir_02_30
FILE n_03_954.md

dir_01_31/dir_02_30/.ls
FILE n_02_536.md

parity.md = 1
```

## 4. Whitebox Expected Transition

Under `ParityTask.expected_transition(...)`, the `.ls` scan at `dir_01_31/dir_02_30/.ls` must:

- append `dir_01_31/dir_02_30/.ls` to `completed`
- remove the current `.ls` path from `todo`
- discover `dir_01_31/dir_02_30/n_02_536.md`
- compute new `todo = ["dir_01_31/dir_02_30/n_02_536.md"]`
- remain in `phase = scan`
- set `next_path = dir_01_31/dir_02_30/n_02_536.md`
- keep `write_mode = keep`
- keep `write_content = null`

## 5. Actual Failure Law

Compressed failure law:

- In a terminal `.ls` scan whose listing contains exactly one discovered `FILE` and no remaining older `todo` items, the current prompt contract can still collapse the run into an invalid jump back to `parity.md` instead of consuming the newly discovered file.

This is not:

- a kernel failure
- a topology failure
- a transport failure
- a numeric parity-computation failure

This is:

- a proposal-contract failure on a specific `.ls` terminal-frontier archetype

## 6. Anti-Hardcoding Boundary

Forbidden fixes:

- special-casing `case_000004`
- special-casing `dir_01_31/dir_02_30/.ls`
- changing `ParityTask`
- changing runtime selection logic
- adding adapter-side semantic repair

Allowed fix surfaces:

- prompt wording
- prompt examples
- prompt-visible feedback shaping
- other non-kernel benchmark-surface changes that do not infer hidden semantics

## 7. Open Repair Questions

- Does the current prompt under-specify the degenerate `.ls` case where one terminal discovered `FILE` becomes the only remaining `todo` item?
- Is the current example set overweight on cases where `next_path` remains an older pending item, and underweight on cases where a current `.ls` discovery must become the immediate next head?
- Can the fix remain short and general enough to preserve the anti-hardcoding rule while improving this archetype?
