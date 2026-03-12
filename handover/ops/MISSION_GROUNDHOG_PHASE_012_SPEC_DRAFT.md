# Mission Groundhog Phase 12 Spec Draft

Status: Completed and frozen after Phase 12 PASS
Mission: Mission Groundhog
Phase: 12
Date: 2026-03-10 UTC

## 0. Phase State

- Phase 12 is now closed and frozen by `handover/ops/MISSION_GROUNDHOG_PHASE_012_CLOSEOUT.md`.
- Phase 11 is closed and frozen by `handover/ops/MISSION_GROUNDHOG_PHASE_011_CLOSEOUT.md`.
- Phase 12 may not claim completion until it has:
  - opened only the bounded observer-side progress-delta surface below
  - passed the required audit ring
  - written `handover/ops/MISSION_GROUNDHOG_PHASE_012_CLOSEOUT.md`
  - written one append-only entry matching `handover/ai-direct/entries/*_groundhog_phase012_closeout.md`

## 1. Objective

- Open the first observer-side progress-delta surface above `RunSummary`.
- Let future polling-agent consumers compare two successive summary states without inventing ad hoc diff semantics.
- Preserve `RunSummary` as the compact current-state read surface while keeping any delta layer fully derived and observer-side.

## 2. Canonical Authority

- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
- `handover/ops/ACTIVE_MISSION_CHARTER.md`
- `handover/ops/MISSION_GROUNDHOG_PROJECT_SPEC.md`
- `handover/ops/MISSION_GROUNDHOG_EXECUTION_LOOP.md`
- `handover/ops/MISSION_GROUNDHOG_IMPLEMENTATION_GUARDRAILS.md`
- `handover/ops/GEMINI_GROUNDHOG_OPERATING_POLICY.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_001_ARCHITECTURE_PACKET.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_010_CLOSEOUT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_011_CLOSEOUT.md`

## 3. In Scope Files

Writable in this phase:

- `crates/turingos-observe/src/lib.rs`
- `crates/turingos-observe/src/run_summary.rs`
- `crates/turingos-observe/src/run_delta.rs`
- `handover/ops/MISSION_GROUNDHOG_PHASE_012_SPEC_DRAFT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_012_CLOSEOUT.md`
- one new append-only entry matching `handover/ai-direct/entries/*_groundhog_phase012_closeout.md`
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

- Phase 12 may add one observer-side delta layer above `RunSummary`.
- The phase may:
  - compare two successive summaries
  - expose progress or no-progress facts for polling-agent consumers
  - expose bounded change facts such as attempted-step advance, committed-count advance, terminal-step advance, stop-kind transition, or halt-status transition
- The phase must not:
  - move truth ownership out of `RunExport`
  - create a second owned summary packet
  - create a polling loop, watcher, daemon, or supervisor
  - feed delta data back into the kernel
  - add scheduler policy
  - add retry policy
  - add provider orchestration

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

- one observer-side progress-delta surface above `RunSummary`
- one faithful comparison path from prior summary to current summary
- tests proving:
  - no-progress detection is exact
  - attempted-step and committed-count deltas are exact
  - terminal-step advance is exact
  - stop-kind and halt-status transition facts are exact
  - delta construction does not become a second owned truth packet or a hidden polling policy layer
- one Phase 12 closeout packet

## 9. Acceptance Criteria

- the delta surface lives in `turingos-observe`
- `RunExport` remains the sole observer-side owned truth packet
- `RunSummary` remains the compact current-state read surface
- the delta layer is read-only, derived, and observer-side
- no kernel authority, serialization, provider, CLI, scheduler, polling loop, or daemon logic is introduced
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

- A delta layer can accidentally smuggle polling policy into what should remain a pure comparison surface.
- If the delta API grows too wide, observer ergonomics can regress into wrapper sprawl.
- Future consumers may try to cache derived delta state as owned truth; that must remain forbidden.
