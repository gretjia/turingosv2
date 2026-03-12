# Mission Groundhog Phase 15 Spec Draft

Status: Completed and frozen after Phase 15 PASS
Mission: Mission Groundhog
Phase: 15
Date: 2026-03-10 UTC

## 0. Phase State

- Phase 15 is now closed and frozen.
- Phase 14 is closed and frozen by `handover/ops/MISSION_GROUNDHOG_PHASE_014_CLOSEOUT.md`.
- Phase 15 is closed by `handover/ops/MISSION_GROUNDHOG_PHASE_015_CLOSEOUT.md`.
- Phase 15 may not claim completion until it has:
  - opened only the bounded detached current poll-packet surface below
  - passed the required audit ring
  - written `handover/ops/MISSION_GROUNDHOG_PHASE_015_CLOSEOUT.md`
  - written one append-only entry matching `handover/ai-direct/entries/*_groundhog_phase015_closeout.md`

## 1. Objective

- Open the first compact detached current poll packet above `RunBasis` and optional `RunDelta`.
- Let future polling-agent consumers receive one owned packet per poll without holding Rust borrows to `RunExport`.
- Keep the packet policy-free, observer-side, and explicitly smaller in authority than `RunExport`.

## 2. Canonical Authority

- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
- `handover/ops/ACTIVE_MISSION_CHARTER.md`
- `handover/ops/MISSION_GROUNDHOG_PROJECT_SPEC.md`
- `handover/ops/MISSION_GROUNDHOG_EXECUTION_LOOP.md`
- `handover/ops/MISSION_GROUNDHOG_IMPLEMENTATION_GUARDRAILS.md`
- `handover/ops/GEMINI_GROUNDHOG_OPERATING_POLICY.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_013_CLOSEOUT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_014_CLOSEOUT.md`

## 3. In Scope Files

Writable in this phase:

- `crates/turingos-observe/src/lib.rs`
- `crates/turingos-observe/src/run_basis.rs`
- `crates/turingos-observe/src/run_delta.rs`
- `crates/turingos-observe/src/run_sample.rs`
- `crates/turingos-observe/src/run_pulse.rs`
- `handover/ops/MISSION_GROUNDHOG_PHASE_015_SPEC_DRAFT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_015_CLOSEOUT.md`
- one new append-only entry matching `handover/ai-direct/entries/*_groundhog_phase015_closeout.md`
- `handover/ai-direct/LATEST.md`
- `handover/BOARD.md`

Read-only context:

- `crates/turingos-observe/src/run_export.rs`
- `crates/turingos-observe/src/run_summary.rs`
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

- Phase 15 may add one compact detached current poll packet above `RunBasis` and optional `RunDelta`.
- The phase may:
  - package current `RunBasis` and optional `RunDelta` into one owned poll packet
  - expose faithful derivation from `RunExport`
  - expose faithful derivation from current summary plus prior basis
  - reduce future polling-agent wrapper logic for detached owned polling surfaces
- The phase must not:
  - move observer truth ownership out of `RunExport`
  - turn the new packet into a replacement for `RunExport`
  - smuggle timestamps, retries, cadence, or health policy into the packet
  - add scheduler, provider, CLI, serialization, or storage logic
  - feed packet data back into the kernel

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

- one compact detached current poll packet in `turingos-observe`
- one faithful derivation path from current observer truth into that packet
- one faithful derivation path using prior `RunBasis`
- tests proving:
  - the packet remains detached and policy-free
  - packet-derived comparison facts equal borrowed sample-derived comparison facts
  - the packet does not become a second observer-side truth packet
  - the packet does not become daemon or retry policy
- one Phase 15 closeout packet

## 9. Acceptance Criteria

- the packet surface lives in `turingos-observe`
- `RunExport` remains the sole observer-side owned truth packet
- `RunBasis` remains the minimal retained comparison token
- the new packet is detached, compact, derived, and explicitly non-authoritative
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

- A detached poll packet can accidentally widen into a second owned truth packet if it starts copying heavyweight `RunExport` data instead of staying basis-sized.
- Convenience pressure may try to add timestamps or operational health metadata because this stage targets polling-agent consumers; that remains forbidden.
- If the detached packet API is not clearly better than `RunExport` plus `RunBasis`, future consumers may ignore it and rebuild wrappers anyway.
