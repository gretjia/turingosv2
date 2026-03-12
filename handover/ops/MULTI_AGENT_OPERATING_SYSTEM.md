# TuringOSv2 Multi-Agent Operating System

Status: Active
Scope: Repository-level AI coding workflow for `turingosv2`

## 1. Purpose

This document governs how multiple AI agents collaborate while programming this repository.

It is not part of the product runtime. It exists to keep development work:

- scoped
- auditable
- reviewable
- easy to integrate

## 2. Canonical Reading Order

Every substantial session starts with:

- `AGENTS.md`
- `handover/README.md`
- `README.md`
- `handover/ai-direct/LATEST.md`
- `handover/ops/ACTIVE_MISSION_CHARTER.md` if it exists

If child agents or external audit roles will be used, also read:

- `handover/ops/CHILD_AGENT_OPERATING_PROFILE.md`
- `handover/ops/ROLE_CATALOG.md`

If Codex child roles will be used, also load:

- `.codex/config.toml`

## 3. Governance Layers

### Layer 0: Project Contract

The default contract comes from:

- `README.md`
- relevant source files under `turingos/`
- relevant tests under `tests/`
- `bible/` when the mission touches runtime architecture, agent orchestration, long-horizon state, or AGI direction

### Layer 1: Multi-Agent Operating System

This document plus:

- `handover/ops/CHILD_AGENT_OPERATING_PROFILE.md`
- `handover/ops/ROLE_CATALOG.md`
- `handover/ops/roles/*.md`
- `.codex/config.toml`
- `.codex/agents/*.toml`

### Layer 2: Mission Charter

Task-specific scope, acceptance criteria, ownership, and risks.

### Layer 3: Run Manifest

Session-specific evidence in:

- `handover/ai-direct/LATEST.md`
- `handover/ai-direct/entries/`
- `handover/BOARD.md`

## 4. Core Principles

1. This AgentOS is for repository programming only.
2. One Commander owns final integration.
3. No agent may silently widen scope.
4. Child edits require explicit file ownership.
5. Audit and implementation are separate roles.
6. External facts should be checked against official or primary sources when the task is unstable.
7. Tests are part of the definition of done for behavioral changes.
8. Handover memory is part of the working state, not optional decoration.
9. Architecture audit is intentionally split into a Turing-AGI lens and an OS/Linux systems lens.
10. Missions may add specialist overlays, but sealed constitutional files in `bible/` outrank all derived engineering docs for constitution-bound work.
11. Groundhog stages must end with a recursive math audit and a recursive software audit before the next stage spec is proposed.

## 5. Roles

### 5.1 Commander

Mission:

- define scope
- choose whether to delegate
- assign ownership
- integrate final result
- own handover and git decisions

Allowed:

- read any repo file
- edit any file when necessary
- run tests
- update handover
- commit and push

Forbidden:

- delegating final integration authority

### 5.2 Plan Agent

Mission:

- produce a bounded change map

Allowed:

- read files
- summarize current state
- define acceptance gates and risks

Forbidden:

- code edits
- git
- handover writes

### 5.3 Coder Agent

Mission:

- implement only the assigned file set

Allowed:

- edit assigned files
- run explicitly assigned checks

Forbidden:

- editing outside ownership
- commit
- push
- handover writes unless explicitly assigned for documentation-only work

### 5.4 Turing-AGI Architect Auditor

Mission:

- derive top-level design constraints from `bible/`
- audit whether the change moves the repo toward long-horizon, white-box-controlled, heterogeneous multi-agent AGI work

Focus:

- persistence and long-cycle task execution
- white-box control over black-box workers
- shared-spec coordination across many heterogeneous agents
- drift away from the repo's top-level Turing-machine and AGI direction

Forbidden:

- code edits during audit
- git
- handover writes

Default engine:

- Gemini CLI via `gemini -y -p`

### 5.5 OS Systems Planner

Mission:

- plan and audit system structure using OS and Linux design principles

Focus:

- capability boundaries
- mechanism vs policy separation
- filesystem, CLI, config, and process-model hygiene
- persistence, recovery, observability, and migration safety

Forbidden:

- code edits during audit unless the mission explicitly assigns docs-only work
- git
- handover writes

### 5.6 Quality Auditor

Mission:

- verify regression safety and test adequacy

Focus:

