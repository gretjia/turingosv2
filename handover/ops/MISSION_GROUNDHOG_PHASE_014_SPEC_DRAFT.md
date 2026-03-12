# Mission Groundhog Phase 14 Spec Draft

Status: Completed and frozen after Phase 14 PASS
Mission: Mission Groundhog
Phase: 14
Date: 2026-03-10 UTC

## 0. Phase State

- Phase 14 is now closed and frozen.
- Phase 13 is closed and frozen by `handover/ops/MISSION_GROUNDHOG_PHASE_013_CLOSEOUT.md`.
- Phase 14 is closed by `handover/ops/MISSION_GROUNDHOG_PHASE_014_CLOSEOUT.md`.
- Phase 14 may not claim completion until it has:
  - opened only the bounded detached comparison-basis surface below
  - passed the required audit ring
  - written `handover/ops/MISSION_GROUNDHOG_PHASE_014_CLOSEOUT.md`
  - written one append-only entry matching `handover/ai-direct/entries/*_groundhog_phase014_closeout.md`

## 1. Objective

- Open the first compact detached comparison-basis surface above `RunSummary` and below future polling-agent consumers.
- Let future polling-agent consumers retain minimal owned prior-state comparison facts between polls without retaining the full previous `RunExport`.
- Keep the basis policy-free, observer-side, and strictly smaller in authority than `RunExport`.

## 2. Canonical Authority

- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
- `handover/ops/ACTIVE_MISSION_CHARTER.md`
- `handover/ops/MISSION_GROUNDHOG_PROJECT_SPEC.md`
- `handover/ops/MISSION_GROUNDHOG_EXECUTION_LOOP.md`
- `handover/ops/MISSION_GROUNDHOG_IMPLEMENTATION_GUARDRAILS.md`
- `handover/ops/GEMINI_GROUNDHOG_OPERATING_POLICY.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_012_CLOSEOUT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_013_CLOSEOUT.md`

## 3. In Scope Files

Writable in this phase:

- `crates/turingos-observe/src/lib.rs`
- `crates/turingos-observe/src/run_summary.rs`
- `crates/turingos-observe/src/run_delta.rs`
- `crates/turingos-observe/src/run_sample.rs`
- `crates/turingos-observe/src/run_basis.rs`
- `handover/ops/MISSION_GROUNDHOG_PHASE_014_SPEC_DRAFT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_014_CLOSEOUT.md`
- one new append-only entry matching `handover/ai-direct/entries/*_groundhog_phase014_closeout.md`
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

- Phase 14 may add one compact detached comparison-basis type above `RunSummary`.
- The phase may:
  - capture only the exact current-state facts needed to reproduce `RunDelta`
  - expose a faithful path from `RunSummary` into that basis
  - expose a faithful path using that basis to derive sample or delta facts
  - reduce future polling-agent need to retain the full previous `RunExport`
- The phase must not:
  - move observer truth ownership out of `RunExport`
  - turn the basis into a replacement for `RunSummary`
  - add polling cadence, retries, watchdog logic, or supervisor behavior
  - add scheduler, provider, CLI, serialization, or storage logic
  - feed basis data back into the kernel

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

- one compact detached comparison-basis surface in `turingos-observe`
- one faithful derivation path from `RunSummary` into that basis
- one faithful comparison path from prior basis into current summary or sample
- tests proving:
  - the basis stores only the exact facts needed by `RunDelta`
  - basis-derived comparison is exactly equivalent to prior-summary comparison
  - the basis does not become a second observer-side truth packet
  - the basis does not become polling policy, daemon policy, or retry policy
- one Phase 14 closeout packet

## 9. Acceptance Criteria

- the basis surface lives in `turingos-observe`
- `RunExport` remains the sole observer-side owned truth packet
- `RunSummary` remains the borrowed compact current-state read surface
- `RunDelta` remains the canonical comparison layer
- the new basis is detached but minimal, derived, and explicitly non-authoritative
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

- A detached comparison basis can accidentally widen into a second owned truth packet if it copies fields that do not participate in comparison semantics.
- Convenience pressure may try to smuggle polling cadence or retry state into the new basis because it is explicitly aimed at future polling-agent consumers; that remains forbidden.
- If the basis API becomes too abstract, future consumers may ignore it and keep retaining full `RunExport` packets anyway.
