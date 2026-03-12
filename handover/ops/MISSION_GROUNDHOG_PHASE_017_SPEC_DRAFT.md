# Mission Groundhog Phase 17 Spec Draft

Status: Active under autonomous progression after Phase 16 PASS
Mission: Mission Groundhog
Phase: 17
Date: 2026-03-10 UTC

## 0. Phase State

- Phase 17 is now the active bounded stage.
- Phase 16 is closed and frozen by `handover/ops/MISSION_GROUNDHOG_PHASE_016_CLOSEOUT.md`.
- Phase 17 may not claim completion until it has:
  - opened only the bounded classified detached poll-frame surface below
  - passed the required audit ring
  - written `handover/ops/MISSION_GROUNDHOG_PHASE_017_CLOSEOUT.md`
  - written one append-only entry matching `handover/ai-direct/entries/*_groundhog_phase017_closeout.md`

## 1. Objective

- Open the first compact detached classified poll frame above `RunPulse` and `RunPulseClass`.
- Let future polling-agent consumers carry one owned detached object per poll cycle instead of coupling:
  - `RunPulse`
  - `RunPulseClass`
- Keep the frame policy-free, observer-side, and explicitly smaller in authority than `RunExport`.

## 2. Canonical Authority

- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
- `handover/ops/ACTIVE_MISSION_CHARTER.md`
- `handover/ops/MISSION_GROUNDHOG_PROJECT_SPEC.md`
- `handover/ops/MISSION_GROUNDHOG_EXECUTION_LOOP.md`
- `handover/ops/MISSION_GROUNDHOG_IMPLEMENTATION_GUARDRAILS.md`
- `handover/ops/GEMINI_GROUNDHOG_OPERATING_POLICY.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_015_CLOSEOUT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_016_CLOSEOUT.md`

## 3. In Scope Files

Writable in this phase:

- `crates/turingos-observe/src/lib.rs`
- `crates/turingos-observe/src/run_pulse.rs`
- `crates/turingos-observe/src/run_pulse_class.rs`
- `crates/turingos-observe/src/run_pulse_frame.rs`
- `handover/ops/MISSION_GROUNDHOG_PHASE_017_SPEC_DRAFT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_017_CLOSEOUT.md`
- one new append-only entry matching `handover/ai-direct/entries/*_groundhog_phase017_closeout.md`
- `handover/ai-direct/LATEST.md`
- `handover/BOARD.md`

Read-only context:

- `crates/turingos-observe/src/run_export.rs`
- `crates/turingos-observe/src/run_summary.rs`
- `crates/turingos-observe/src/run_basis.rs`
- `crates/turingos-observe/src/run_delta.rs`
- `crates/turingos-observe/src/run_sample.rs`
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

- Phase 17 may add one compact detached classified poll frame above `RunPulse` and `RunPulseClass`.
- The phase may:
  - package detached pulse facts and their already-derived macro-state into one owned observer packet
  - expose faithful derivation from `RunExport`
  - expose faithful derivation using prior `RunBasis`
  - reduce future polling-agent wrapper logic for carrying current poll state
- The phase must not:
  - move observer truth ownership out of `RunExport`
  - turn the new frame into a replacement for `RunPulse` or `RunPulseClass`
  - smuggle timestamps, retries, cadence, or health policy into the frame
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

- one compact detached classified poll frame in `turingos-observe`
- one faithful derivation path from current observer truth into that frame
- one faithful derivation path using prior `RunBasis`
- tests proving:
  - the frame remains detached and policy-free
  - frame-derived pulse and class facts equal the canonical split derivation path
  - the frame does not become a second observer-side truth packet
  - the frame does not become daemon, retry, or alert policy
- one Phase 17 closeout packet

## 9. Acceptance Criteria

- the frame surface lives in `turingos-observe`
- `RunExport` remains the sole observer-side owned truth packet
- `RunPulse` remains the detached current poll packet
- `RunPulseClass` remains the derived macro-state projection
- the new frame is detached, compact, derived, and explicitly non-authoritative
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

- A classified detached frame can accidentally widen into a second authoritative packet if it starts copying more than the canonical pulse-plus-class facts.
- Convenience pressure may try to add timestamps or operational health metadata because this stage targets polling-agent consumers; that remains forbidden.
- If the frame API is not clearly better than carrying `RunPulse` plus `RunPulseClass` separately, future consumers may ignore it and rebuild wrappers anyway.