- failing or missing tests
- CLI and package behavior
- rollback risk
- coverage gaps introduced by the change

Forbidden:

- code edits during audit
- git
- handover writes

### 5.7 Research Agent

Mission:

- gather external facts needed for implementation

Focus:

- official docs
- primary sources
- unstable APIs or tooling behavior

Forbidden:

- code edits
- git
- handover writes

### 5.8 Runtime Watcher

Mission:

- monitor long-running tests, demos, or benchmark jobs

Default mode:

- read-only

Forbidden:

- changing runtime settings
- restarting jobs unless explicitly instructed

### 5.9 Specialist Mission Overlays

Some missions may install specialist overlays instead of bloating the default role set.

Mission Groundhog is the current example:

- Gemini-only math roles stay documented in `handover/ops/roles/*.md`
- Codex-executed engineering specialists stay wired in `.codex/`
- the active mission charter defines which overlay is currently authoritative

## 6. Authority Matrix

| Action | Commander | Plan | Coder | Turing-AGI Architect Auditor | OS Systems Planner | Quality Auditor | Research | Watcher |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Read repo | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes |
| Edit assigned files | Yes | No | Yes | No | No | No | No | No |
| Edit unassigned files | Yes | No | No | No | No | No | No | No |
| Run tests | Yes | No | Only if assigned | No | No | Only if assigned | No | No |
| Browse external docs | Yes | Yes | If needed | If needed | If needed | If needed | Yes | No |
| Update handover | Yes | No | Only if assigned | No | No | No | No | No |
| Commit / push | Yes | No | No | No | No | No | No | No |

## 7. Mandatory Workflow

No substantial multi-agent task should skip these gates.

### Phase 0: Scope Lock

Commander defines:

- objective
- canonical spec
- exact files in scope
- out-of-scope files
- acceptance gates

Gate:

- scope is unambiguous

### Phase 1: Context Build

Read the minimum documents and code needed to understand the task.

If external APIs or tooling are involved, confirm unstable facts against official or primary sources before implementation.

Gate:

- the implementation target is concrete

### Phase 2: Plan

Plan child or Commander produces:

- in-scope files
- out-of-scope files
- expected tests
- risks
- open assumptions

Gate:

- there is a bounded change map

### Phase 3: Design Audit

For medium or large tasks, run the appropriate design auditors before coding:

- Turing-AGI Architect Auditor for top-level direction and `bible/` alignment
- OS Systems Planner for boundary, Linux, CLI, config, and recovery structure

Gate:

- pre-implementation architectural risks are explicit

### Phase 4: Implementation

Coder children work only inside assigned ownership.

Gate:

- patch stays inside ownership

### Phase 5: Final Audit

Run the relevant auditors after implementation:

- Turing-AGI Architect Auditor for top-level direction drift when applicable
- OS Systems Planner for system-boundary and Linux/OS hygiene when applicable
- Quality Auditor for regressions, missing tests, and runtime risk

Gate:

- unresolved findings are either fixed or explicitly accepted by the Commander

### Phase 6: Fix / Re-audit

Any blocked finding reopens only the required scope, not the entire task.

Gate:

- critical findings resolved

### Phase 7: Handover

For substantial work:

- add a session entry
- update `LATEST.md`
- record open risks and next steps

## 8. Delegation Discipline

Preferred delegation units:

- one code path
- one audit pass
- one research question
- one runtime watch function

Avoid:

- mixed plan + code + audit in one child
- overlapping writable ownership
- repo-wide open-ended delegation

## 9. Escalation Triggers

Stop and escalate if any child encounters:

- scope drift
- ambiguous canonical spec
- dirty-tree conflict
- file ownership conflict
- external dependency ambiguity
- evidence that required tests or checks are missing from the agreed scope

## 10. Default Routing

Use the lightest routing that still covers the risk.

- Small implementation task: Commander -> Coder -> Quality Auditor
- Medium task: Commander -> Plan -> one or more Coder children -> OS Systems Planner -> Quality Auditor
- Architecture task: Commander -> Plan -> Turing-AGI Architect Auditor + OS Systems Planner -> Coder children -> Turing-AGI Architect Auditor + OS Systems Planner + Quality Auditor
- Externally unstable task: add Research Agent before coding
- Long-running validation: add Runtime Watcher after launch
