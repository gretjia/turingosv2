# TuringOSv2 Parity Protocol Law Four-Layer Solidification

Status: Active
Date: 2026-03-10 UTC
Scope: Compressed good-work pattern for evidence-led, non-kernel protocol-law repair on the parity pressure-test lane

## 1. Why This Pattern Exists

The `rung 10` first fail on `case_000004` produced a clean, reusable lesson:

- a compressed failure law is not safely "fixed" by one prompt edit alone
- it should be solidified across multiple layers so the law becomes durable, auditable, and regression-resistant

This pattern records the preferred Groundhog method for that solidification.

## 2. The Four Layers

### 2.1 Whitebox Truth Layer

Authority lives first in the task's whitebox transition law:

- `turingos/tasks/parity.py`

For the relevant `.ls` frontier law, `ParityTask.expected_transition(...)` encodes:

- recompute `todo` as:
  - `remaining_old_todo_without_current_path + discovered_md_files + discovered_child_ls_paths`
- if recomputed `todo` is non-empty, `next_path` must be `todo[0]`
- only when recomputed `todo` is empty may control return to `parity.md`

Then `ParityTask.verify_proposal(...)` compresses drift into explicit failure fingerprints:

- `wrong_path`
- `wrong_register`
- `wrong_write_content`
- and other exact mismatch classes

### 2.2 Blackbox Contract Layer

The same law must then be translated into the blackbox proposal contract:

- `harnesses/mission002_py/mission002_prompting.py`

This is not case-specific wording. It is the public protocol rendered in blackbox-usable form:

- `.ls` recomputed-todo law
- FILE-before-DIR/.ls discovery ordering inside the current batch
- no jump back to `parity.md` while recomputed `todo` is non-empty
- no early phase blending during `apply_pending`

### 2.3 Regression Lock Layer

The law must then be locked so future prompt edits cannot silently weaken it:

- `tests/test_mission002_prompting.py`

Required lock points:

- invariants are asserted directly
- worked examples are asserted directly
- prompt-version movement is visible and test-gated

### 2.4 Evidence Memory Layer

The reason the law exists must remain preserved as authority-bearing evidence:

- `handover/ops/TURINGOSV2_RUNG10_CASE4_EVIDENCE_PACKET.md`

That packet stores:

- the observed `Q_t`
- the relevant TAPE slice
- the whitebox expected transition
- the compressed failure law
- the anti-hardcoding boundary

## 3. Why This Counts As Good Work

This pattern is preferred because it:

- stays outside the kernel
- does not alter topology
- does not hard-code a case id or path
- is grounded in compressed historical evidence
- makes the repair auditable and reversible
- turns one failure into a durable protocol-law improvement instead of a one-off patch

## 4. Reuse Rule

When a new parity pressure-test failure compresses into a clean protocol law, prefer this order:

1. identify the whitebox truth already implied by the task
2. translate that law into the blackbox contract
3. lock it with regression tests
4. preserve the evidence packet that justifies the law

If a proposed repair cannot be expressed across these four layers, it should not be described as a solidified protocol-law repair.

## 5. Current Limitation

This pattern is currently parity-scoped.

It is not yet a task-agnostic core module for all future TuringOS tasks. At this stage it is a reusable repair method and a parity-lane protocol-law pattern, not yet a generalized cross-task kernel abstraction.
