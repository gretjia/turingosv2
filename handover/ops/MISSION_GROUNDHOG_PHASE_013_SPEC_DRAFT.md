# Mission Groundhog Phase 13 Spec Draft

Status: Completed and frozen after Phase 13 PASS
Mission: Mission Groundhog
Phase: 13
Date: 2026-03-10 UTC

## 0. Phase State

- Phase 13 is now closed and frozen.
- Phase 12 is closed and frozen by `handover/ops/MISSION_GROUNDHOG_PHASE_012_CLOSEOUT.md`.
- Phase 13 is closed by `handover/ops/MISSION_GROUNDHOG_PHASE_013_CLOSEOUT.md`.
- Phase 13 may not claim completion until it has:
  - opened only the bounded observer-side poll-sample surface below
  - passed the required audit ring
  - written `handover/ops/MISSION_GROUNDHOG_PHASE_013_CLOSEOUT.md`
  - written one append-only entry matching `handover/ai-direct/entries/*_groundhog_phase013_closeout.md`

## 1. Objective

- Open the first canonical observer-side poll-sample surface above current `RunSummary` and optional prior-state `RunDelta`.
- Let future polling-agent consumers read one compact packet per poll instead of assembling summary and delta logic by hand.
- Keep the sample layer read-only, observer-side, and free of polling cadence or daemon policy.

## 2. Canonical Authority

- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
- `handover/ops/ACTIVE_MISSION_CHARTER.md`
- `handover/ops/MISSION_GROUNDHOG_PROJECT_SPEC.md`
- `handover/ops/MISSION_GROUNDHOG_EXECUTION_LOOP.md`
- `handover/ops/MISSION_GROUNDHOG_IMPLEMENTATION_GUARDRAILS.md`
- `handover/ops/GEMINI_GROUNDHOG_OPERATING_POLICY.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_011_CLOSEOUT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_012_CLOSEOUT.md`

## 3. In Scope Files

Writable in this phase:

- `crates/turingos-observe/src/lib.rs`
- `crates/turingos-observe/src/run_summary.rs`
- `crates/turingos-observe/src/run_delta.rs`
- `crates/turingos-observe/src/run_sample.rs`
- `handover/ops/MISSION_GROUNDHOG_PHASE_013_SPEC_DRAFT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_013_CLOSEOUT.md`
- one new append-only entry matching `handover/ai-direct/entries/*_groundhog_phase013_closeout.md`
- `handover/ai-direct/LATEST.md`
- `handover/BOARD.md`

Read-only context:

- `crates/turingos-observe/src/run_export.rs`
- `crates/turingos-kernel/src/report.rs`
- `crates/turingos-kernel/src/run.rs`
- `crates/turingos-core/`
- `tests/`
- `benchmarks/`
- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`

## 4. Out of Scope Files

- `turingos/`
- any kernel theorem-bearing type
- any real model-provider integration
- any network client or server integration
- any scheduler module
- any CLI crate or CLI implementation
- any serialization or storage backend
- any benchmark migration code
- any constitution file

## 5. Surface Touch Matrix

- kernel: no
- observer crate: yes
- pluggable adapters: no
- tools: no
- CLI: no
- validation: yes

## 6. Kernel/Plugin Boundary Implications

- Phase 13 may add one observer-side sample layer above `RunSummary` and optional `RunDelta`.
- The phase may:
  - package the current summary and optional change facts into one bounded read packet
  - expose convenience accessors over that packet
  - reduce future polling-agent wrapper logic
- The phase must not:
  - move truth ownership out of `RunExport`
  - create a polling loop, watcher, daemon, or supervisor
  - create scheduler or retry policy
  - feed sample data back into the kernel
  - add provider orchestration
  - add serialization or network logic

## 7. Required Roles

Execution lanes:

- `turingos_plan`
- `groundhog_math_constitution_custodian`
- `groundhog_formal_methods_auditor`
- `groundhog_turing_machine_theorist`
- `turingos_rust_systems_architect`
- `turingos_rust_coder`
- `turingos_git_historian`

Phase-close auditors:

- `turingos_quality_auditor`
- `turing_agi_architect_auditor`

Phase-close recursive auditors:

- `groundhog_recursive_math_auditor`
- `turingos_recursive_software_auditor`

Advisory simplification lane:

- `turingos_karpathy_editor`

## 8. Deliverables

- one observer-side poll-sample surface above `RunSummary` and optional `RunDelta`
- one faithful construction path for:
  - current-only sample
  - current-plus-delta sample
- tests proving:
  - the current summary remains authoritative inside the sample
  - optional delta is absent when no prior sample basis exists
  - optional delta is exact when prior summary is supplied
  - sample construction does not become polling policy or a second owned truth packet
- one Phase 13 closeout packet

## 9. Acceptance Criteria

- the sample surface lives in `turingos-observe`
- `RunExport` remains the sole observer-side owned truth packet
- `RunSummary` remains the compact current-state read surface
- `RunDelta` remains the canonical comparison layer
- the sample layer is read-only, derived, and observer-side
- no kernel authority, polling loop, daemon, provider, CLI, scheduler, or serialization logic is introduced
- the resulting API remains compact and legible

## 10. Required Audits

- constitution audit
- formal methods audit
- Turing-machine semantics audit
- Rust systems architecture audit
- git/provenance audit
- quality audit
- recursive audit pair
- AGI direction audit

## 11. Required Tests And Validation

- `cargo fmt --check`
- `cargo test`
- `./.venv/bin/python -m pytest -q`

## 12. Constraints

- no watchdog or supervisor daemon in any language
- runtime observation must continue to be handled by polling agents rather than background daemon logic
- no real provider or network integration
- no CLI embodiment
- no scheduler embodiment
- no benchmark migration embodiment
- no constitution edits

## 13. Open Risks

- A sample layer can regress into wrapper sprawl if it re-exports every low-level field instead of staying compact.
- Convenience pressure may try to smuggle polling cadence or stateful retry policy into the sample constructor; that remains forbidden.
- If the optional-delta path is overcomplicated, future consumers may bypass it and rebuild their own wrappers anyway.
