# Mission Groundhog Project Spec

Status: Active
Mission: Mission Groundhog
Date: 2026-03-09 UTC

## 1. Project Character

- Mission Groundhog is not a one-shot feature delivery.
- It is a staged engineering program that turns the sealed constitution into durable software.
- Every stage must end with audit, retrospective, and a proposed next-stage spec.
- During explicitly enabled autonomous progression windows recorded in the active mission charter, the Commander may auto-open the next stage after PASS instead of waiting for a fresh human reply.
- Every stage must also end with the recursive audit pair defined in `handover/ops/MISSION_GROUNDHOG_TEAM.md`.
- Any stage that closes with `FIX` or `BLOCK` must stop rather than auto-advance.
- Stage-close verdict normalization is defined in `handover/ops/MISSION_GROUNDHOG_EXECUTION_LOOP.md` and is mandatory for autonomous progression.

## 2. Highest Authority

The whole project derives from:

1. `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
2. `handover/ops/MISSION_GROUNDHOG_CONSTITUTION_INDEX.md`
3. `handover/ops/ACTIVE_MISSION_CHARTER.md`
4. this file
5. `handover/ops/MISSION_GROUNDHOG_EXECUTION_LOOP.md`
6. `handover/ops/MISSION_GROUNDHOG_IMPLEMENTATION_GUARDRAILS.md`
7. `handover/ops/MISSION_GROUNDHOG_TEAM.md`
8. `handover/ops/GEMINI_GROUNDHOG_OPERATING_POLICY.md`

## 3. End State

Mission Groundhog aims to produce a constitution-faithful TuringOS embodiment with:

- Rust as the primary implementation language for the new core embodiment
- explicit separation between kernel and pluggable external agents/tools
- Unix-quality operational structure
- compact, legible, Karpathy-grade code style
- auditable commit/abort semantics
- reproducible tests, traces, and stage-level review records

## 4. Architectural Non-Negotiables

- Kernel and pluggable external code must be separated from the beginning, not retrofitted later.
- The black box may emit intent, but may not directly materialize future world state.
- The commit engine remains a white-box authority boundary.
- Every stage must preserve theorem authority and implementation provenance.

## 5. Core Workstreams

- Constitution-preservation workstream
  - theorem preservation
  - topology preservation
  - invariant review
- Kernel embodiment workstream
  - snapshot model
  - read tool
  - write tool
  - predicate gate
  - atomic commit engine
- Pluggable extension workstream
  - external agent adapters
  - external tool surfaces
  - integration boundaries
  - public contracts
- Unix systems workstream
  - CLI
  - files
  - logs
  - observability
  - process topology
- Validation workstream
  - unit tests
  - replay tests
  - invariant tests
  - integration traces
  - phase-close audit packets

## 6. Phase Model

The project runs as repeating phases, not one giant waterfall:

1. phase spec
2. execution
3. audit
4. retrospective
5. next-phase spec proposal
6. human confirmation, unless the active mission charter explicitly enables an autonomous progression window and the current phase closes with PASS

No phase may silently continue into the next one without either the confirmation step or the recorded autonomous-PASS rule.

## 7. Expected Early Phases

- Phase 1: Groundhog architecture map
  - define Rust workspace shape
  - define kernel versus pluggable boundary
  - define canonical types for snapshot, intent, predicates, and commit outcome
- Phase 2: core state and commit prototype
  - encode `Q_t`
  - encode intent envelope
  - encode predicate gate
  - encode commit/abort engine
- Phase 3: read/write tool surfaces and replayable traces
- Phase 4: pluggable black-box adapter boundary
- Phase 5: Unix CLI and operational surface
- Phase 6: deeper validation, benchmark, and migration planning

These are defaults, not locked sub-missions. Each phase still needs a confirmed phase spec at runtime.

## 8. Definition of Done for Any Phase

- the phase scope is closed against its own spec
- theorem and invariant audits are recorded
- OS and quality audits are recorded
- recursive math and recursive software audits are recorded
- resulting code remains compact and reviewable
- kernel/plugin separation remains explicit
- a next-phase spec is proposed to the human owner instead of assumed

## 9. Testing Rule

- Every implementation phase must leave behind a runnable test command.
- The current stable test path is the project-local virtual environment:
  - `./.venv/bin/python -m pytest -q`
- Groundhog should prefer repo-local reproducibility over dependence on the host Python package state.
- Runtime observation must be handled by polling agents rather than watchdog or supervisor Python programs.
- Execution-window-specific hardware routing belongs in the active mission charter or a stage runbook, not in this stable project spec.
