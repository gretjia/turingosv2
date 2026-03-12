# Mission Groundhog Phase 11 Spec Draft

Status: Completed and frozen after Phase 11 PASS
Mission: Mission Groundhog
Phase: 11
Date: 2026-03-10 UTC

## 0. Phase State

- Phase 11 is now closed and frozen by `handover/ops/MISSION_GROUNDHOG_PHASE_011_CLOSEOUT.md`.
- Phase 10 is closed and frozen by `handover/ops/MISSION_GROUNDHOG_PHASE_010_CLOSEOUT.md`.
- Phase 11 may not claim completion until it has:
  - opened only the bounded compact observer-side summary surface below
  - passed the required audit ring
  - written `handover/ops/MISSION_GROUNDHOG_PHASE_011_CLOSEOUT.md`
  - written one append-only entry matching `handover/ai-direct/entries/*_groundhog_phase011_closeout.md`

## 1. Objective

- Open the first compact observer-side summary surface above detached `RunExport`.
- Let future polling-agent, CLI, and benchmark consumers ask compact read questions without parsing the whole export packet.
- Preserve `RunExport` as the sole observer-side owned truth packet while keeping the summary surface read-only and derived.

## 2. Canonical Authority

- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
- `handover/ops/ACTIVE_MISSION_CHARTER.md`
- `handover/ops/MISSION_GROUNDHOG_PROJECT_SPEC.md`
- `handover/ops/MISSION_GROUNDHOG_EXECUTION_LOOP.md`
- `handover/ops/MISSION_GROUNDHOG_IMPLEMENTATION_GUARDRAILS.md`
- `handover/ops/GEMINI_GROUNDHOG_OPERATING_POLICY.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_001_ARCHITECTURE_PACKET.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_009_CLOSEOUT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_010_CLOSEOUT.md`

## 3. In Scope Files

Writable in this phase:

- `crates/turingos-observe/src/lib.rs`
- `crates/turingos-observe/src/run_export.rs`
- `crates/turingos-observe/src/run_summary.rs`
- `handover/ops/MISSION_GROUNDHOG_PHASE_011_SPEC_DRAFT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_011_CLOSEOUT.md`
- one new append-only entry matching `handover/ai-direct/entries/*_groundhog_phase011_closeout.md`
- `handover/ai-direct/LATEST.md`
- `handover/BOARD.md`

Read-only context:

- `crates/turingos-kernel/src/report.rs`
- `crates/turingos-kernel/src/run.rs`
- `crates/turingos-core/`
- `crates/turingos-adapter/`
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

- Phase 11 may add one compact read-only summary layer above `RunExport`.
- The phase may:
  - define a compact summary shape or borrowed summary projection for observer-side consumers
  - expose faithful stop classification, success status, and committed-history counts from `RunExport`
  - reduce consumer need to inspect full stop payloads when they only need summary facts
- The phase must not:
  - move truth ownership out of `RunExport`
  - create a second observer-side owned truth packet
  - feed summary data back into the kernel
  - add scheduler policy
  - add retry policy
  - add provider orchestration
  - add daemon, watchdog, or supervisor logic

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

- one compact observer-side summary surface above detached `RunExport`
- one faithful read path from `RunExport` into that summary surface
- tests proving:
  - summary facts remain faithful to `RunExport`
  - success or halt classification is not reinterpreted into new theorem semantics
  - committed-history counts and attempted-step counts remain exact
  - summary construction does not become a second owned export truth packet
- one Phase 11 closeout packet

## 9. Acceptance Criteria

- the compact summary surface lives in `turingos-observe`
- `RunExport` remains the sole observer-side owned truth packet
- the summary surface is read-only and derived
- no kernel authority, serialization, provider, CLI, scheduler, or daemon logic is introduced
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

- A compact summary layer can accidentally become a second owned truth packet if it duplicates rather than borrows or cleanly derives from `RunExport`.
- Convenience pressure from future polling-agent or CLI consumers can smuggle policy into what should remain a pure read-side observer layer.
- If stop or success classification is over-compressed, the summary surface could silently invent semantics not present in `RunExport`.
