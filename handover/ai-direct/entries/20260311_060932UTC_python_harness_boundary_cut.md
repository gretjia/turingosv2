# Handover Entry

## Summary

- Date: 2026-03-11 UTC
- Slice: Python scope reduction, external harness boundary cut

## What Changed

- Moved the active Mission 002 benchmark/provider Python out of the `turingos` package and into `harnesses/mission002_py/`.
- Updated `turingos/cli.py` so `benchmark-mission002` now resolves through the external harness package instead of `turingos.benchmarks.*`.
- Updated `pyproject.toml` so installed distributions now include `harnesses*`.
- Changed `turingos/cli.py` to lazy-import the Mission 002 harness registration, so `parity-demo` does not hard-fail if the external harness package is missing.
- Retained theorem-bearing Python in place:
  - `turingos/runtime.py`
  - `turingos/tasks/parity.py`
  - `turingos/agents/parity.py`
- Kept benchmark functionality intact while making the file tree better reflect the intended architecture:
  - `turingos` = core package
  - `harnesses/mission002_py` = external Python orchestration/provider surface

## Validation

- `cd /home/zephryj/projects/turingosv2 && /home/zephryj/.cargo/bin/cargo test -q`
- `cd /home/zephryj/projects/turingosv2 && ./.venv/bin/python -m pytest -q`
- `cd /home/zephryj/projects/turingosv2 && ./.venv/bin/python -m turingos.cli parity-demo --help`
- `cd /home/zephryj/projects/turingosv2 && ./.venv/bin/python -m turingos.cli benchmark-mission002 --help`

## Why It Matters

- The benchmark/provider Python path still exists, but it no longer masquerades as part of the theorem-bearing `turingos` core package.
- This creates a cleaner next migration boundary: parity/task/runtime truth can now move to Rust without benchmark/provider harness code being entangled with the core package namespace.
