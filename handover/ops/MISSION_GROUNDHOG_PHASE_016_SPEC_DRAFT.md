# Mission Groundhog Phase 16 Spec Draft

Status: Completed and frozen after Phase 16 PASS
Mission: Mission Groundhog
Phase: 16
Date: 2026-03-10 UTC

## 0. Phase State

- Phase 16 is now closed and frozen.
- Phase 15 is closed and frozen by `handover/ops/MISSION_GROUNDHOG_PHASE_015_CLOSEOUT.md`.
- Phase 16 is closed by `handover/ops/MISSION_GROUNDHOG_PHASE_016_CLOSEOUT.md`.
- Phase 16 may not claim completion until it has:
  - opened only the bounded deterministic pulse-classification surface below
  - passed the required audit ring
  - written `handover/ops/MISSION_GROUNDHOG_PHASE_016_CLOSEOUT.md`
  - written one append-only entry matching `handover/ai-direct/entries/*_groundhog_phase016_closeout.md`

## 1. Objective

- Open the first deterministic pulse-classification surface above `RunPulse`.
- Let future polling-agent consumers classify unchanged, progressed, regressed, and terminal macro-state transitions without rebuilding that logic ad hoc.
- Keep the classifier policy-free, observer-side, and derived only from `RunPulse`.

## 2. Canonical Authority

- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
- `handover/ops/ACTIVE_MISSION_CHARTER.md`
- `handover/ops/MISSION_GROUNDHOG_PROJECT_SPEC.md`
- `handover/ops/MISSION_GROUNDHOG_EXECUTION_LOOP.md`
- `handover/ops/MISSION_GROUNDHOG_IMPLEMENTATION_GUARDRAILS.md`
- `handover/ops/GEMINI_GROUNDHOG_OPERATING_POLICY.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_014_CLOSEOUT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_015_CLOSEOUT.md`

## 3. In Scope Files

Writable in this phase:

- `crates/turingos-observe/src/lib.rs`
- `crates/turingos-observe/src/run_basis.rs`
- `crates/turingos-observe/src/run_delta.rs`
- `crates/turingos-observe/src/run_pulse.rs`
- `crates/turingos-observe/src/run_pulse_class.rs`
- `handover/ops/MISSION_GROUNDHOG_PHASE_016_SPEC_DRAFT.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_016_CLOSEOUT.md`
- one new append-only entry matching `handover/ai-direct/entries/*_groundhog_phase016_closeout.md`
- `handover/ai-direct/LATEST.md`
- `handover/BOARD.md`

Read-only context:

- `crates/turingos-observe/src/run_export.rs`
- `crates/turingos-observe/src/run_summary.rs`
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

- Phase 16 may add one deterministic classification layer above `RunPulse`.
- The phase may:
  - classify unchanged vs progressed vs regressed vs terminal-reclassified macro-state based on pulse facts only
  - classify terminal/fault/abort/budget classes based on current pulse facts only
  - reduce future polling-agent need to rebuild classification logic ad hoc
- The phase must not:
  - add timestamps, time windows, retry counters, or alert thresholds
  - add scheduler, provider, CLI, serialization, or storage logic
  - feed classification data back into the kernel
  - create a daemon, supervisor, or background watcher

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

- one deterministic pulse-classification surface in `turingos-observe`
- one faithful derivation path from `RunPulse` into that classification
- tests proving:
  - unchanged classification is exact
  - progress classification is exact
  - regression classification is exact
  - terminal/fault/budget classes remain faithful to pulse facts
  - the classifier introduces no timing or retry policy
- one Phase 16 closeout packet

## 9. Acceptance Criteria

- the classification surface lives in `turingos-observe`
- `RunExport` remains the sole observer-side owned truth packet
- `RunPulse` remains the detached current poll packet
- the new classifier is deterministic, derived, and explicitly non-authoritative
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

- A pulse classifier can accidentally drift into alerting policy if it starts encoding thresholds, escalation, or retry semantics instead of pure state classes.
- Convenience pressure may try to add timestamps or rates into the classifier because it targets polling-agent consumers; that remains forbidden.
- If the classifier API overfits one consumer, future consumers may bypass it and rebuild their own parallel logic anyway.
