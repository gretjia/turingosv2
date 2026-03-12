# Mission Groundhog Phase 18 Spec Draft

Status: Active under autonomous progression after the initial wrapper attempt was rejected by simplification audit
Mission: Mission Groundhog
Phase: 18
Date: 2026-03-10 UTC

## 0. Phase State

- Phase 18 is now the active bounded stage.
- Phase 17 is closed and frozen by `handover/ops/MISSION_GROUNDHOG_PHASE_017_CLOSEOUT.md`.
- Phase 18 may not claim completion until it has:
  - opened only the bounded direct basis-carry seam below
  - passed the required audit ring
  - written `handover/ops/MISSION_GROUNDHOG_PHASE_018_CLOSEOUT.md`
  - written one append-only entry matching `handover/ai-direct/entries/*_groundhog_phase018_closeout.md`

## 1. Objective

- Open one direct basis-carry seam on `RunPulseFrame`.
- Let future polling-agent consumers treat `RunPulseFrame::basis()` as the exact carry path for the next-cycle comparison token.
- Keep the seam policy-free, observer-side, and explicitly smaller in authority than `RunExport`.

## 2. Canonical Authority

- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
- `handover/ops/ACTIVE_MISSION_CHARTER.md`
- `handover/ops/MISSION_GROUNDHOG_PROJECT_SPEC.md`
- `handover/ops/MISSION_GROUNDHOG_EXECUTION_LOOP.md`
- `handover/ops/MISSION_GROUNDHOG_IMPLEMENTATION_GUARDRAILS.md`
- `handover/ops/GEMINI_GROUNDHOG_OPERATING_POLICY.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_016_CLOSEOUT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_017_CLOSEOUT.md`

## 3. In Scope Files

Writable in this phase:

- `crates/turingos-observe/src/lib.rs`
- `crates/turingos-observe/src/run_pulse_frame.rs`
- `handover/ops/MISSION_GROUNDHOG_PHASE_018_SPEC_DRAFT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_018_CLOSEOUT.md`
- one new append-only entry matching `handover/ai-direct/entries/*_groundhog_phase018_closeout.md`
- `handover/ai-direct/LATEST.md`
- `handover/BOARD.md`

Read-only context:

- `crates/turingos-observe/src/run_export.rs`
- `crates/turingos-observe/src/run_basis.rs`
- `crates/turingos-observe/src/run_pulse.rs`
- `crates/turingos-observe/src/run_pulse_class.rs`
- `crates/turingos-observe/src/run_pulse_frame.rs`
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

- Phase 18 may add one direct basis-carry seam on `RunPulseFrame`.
- The phase may:
  - expose the exact next-cycle comparison basis directly from the current classified frame
  - expose faithful derivation from `RunExport`
  - expose faithful derivation using prior `RunBasis`
  - reduce future polling-agent wrapper logic for carrying current cycle plus next-cycle carry token
- The phase must not:
  - move observer truth ownership out of `RunExport`
  - turn the new seam into a second parallel truth surface
  - smuggle timestamps, retries, cadence, polling policy, or health interpretation into the frame
  - add scheduler, provider, CLI, serialization, or storage logic
  - feed frame data back into the kernel

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

- one direct basis-carry seam in `turingos-observe`
- one faithful derivation path from current observer truth into that seam
- one faithful derivation path using prior `RunBasis`
- tests proving:
  - the seam remains detached and policy-free
  - frame-derived next-basis facts equal the canonical split derivation path
  - the seam does not become a second observer-side truth surface
  - the seam does not become daemon, retry, or alert policy
- one Phase 18 closeout packet

## 9. Acceptance Criteria

- the basis-carry seam lives in `turingos-observe`
- `RunExport` remains the sole observer-side owned truth packet
- `RunPulseFrame` remains the detached current classified poll packet
- `RunBasis` remains the detached comparison token
- the new seam is detached, compact, derived, and explicitly non-authoritative
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

- A direct basis-carry seam can still widen into accessor creep if it becomes a pretext for flattened convenience expansion.
- Convenience pressure may try to add timestamps, poll counters, retries, or health metadata because this stage targets polling-agent carry surfaces; that remains forbidden.
- If the seam API is not clearly better than reaching through `frame.pulse().current()`, future consumers may ignore it and rebuild wrappers anyway.
