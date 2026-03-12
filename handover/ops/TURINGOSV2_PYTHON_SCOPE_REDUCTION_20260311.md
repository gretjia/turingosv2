# TuringOSv2 Python Scope Reduction

Status: Active migration note
Date: 2026-03-11 UTC
Scope:
- bounded removal of non-essential Python from the repository core while preserving the constitution-aligned Rust kernel direction

## 1. Executive Decision

This slice removes only Python surfaces that are no longer theorem-bearing, benchmark-critical, or required as migration debt.

Removed now:

- `turingos/oracle.py`
- `turingos/tools/terminal.py`
- `turingos/tools/__init__.py`
- `tests/test_oracle.py`
- `tests/test_terminal_tool.py`

Reduced in place:

- `turingos/cli.py`
  - removed the `oracle-demo` command
  - kept `parity-demo`
  - kept `benchmark-mission002`, but its implementation now lives outside the `turingos` package in `harnesses/mission002_py/`

Moved out of the `turingos` core package:

- `turingos/benchmarks/mission002.py` -> `harnesses/mission002_py/mission002.py`
- `turingos/benchmarks/parity_cases.py` -> `harnesses/mission002_py/parity_cases.py`
- `turingos/benchmarks/artifacts.py` -> `harnesses/mission002_py/artifacts.py`
- `turingos/agents/llama_openai.py` -> `harnesses/mission002_py/llama_openai.py`
- `turingos/agents/mission002_prompting.py` -> `harnesses/mission002_py/mission002_prompting.py`

## 2. Why This Slice Is Safe

These removed files were not part of:

- the Rust theorem-bearing core
- the sealed constitutional stages
- the active Mission 002 benchmark path
- the current parity reference truth path

They only served:

- a tiny Ralph-loop demo
- a whitelisted terminal tool demo
- the tests for those demos

## 3. Keep-For-Now Python

The following Python surfaces remain because they still carry one of:

- current parity task truth
- current Python reference runtime behavior
- active provider/bounded benchmark harness behavior

Keep for now:

- `turingos/runtime.py`
- `turingos/models.py`
- `turingos/fs.py`
- `turingos/ledger.py`
- `turingos/masking.py`
- `turingos/scheduler.py`
- `turingos/signals.py`
- `turingos/tasks/base.py`
- `turingos/tasks/parity.py`
- `turingos/agents/parity.py`
- `turingos/cli.py`
- `harnesses/mission002_py/mission002.py`
- `harnesses/mission002_py/parity_cases.py`
- `harnesses/mission002_py/artifacts.py`
- `harnesses/mission002_py/llama_openai.py`
- `harnesses/mission002_py/mission002_prompting.py`

## 4. Next Reduction Principle

Do not delete Python merely because it is Python.

Delete or demote Python only when one of the following is true:

1. the file is pure demo or shell glue and carries no constitutional truth
2. the same theorem-bearing responsibility already exists in Rust
3. the file is external harness logic and can move outside the repository core without taking white-box authority with it

## 6. New Rust Parity-Law Slice

A new trusted Rust parity task-law crate now exists:

- `crates/turingos-parity/`

This changes one important migration fact:

- Python `turingos/tasks/parity.py` is still the live reference path for the current Python runtime
- but it is no longer the only repository embodiment of parity white-box truth
- the Rust side now has an explicit `ParityTask` law surface outside the generic kernel physics

This is intentionally not a full runtime migration yet. It is the first slice that reduces
theorem-bearing Python without collapsing the kernel/task-law boundary.

## 5. Validation

Commands run:

- `cd /home/zephryj/projects/turingosv2 && /home/zephryj/.cargo/bin/cargo test -q`
- `cd /home/zephryj/projects/turingosv2 && ./.venv/bin/python -m pytest -q`

Result:

- both Rust and Python validation remained green after this slice
- `python -m turingos.cli benchmark-mission002 --help` still resolved and executed through the new external harness path
- setuptools package discovery now includes `harnesses*`, so installed distributions still contain the external Mission 002 harness package
- `turingos/cli.py` now lazy-imports the Mission 002 harness registration, so unrelated commands like `parity-demo` do not hard-fail if the external harness package is absent
