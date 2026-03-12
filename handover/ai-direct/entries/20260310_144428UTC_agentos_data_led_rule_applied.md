# Handover Entry

## Summary

- Mission: Mission 002 pressure-test and bounded self-upgrade loop
- Date: 2026-03-10 UTC
- Owner: Codex Commander

## What Changed

- Applied the new data-led upgrade rule to AgentOS itself instead of leaving it only at the project-governance layer.
- The core effect is:
  - expert roles may still generate hypotheses
  - but AgentOS promotion/debug/rerun decisions must now cite compressed historical success/failure evidence
- Updated:
  - `handover/ops/ROLE_CATALOG.md`
  - `handover/ops/MISSION_GROUNDHOG_EXECUTION_LOOP.md`
  - `handover/ops/MISSION_GROUNDHOG_TEAM.md`
  - `handover/ops/roles/GOAL_POLLER.md`
  - `handover/ops/CHILD_AGENT_OPERATING_PROFILE.md`
  - `.codex/agents/turingos_goal_poller.toml`
  - `handover/ai-direct/LATEST.md`

## Evidence

- `turingos_goal_poller` now explicitly requires:
  - compressed historical success/failure basis
  - exact evidence used for verdicts
  - no promotion-style verdict without an explicit evidence basis
- Groundhog execution loop now requires:
  - stage-close evidence compression before promotion or next-stage movement
  - retrospective recording of the evidence basis
- Child-agent packet/output rules now treat evidence basis as mandatory for debug/rerun/promotion decisions.
- `.codex` parse validation passed: `toml-ok 13`

## Open Risks

- The rule is now wired into AgentOS, but the active self-upgrade loop still needs to keep proving it by using rung evidence rather than persuasive but weak hypotheses.

## Next Step

- Continue the current bounded self-upgrade loop and report progress using the new AgentOS evidence-basis discipline.
