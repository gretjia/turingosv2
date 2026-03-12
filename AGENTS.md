# AGENTS.md — TuringOSv2 Unified Agent Entry Point

This file governs repository-level development workflow for `turingosv2`.

It does not define the runtime semantics of `turingos/agents/`. Those modules are product code. This document is only about how AI agents should collaborate while programming this repository.

## Quick Start

1. Read this file.
2. Read `handover/README.md`.
3. Read `README.md`.
4. Read `handover/ai-direct/LATEST.md`.
5. For substantial work, read `handover/ops/ACTIVE_MISSION_CHARTER.md` if present, or instantiate one from `handover/ops/MISSION_CHARTER_TEMPLATE.md`.
6. If child agents or external audit roles will be used, read `handover/ops/CHILD_AGENT_OPERATING_PROFILE.md` and `handover/ops/ROLE_CATALOG.md`.
7. If Codex child roles will be used, load the repo-local `.codex/config.toml`.

## Project Identity

TuringOSv2 is a Python reference implementation of an AI Turing Machine operating system prototype.

Primary code surfaces:

- `turingos/` — package source
- `tests/` — regression suite
- `README.md` — canonical product overview
- `audit/` — audit and framework notes
- `bible/` — top-level design canon for long-horizon Turing-machine, multi-agent, and AGI-direction work

## Hard Rules

1. AgentOS in this repo governs coding workflow only. Do not confuse it with the runtime agent model implemented under `turingos/`.
2. One Commander integrates final state. Child agents are bounded execution tools, not authorities.
3. No silent scope expansion. If the task widens, reopen scope explicitly.
4. Coder children may edit only their assigned file set.
5. Keep child-role wiring project-scoped in `.codex/`. Do not move these roles into `~/.codex/config.toml`.
6. `README.md`, `pyproject.toml`, and the relevant tests are the default project contract unless a mission charter narrows or extends the task.
7. Prefer small, test-backed changes over broad refactors.
8. For substantial work, update `handover/ai-direct/LATEST.md` and add a handover entry.

## Multi-Agent Files

- `handover/README.md` — unified handover and onboarding entrypoint
- `handover/ops/MULTI_AGENT_OPERATING_SYSTEM.md` — permanent multi-agent governance
- `handover/ops/CHILD_AGENT_OPERATING_PROFILE.md` — delegated child-agent rules
- `handover/ops/ACTIVE_MISSION_CHARTER.md` — current task-level canonical spec when instantiated
- `handover/ops/MISSION_CHARTER_TEMPLATE.md` — task-level execution template
- `handover/ops/ROLE_CATALOG.md` — role index, triggers, packet format, and ownership map
- `handover/ops/roles/*.md` — full specs for each role
- `.codex/config.toml` — repo-local Codex child-role registry
- `.codex/agents/*.toml` — role-specific Codex child-agent instructions

## Testing

Primary command:

```bash
python3 -m pytest -q
```

For narrower work, run the smallest relevant test target first, then the full suite if the change touches shared behavior.

## Handover

- `handover/ai-direct/LATEST.md` — current state snapshot
- `handover/ai-direct/entries/` — durable session entries
- `handover/BOARD.md` — shared debrief board
