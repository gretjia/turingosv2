# Active Mission Charter

Status: Draft awaiting human confirmation
Task Name: Mission Groundhog - Final Clockwork Embodiment Stage 001 - pre-implementation design freeze
Owner: Human Owner
Commander: Codex
Date: 2026-03-12 UTC

## 1. Objective

- Open a new bounded mission charter for the pre-implementation design-freeze stage that must precede the first implementation stage under the sealed Final Clockwork constitution.
- Translate the sealed constitution and reviewed code diff into an implementation-grade stage spec before any further code changes.
- Present that stage spec only after multi-agent review, so human confirmation happens on an audited package rather than on a raw draft.

## 2. Canonical Spec

Constitutional authority:

- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`

Derived planning inputs:

- `handover/ops/TURINGOSV2_FINAL_CLOCKWORK_CODE_DIFF_AUDIT_20260311.md`
- `handover/ops/TURINGOSV2_RUST_ONLY_BLUEPRINT_AUDITED_20260311.md`
- `handover/ops/TURINGOSV2_RUST_ONLY_MIGRATION_PLAN_20260311.md`
- `handover/ops/MISSION_GROUNDHOG_EXECUTION_LOOP.md`
- `handover/ops/MISSION_GROUNDHOG_IMPLEMENTATION_GUARDRAILS.md`
- `handover/ops/TURINGOSV2_CONSTITUTION_AI_FRIENDLY_REWRITE_20260311.md`

Supporting context:

- `README.md`
- `handover/ai-direct/LATEST.md`
- `crates/`
- `turingos/`
- `harnesses/`

Conflict rule:

- If any draft design, convenience abstraction, or migration shortcut conflicts with the sealed Final Clockwork constitution, the constitution wins.
- If the draft spec seems to require reinterpretation of the constitution rather than implementation under it, stop and escalate instead of widening scope.

## 3. Scope

Writable files:

- `handover/ops/ACTIVE_MISSION_CHARTER.md`
- `handover/ops/MISSION_GROUNDHOG_CLOCKWORK_STAGE_001_SPEC_DRAFT.md`
- `handover/ai-direct/LATEST.md`
- `handover/ai-direct/entries/`

Read-only but relevant files:

- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
- `crates/`
- `turingos/`
- `harnesses/`

Explicitly out of scope:

- implementation changes under `crates/`
- implementation changes under `turingos/`
- implementation changes under `harnesses/`
- edits to the sealed constitution

## 4. Roles

Commander:

- integrate the mission charter, stage spec, audit packet, and final confirmation package

Required review lenses:

- mathematics and constitutional fidelity
- topology and OS boundary discipline
- minimalism and code-architecture sharpness
- stage-loop and sequencing discipline

## 5. Acceptance Criteria

- `handover/ops/MISSION_GROUNDHOG_CLOCKWORK_STAGE_001_SPEC_DRAFT.md` exists and defines a bounded first implementation stage.
- The draft stage spec names exact in-scope files, out-of-scope files, acceptance gates, and required audits.
- The draft stage spec clearly states that Stage 001 is design-freeze only and does not yet authorize implementation edits.
- A multi-agent review has been performed on the draft and its findings are folded into the final reviewed version.
- `LATEST.md` and a new handover entry record the new mission charter and reviewed stage spec.

## 6. Required Checks

- cross-check the draft against the sealed Final Clockwork constitution
- cross-check the draft against the reviewed code diff and Rust-only blueprint
- record that no implementation code changed in this mission-opening stage

## 7. Constraints

- evidence preservation: the stage spec must cite the reviewed diff audit rather than re-invent the problem statement
- runtime constraints: none; this is a documentation-and-review stage only
- rollback constraints: do not open implementation scope implicitly through wording
- no autonomous progression window is enabled for Stage 001; human confirmation is required before any Stage 002 implementation work opens

## 8. Risks

- The stage spec could drift into implementation detail without first freezing the opening packet for the first implementation stage.
- The stage spec could blur trusted whitebox stages with external harness responsibilities.
- If the first implementation stage is too broad, it will violate the bounded-stage loop before code even begins.
