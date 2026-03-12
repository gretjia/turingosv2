# Mission Groundhog Phase 10 Spec Draft

Status: Completed and frozen after Phase 10 PASS
Mission: Mission Groundhog
Phase: 10
Date: 2026-03-10 UTC

## 0. Phase State

- Phase 10 is now closed and frozen by `handover/ops/MISSION_GROUNDHOG_PHASE_010_CLOSEOUT.md`.
- Phase 9 is closed and frozen by `handover/ops/MISSION_GROUNDHOG_PHASE_009_CLOSEOUT.md`.
- Phase 10 may not claim completion until it has:
  - opened only the bounded detached run-export surface below
  - passed the required audit ring
  - written `handover/ops/MISSION_GROUNDHOG_PHASE_010_CLOSEOUT.md`
  - written one append-only entry matching `handover/ai-direct/entries/*_groundhog_phase010_closeout.md`

## 1. Objective

- Open the first detached read-only run-export packet for downstream consumers that cannot hold borrowed Rust lifetimes.
- Preserve `RunOutcome` and borrowed `RunReport` as the only kernel-side truth and in-kernel read surface.
- Place any owned export packet outside kernel theorem authority so later CLI, benchmark, and polling-agent layers can consume it without reopening commit authority.

## 2. Canonical Authority

- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
- `handover/ops/ACTIVE_MISSION_CHARTER.md`
- `handover/ops/MISSION_GROUNDHOG_PROJECT_SPEC.md`
- `handover/ops/MISSION_GROUNDHOG_EXECUTION_LOOP.md`
- `handover/ops/MISSION_GROUNDHOG_IMPLEMENTATION_GUARDRAILS.md`
- `handover/ops/GEMINI_GROUNDHOG_OPERATING_POLICY.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_001_ARCHITECTURE_PACKET.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_008_CLOSEOUT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_009_CLOSEOUT.md`

## 3. In Scope Files

Writable in this phase:

- `Cargo.toml`
- `Cargo.lock`
- `crates/turingos-observe/Cargo.toml`
- `crates/turingos-observe/src/lib.rs`
- `crates/turingos-observe/src/run_export.rs`
- `crates/turingos-kernel/src/report.rs`
- `handover/ops/MISSION_GROUNDHOG_PHASE_010_SPEC_DRAFT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_010_CLOSEOUT.md`
- one new append-only entry matching `handover/ai-direct/entries/*_groundhog_phase010_closeout.md`
- `handover/ai-direct/LATEST.md`
- `handover/BOARD.md`

Read-only context:

- `crates/turingos-core/`
- `crates/turingos-adapter/`
- `crates/turingos-task/`
- `crates/turingos-kernel/src/run.rs`
- `tests/`
- `benchmarks/`
- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`

## 4. Out of Scope Files

- `turingos/`
- any real model-provider integration
- any network client or server integration
- any scheduler module
- any CLI crate or CLI implementation
- any serialization or storage backend
- any benchmark migration code
- any constitution file

## 5. Surface Touch Matrix

- kernel: narrow read-side touch only
- pluggable adapters: no
- tools: no
- CLI: no
- validation: yes

## 6. Kernel/Plugin Boundary Implications

- Phase 10 may create one detached owned export packet outside the kernel crate.
- The phase may:
  - define one canonical detached run-export shape
  - convert from borrowed kernel report truth into owned observer-side data
  - normalize consumer-facing fields for later read-side layers
- The phase must not:
  - move theorem authority out of `RunOutcome`
  - feed detached packets back into the kernel
  - add scheduler policy
  - add retry policy
  - add provider orchestration
  - add daemon or watchdog logic

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

- one detached owned run-export packet outside kernel theorem authority
- one one-way conversion path from borrowed run truth into that export packet
- tests proving:
  - export preserves committed history exactly
  - export preserves stop classification exactly
  - export preserves terminal snapshot facts exactly
  - export construction does not become a new kernel truth source
- one Phase 10 closeout packet

## 9. Acceptance Criteria

- the detached export packet lives outside kernel theorem authority
- `RunOutcome` remains the sole owner of committed run truth
- borrowed `RunReport` remains the sole in-kernel read surface
- no CLI, scheduler, provider, serialization backend, or daemon logic is introduced
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

- A detached owned export packet can accidentally become a second truth source if it begins to reinterpret kernel facts rather than snapshot them.
- Moving too much consumer convenience into the observer crate can smuggle CLI or benchmark policy into what should remain a pure export layer.
- The next phase must keep detached export and live polling/orchestration separate; otherwise it will reopen the watchdog boundary the user explicitly rejected.
