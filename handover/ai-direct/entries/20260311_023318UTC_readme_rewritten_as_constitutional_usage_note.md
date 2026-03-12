# Handover Entry

## Summary

- Mission: Rewrite the `turingos` explanation document
- Date: 2026-03-11 UTC
- Owner: Codex Commander

## What Changed

- Rewrote `README.md` as a constitution-aligned usage note for `turingos`.
- Fixed the project identity:
  - `turingos` now explicitly refers to this `turingosv2` project
- Reframed the product around the sealed constitutional role:
  - white-box attention kernel
  - worldline / commit kernel
  - not a benchmark harness and not a task-script collection
- Wrote down the two ideal application scenes:
  - one agent holding attention over an extremely long project
  - many agents advancing bounded work without drifting or directly contaminating world state
- Made the outer-boundary rule explicit in the README:
  - parity checks
  - benchmark runners
  - real-world task drivers
  are external disposable harnesses and not part of the theorem-bearing repository core.

## Artifact

- `README.md`

## Next Step

- Keep using this rewritten README as the canonical explanation while the Rust-only migration continues.
