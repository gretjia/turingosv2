# Handover Entry

## Summary

- Date: 2026-03-10 UTC
- Topic: diagnose the long wall-clock time of the official `rung 3` live run

## Finding

- The current long runtime is not primarily explained by whitebox kernel complexity.
- The main cost is the dual-lane black-box inference path on the Mac-hosted `llama.cpp` services.

## Evidence

- The live benchmark Python process remains active:
  - `./.venv/bin/python -m turingos.cli benchmark-mission002 ...`
- Local socket evidence shows the benchmark process maintaining an established connection to the forwarded `27B` endpoint on `127.0.0.1:18082`.
- Mac-side `llama.cpp` logs for the `27B` lane show requests completing with `200` and timing around:
  - prompt eval: about `4.2s`
  - decode/eval: about `41-44s`
  - total: about `45-48s`
- Mac-side `llama.cpp` logs for the `9B` lane show requests completing faster, around:
  - total: about `13-14s`
- The official headline mode uses both lanes repeatedly at step granularity.

## Interpretation

- A parity case with roughly `20-30` committed steps can legitimately consume tens of minutes under the current `27B + 9B` dual-lane topology.
- This means:
  - long wall-clock duration by itself is not enough to call the run abnormal
  - the first suspect should be black-box inference cost or transport blocking, not whitebox state corruption
- The whitebox evidence currently supports this reading:
  - `case_000001` completed cleanly
  - `case_000002` continues to advance through valid committed states
  - no semantic failure artifact has yet been emitted for the current official `rung 3` run

## Operational Rule

- For future live headline runs, distinguish clearly between:
  - semantic failure
  - infra blocker
  - expected high wall-clock cost from dual-lane inference
- Do not classify a run as hung merely because a single case takes many minutes unless the evidence shows no state advance and no active model-side work.
