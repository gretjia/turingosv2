# Mission Groundhog Phase 1 Spec Draft

Status: Completed and frozen
Mission: Mission Groundhog
Phase: 1
Date: 2026-03-09 UTC

## 0. Phase State

- This file is a pre-execution stage spec, not a stage-close packet.
- This file is writable only until Phase 1 is declared opened.
- Once Phase 1 is opened, this spec becomes read-only provenance. Any later change must happen through an explicit amendment artifact, not by mutating this file in place.
- Phase 1 may close only after these additional artifacts exist:
  - `handover/ops/MISSION_GROUNDHOG_PHASE_001_ARCHITECTURE_PACKET.md`
  - `handover/ops/MISSION_GROUNDHOG_PHASE_001_CLOSEOUT.md`
  - one append-only entry matching `handover/ai-direct/entries/*_groundhog_phase001_closeout.md`
- A Phase 1 `PASS` is impossible until the normalized stage-close verdict recorded in the closeout packet is exact `PASS`.

## 1. Objective

- Produce the first bounded Groundhog architecture map before major Rust implementation begins.
- Translate the sealed constitution into an implementation-ready structure without mutating the constitution.
- Lock the kernel versus pluggable external boundary from the start.

## 2. Canonical Authority

- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
- `handover/ops/MISSION_GROUNDHOG_CONSTITUTION_INDEX.md`
- `handover/ops/ACTIVE_MISSION_CHARTER.md`
- `handover/ops/MISSION_GROUNDHOG_PROJECT_SPEC.md`
- `handover/ops/MISSION_GROUNDHOG_EXECUTION_LOOP.md`
- `handover/ops/MISSION_GROUNDHOG_IMPLEMENTATION_GUARDRAILS.md`
- `handover/ops/MISSION_GROUNDHOG_TEAM.md`

## 3. In Scope Files

Writable in this phase:

- `handover/ops/MISSION_GROUNDHOG_PHASE_001_ARCHITECTURE_PACKET.md`
- `handover/ops/MISSION_GROUNDHOG_PHASE_001_CLOSEOUT.md`
- one new append-only entry matching `handover/ai-direct/entries/*_groundhog_phase001_closeout.md`
- `handover/ai-direct/LATEST.md`
- `handover/BOARD.md`

Read-only authority and context:

- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
- `handover/ops/MISSION_GROUNDHOG_CONSTITUTION_INDEX.md`
- `handover/ops/ACTIVE_MISSION_CHARTER.md`
- `handover/ops/MISSION_GROUNDHOG_PROJECT_SPEC.md`
- `handover/ops/MISSION_GROUNDHOG_EXECUTION_LOOP.md`
- `handover/ops/MISSION_GROUNDHOG_IMPLEMENTATION_GUARDRAILS.md`
- `handover/ops/MISSION_GROUNDHOG_TEAM.md`
- existing philosophy canon under `bible/`
- existing Python reference code under `turingos/`

## 4. Out of Scope Files

- `bible/GROUNDHOG_SEALED_CONSTITUTION.md` remains strictly read-only
- `Cargo.toml`
- `Cargo.lock`
- `rust/`
- `crates/`
- `src/`
- `turingos/`
- `tests/`
- `benchmarks/`

## 5. Surface Touch Matrix

- kernel: architecture mapping only, no implementation edits
- pluggable adapters: boundary mapping only, no implementation edits
- tools: boundary mapping only, no implementation edits
- CLI: boundary implications may be described, no implementation edits
- validation: Phase 1 may define validation strategy and phase-close evidence, but may not open new implementation tests

## 6. Kernel/Plugin Boundary Implications

- Phase 1 must define the boundary between:
  - kernel embodiment
  - pluggable external agents
  - external tools
  - CLI and operational surfaces
- Phase 1 may describe future Rust roots, but it may not open them yet.
- Phase 1 must explicitly keep the black-box intent envelope separate from committed world-state materialization.
- Phase 1 must state that no plug-in or model adapter receives a direct write path to future state.

## 7. Sealed-File Handling

