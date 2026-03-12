# TuringOSv2 Self-Upgrade Outer Promotion Spec

Status: Draft
Date: 2026-03-10 UTC
Scope: Outer whitebox promotion, audit, and gate-control spec for bounded self-improvement under the 1M pressure-test project

## 1. Purpose

- Define the external control plane that observes, audits, debugs, and either promotes or rejects inner self-upgrade candidates.
- Keep benchmark score, gate advancement, and acceptance decisions outside the inner runtime loop.
- Keep promotion decisions grounded in compressed historical success/failure evidence rather than expert intuition alone.
- Keep AgentOS in the role of scaffolding, audit, and promotion authority rather than letting it become the primary source of intelligence for `turingosv2`.
- Recognize that the outer AgentOS models are intelligent too, while still constraining that intelligence to support, critique, compression, audit, and promotion rather than long-term replacement of the inner proposer.

## 2. Outer Control Plane

Outer authority is held by:

- Commander
- `turingos_goal_poller`
- `turingos_runtime_watcher`
- the relevant debugging / planning / quality roles
- the recursive audit pair when changes are material

Outer authority does not mean outer authorship by default.
The preferred steady-state is:

- `turingosv2` originates bounded upgrade proposals
- AgentOS validates, criticizes, and either promotes or blocks them
- AgentOS may help the inner loop become smarter, but it should not remain the default author of the actual upgrade forever

## 3. Promotion Authority

Only the outer loop may decide:

- whether a rung is officially `PASS`
- whether a candidate is promoted, held, rerun, or rejected
- whether the next rung may begin
- whether a failure is semantic or infra
- whether anti-hardcoding constraints were preserved

## 4. Required Outer Verdicts

The outer loop may emit only:

- `ADVANCE`
- `HOLD`
- `RERUN_SAME_RUNG`
- `STOP_AND_DEBUG`

## 5. Promotion Requirements

Outer promotion requires all of:

- fixed-law compliance
- unchanged kernel
- unchanged topology
- declared changed surface limited to allowed inner surfaces
- required artifacts present
- no benchmark-local hardcoding findings
- current rung exact `PASS`
- promotion rationale traceable to observed run data and compressed cross-run failure/success patterns

## 6. Required Artifacts

Per candidate or rung:

- run manifest
- mode summary
- case-level results for non-pass cases
- first-fail chain when present
- exact changed files
- hypothesis note
- anti-hardcoding review note

## 7. Debug / Fix / Test Cycle

The outer loop owns this sequence:

1. observe current rung artifacts
2. classify blocker type
3. request or prepare a bounded inner proposal packet
4. review candidate change
5. rerun the same rung or advance
6. write retrospective memory

Expert roles may help generate hypotheses, but they may not substitute for recorded evidence when deciding promotion.
They also should not, by default, substitute for the inner loop as the origin of the candidate revision.

The official scored run itself remains evaluation-only and may not mutate policy mid-run.

## 8. Anti-Hardcoding Promotion Rule

Promotion must be blocked if a candidate:

- encodes case-local paths, values, or register states
- targets only the known current failing case without a general protocol reason
- introduces hidden repair logic
- changes packaging or wording in a way that obscures failure evidence

## 9. Safe Reading Of The 100-Pass Goal

- `100` consecutive passes is the first-stage external target.
- It is not a whitebox theorem.
- It is a gate that measures whether bounded inner revisions are improving real protocol-following under unchanged law.

## 10. Current Intended Use

- Current loop: Mission 002 pressure-test prompt/debug progression
- Current headline mode: `turingosv2_planner27_worker9`
- Current ladder: `1 -> 3 -> 10 -> 25 -> 50 -> 100`
- Current promotion responsibility: outer AgentOS team, not inner runtime code
- Current correction note:
  - some early bounded prompt fixes were still authored directly by the outer loop
  - this is transitional behavior, not the desired end-state for the self-upgrade architecture
  - the intended end-state is that outer intelligence helps `turingosv2` stand up its own bounded project intelligence and then stays in the role of evidence-led governor
