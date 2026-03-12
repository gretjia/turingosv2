# Handover Entry

## Summary

- Mission: Mission Groundhog
- Phase: 14
- Date: 2026-03-10 UTC
- State: Initial bounded comparison-basis slice landed

## What Changed

- Added `crates/turingos-observe/src/run_basis.rs` to define compact detached `RunBasis`.
- `RunBasis` now captures only the exact current-state facts needed to reproduce `RunDelta`:
  - attempted step count
  - committed step count
  - terminal step
  - stop class
  - halt status
- Added derivation paths:
  - `RunSummary::basis()`
  - `RunExport::basis()`
- Added basis-aware comparison path:
  - `RunDelta::between_basis(...)`
  - `RunExport::sample_after_basis(...)`
- Added equivalence tests proving basis-derived comparison matches prior-summary comparison.

## Evidence

- `crates/turingos-observe/src/run_basis.rs`
- `crates/turingos-observe/src/run_delta.rs`
- `crates/turingos-observe/src/run_sample.rs`
- `/home/zephryj/.cargo/bin/cargo fmt --check`
- `/home/zephryj/.cargo/bin/cargo test`
- `./.venv/bin/python -m pytest -q`

## Open Risks

- Phase 14 audit ring has not been fully run yet.
- The basis must remain minimal and must not drift into an owned replacement for `RunExport` or a hidden polling-policy packet.

## Next Step

- Run the Phase 14 audit ring over the new comparison-basis slice, fix any bounded findings, and decide whether the stage can close.
