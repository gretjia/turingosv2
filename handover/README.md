# TuringOSv2 Handover and Multi-Agent Manual

This is the unified entrypoint under `/handover` for repository-level AI collaboration.

## Reading Order

1. `AGENTS.md`
2. `handover/README.md`
3. `README.md`
4. `handover/ai-direct/LATEST.md`
5. `handover/ops/ACTIVE_MISSION_CHARTER.md` when it exists
6. `handover/ops/MULTI_AGENT_OPERATING_SYSTEM.md` for substantial or delegated work
7. `handover/ops/CHILD_AGENT_OPERATING_PROFILE.md` when child agents or external audit roles are used
8. `handover/ops/ROLE_CATALOG.md` when role routing, triggers, or ownership need to be decided
9. `.codex/config.toml` when Codex child roles are used

If no mission charter exists for substantial work, instantiate one from `handover/ops/MISSION_CHARTER_TEMPLATE.md`.

## Governance Layers

### Layer 0: Project Contract

Default canonical sources:

- `README.md`
- `pyproject.toml`
- the relevant files under `turingos/`
- the relevant files under `tests/`

### Layer 1: Multi-Agent Operating System

Permanent repo-level collaboration rules:

- `handover/ops/MULTI_AGENT_OPERATING_SYSTEM.md`
- `handover/ops/CHILD_AGENT_OPERATING_PROFILE.md`
- `handover/ops/ROLE_CATALOG.md`
- `handover/ops/roles/*.md`
- `.codex/config.toml`
- `.codex/agents/*.toml`

### Layer 2: Mission Charter

Task-specific scope, acceptance gates, risks, and delegated ownership.

### Layer 3: Run Manifest and Handover

Execution-specific evidence and memory:

- `handover/ai-direct/LATEST.md`
- `handover/ai-direct/entries/`
- `handover/BOARD.md`

## What `/handover` Owns

`/handover` is the home for:

- onboarding order
- permanent multi-agent governance
- active state snapshots
- session handoffs
- mission charters
- debrief notes

## File Map

### Governance

- `AGENTS.md` — repo entrypoint for all agents
- `handover/README.md` — unified handover entrypoint
- `handover/ops/MULTI_AGENT_OPERATING_SYSTEM.md` — permanent roles and workflow gates
- `handover/ops/CHILD_AGENT_OPERATING_PROFILE.md` — child packet and authority rules
- `handover/ops/ACTIVE_MISSION_CHARTER.md` — current mission, if instantiated
- `handover/ops/MISSION_CHARTER_TEMPLATE.md` — task-level template
- `handover/ops/ROLE_CATALOG.md` — role registry, triggers, packet format, and ownership slices
- `handover/ops/roles/*.md` — full role specs
- `.codex/config.toml` — project-scoped Codex role registry
- `.codex/agents/*.toml` — per-role Codex configs

### Code

- `turingos/` — package source
- `tests/` — regression suite
- `audit/` — audit and framework notes

### Runtime Memory

- `handover/BOARD.md` — debrief board
- `handover/ai-direct/LATEST.md` — current state summary
- `handover/ai-direct/entries/` — durable entries
- `handover/ai-direct/HANDOVER_TEMPLATE.md` — session entry template

## Session Protocol

### Starting a session

1. Read the files in the reading order above.
2. Check `git status --short --branch`.
3. Identify `handover/ops/ACTIVE_MISSION_CHARTER.md`, or create one if the work is substantial.
4. If using child agents, define their ownership and stop conditions before delegation.

### Ending a session

1. Add a debrief or note to `handover/BOARD.md` if the work was substantial.
2. Create a session entry under `handover/ai-direct/entries/`.
3. Update `handover/ai-direct/LATEST.md`.

## Codex Child-Agent Notes

- Codex child roles in this repo are project-scoped.
- The project wiring lives in `.codex/config.toml`.
- Role instructions live in `.codex/agents/*.toml`.
- The human-readable governance source remains the handover docs above.