- `bible/GROUNDHOG_SEALED_CONSTITUTION.md` is read-only.
- It may not be reformatted, “clarified”, or mixed into derivation commits.
- Any future supersession must happen through a new sealed file plus explicit handover record, never by in-place edit.

## 8. Required Roles

Execution lanes:

- `turingos_plan`
- `groundhog_math_constitution_custodian`
- `groundhog_turing_machine_theorist`
- `turingos_os_system_planner`
- `turingos_rust_systems_architect`
- `turingos_git_historian`

Phase-close recursive auditors:

- `groundhog_recursive_math_auditor`
- `turingos_recursive_software_auditor`

Phase-close auditors:

- `groundhog_formal_methods_auditor`
- `turing_agi_architect_auditor`
- `turingos_quality_auditor`

All Gemini roles in this phase must use `Gemini 3.1 Pro Preview`.

## 9. Deliverables

- one `Phase 1 architecture packet` at `handover/ops/MISSION_GROUNDHOG_PHASE_001_ARCHITECTURE_PACKET.md` with these fixed sections:
  - Rust workspace shape
  - kernel versus plug-in boundary map
  - canonical types for:
    - universe snapshot `Q_t`
    - intent envelope
    - predicate gate
    - atomic commit/abort outcome
  - proposed crate/module map naming the intended Rust homes and public/private seams for:
    - `Q_t`
    - intent envelope
    - predicate gate
    - commit/abort outcome
    - read tool surface
    - write tool surface
  - mapping of read phase, generate phase, and commit/abort phase onto modules
  - Rust type and ownership plan
  - invariant-sensitive API boundary matrix
  - Unix operational shape note
  - Git provenance and migration-sequencing note
  - early validation and proof-obligation plan
- one `Phase 1 closeout packet` at `handover/ops/MISSION_GROUNDHOG_PHASE_001_CLOSEOUT.md` with these fixed sections:
  - normalized stage-close verdict:
    - `PASS`
    - `FIX`
    - `BLOCK`
  - audit verdict matrix
  - retrospective fields required by `MISSION_GROUNDHOG_EXECUTION_LOOP.md`
  - evidence and commands
  - exact Phase 2 opening slice
  - any Gemini deviation record required by `handover/ops/ACTIVE_MISSION_CHARTER.md`
- one append-only handover entry matching `handover/ai-direct/entries/*_groundhog_phase001_closeout.md`

## 10. Acceptance Criteria

- the proposed structure is constitution-consistent
- kernel and plug-in boundaries are explicit
- exact in-scope and out-of-scope file boundaries are recorded
- Rust embodiment path is specific enough to begin implementation in Phase 2
- Unix/process/filesystem implications are stated
- git/provenance slicing advice is recorded as a durable phase artifact
- type-level invariants guaranteeing atomic commit and zero-mutation abort are defined
- formal proof obligations for read, generate, and commit/abort invariants are stated
- the first intended Phase 2 opening slice is named explicitly
- the exact Phase 1 architecture packet and closeout packet paths are honored
- any Gemini CLI workaround is captured with durable provenance in the closeout packet
- no Gemini workaround is stage-close eligible unless the recorded observed model string remains `gemini-3.1-pro-preview`
- next implementation phase can be bounded without reopening theorem questions

## 11. Required Audits

- constitution audit
- formal methods audit
- Turing-machine semantics audit
- Unix systems audit
- AGI direction audit
- quality audit
- recursive audit pair

## 12. Required Tests And Validation

- repo guard test command:

```bash
./.venv/bin/python -m pytest -q
```

- phase-specific validation:
  - authority-chain completeness check
  - file-scope completeness check
  - audit-packet completeness check
  - proof-obligation completeness check

## 13. Migration Sequencing

- Phase 1 is documentation-only plus audit packaging.
- Phase 1 does not open Rust implementation roots.
- The intended first Phase 2 opening slice is:
  - Rust workspace scaffold
  - core types for `Q_t`, intent envelope, predicate gate, and commit outcome
  - no model adapters and no CLI work yet

## 14. Open Risks

- the repo currently has both a Python reference implementation and a future Rust embodiment path
- crate boundaries can still be overdesigned if Phase 1 is allowed to sprawl
- theorem vocabulary can drift if derived type names become too implementation-specific too early
