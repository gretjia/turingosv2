# TuringOSv2 Rust Slimming Slice

Date: 2026-03-11 UTC
Status: Applied
Scope: bounded crate-topology slimming under the sealed constitution

## Objective

- Reduce theorem-bearing Rust shell-crate noise without weakening the constitutional stages.
- Keep the WHITEBOX path explicit:
  - `WORLD_t`
  - `rtool`
  - `TaskContract`
  - `predicates`
  - `wtool`
  - `abort`

## Constitutional Basis

- The sealed source remains `bible/GROUNDHOG_SEALED_CONSTITUTION.md`.
- This slice follows the derived `WHITEBOX / BLACKBOX` interpretation in:
  - `handover/ops/TURINGOSV2_CONSTITUTION_AI_FRIENDLY_REWRITE_20260311.md`

## What Changed

- Moved the WHITEBOX task admissibility trait into the kernel crate:
  - from the deleted `crates/turingos-task/src/contract.rs`
  - to `crates/turingos-kernel/src/task.rs`
- Re-exported `TaskContract` from `crates/turingos-kernel/src/lib.rs`.
- Rewired kernel and test imports to use `turingos_kernel::TaskContract`.
- Removed the lightweight `turingos-task` workspace member from `Cargo.toml`.
- Deleted the unused `turingos-task` replay scaffolding instead of promoting it into the trusted kernel.
- Relocated the shared parity golden replay support from the old crate-like path to the repo-level dev-only support path:
  - `tests/support/parity_golden.rs`

## Why This Is Safe

- The move does not widen the BLACKBOX seam.
- `TaskContract` remains a WHITEBOX admissibility law over:
  - `WorldState<QState>`
  - `IntentEnvelope<QState>`
- Replay helpers were not moved into the kernel; they were removed because they were no longer used anywhere in the active tree.
- The shared parity golden scaffold is now repo-level test support, not a fake crate layer and not a private test directory of another crate.
- The explicit constitutional stages remain visible in separate files:
  - `read.rs`
  - `task.rs`
  - `predicate_gate.rs`
  - `commit.rs`
  - `run.rs`

## AgentOS Review

- Unix line: `FIX`
  - collapse only the task-law shell, do not move replay into kernel
- Karpathy line: `FIX`
  - remove the shell crate cleanly; preserve explicit stage seams
- Math line: `FIX`
  - keep task truth in WHITEBOX core; do not smuggle replay ownership into kernel

Resolution:

- The implementation satisfies the review guidance:
  - `TaskContract` moved into kernel
  - replay was not promoted into kernel
  - shell-crate noise was removed

## Validation

- `cd /home/zephryj/projects/turingosv2 && /home/zephryj/.cargo/bin/cargo test -q`
- `cd /home/zephryj/projects/turingosv2 && ./.venv/bin/python -m pytest -q`

## Remaining Work

- Continue Rust-first slimming without collapsing constitutional visibility.
- The next safe slimming candidates should be evaluated against the same rule:
  - remove shell layers
  - do not merge away WHITEBOX stage clarity
