# Mission Groundhog Execution Loop

Status: Active
Mission: Mission Groundhog
Date: 2026-03-09 UTC

## 1. Loop Purpose

- Keep the project disciplined across many stages.
- Prevent uncontrolled scope drift.
- Force every stage to end in review instead of momentum-only continuation.
- Force upgrade and promotion decisions to remain data-led rather than intuition-led.

## 2. Mandatory Stage Loop

Every Groundhog stage must follow this exact loop:

1. Produce a bounded stage spec.
2. Get human confirmation on that stage spec unless the active mission charter explicitly enables an autonomous progression window.
3. Execute only that bounded stage.
4. Compress the current success/failure evidence into an explicit stage-close basis before deciding promotion or next-stage movement.
5. Run required audits.
6. Run the recursive audit pair defined in `handover/ops/MISSION_GROUNDHOG_TEAM.md`.
7. Write a stage retrospective and evidence record.
8. Propose the next stage spec.
9. Stop for human confirmation before continuing unless the active mission charter explicitly enables an autonomous progression window and the current stage has closed with PASS.

## 3. Autonomous Progression Rule

- An active mission charter may explicitly enable an autonomous progression window for Groundhog.
- While this mode is active, the Commander may move from one Groundhog stage to the next without waiting for a fresh human reply, but only if:
  - the current stage has a bounded spec
  - the current stage closes with PASS after the required audit pack
  - the stage retrospective and evidence record are written before the next stage opens
  - the next stage spec is still written as a bounded, reviewable artifact
- Any stage that closes with `FIX` or `BLOCK` must stop. Autonomous progression is allowed only on clean PASS.
- Any constitutional ambiguity or theorem-pressure escalation must stop regardless of audit status.

## 4. Stage-Close Verdict Schema

- Each required audit returns one of:
  - `PASS`
  - `FIX`
  - `BLOCK`
- The Commander must normalize those results into one stage-close verdict:
  - `PASS` means every required audit returned exact `PASS`, all required artifacts exist, and the closeout packet is complete.
  - `FIX` means at least one required audit returned `FIX`, no required audit returned `BLOCK`, and the stage must not auto-advance.
  - `BLOCK` means any required audit returned `BLOCK`, any required artifact is missing, or any constitutional or execution precondition remains unresolved.
- The normalized stage-close verdict must be written into the stage closeout packet before any next-stage spec opens.

## 5. Stage Spec Minimum Fields

Every stage spec must define:

- stage name
- objective
- canonical authority
- exact in-scope files
- exact out-of-scope files
- kernel/plugin boundary implications
- acceptance criteria
- required audits
- required tests
- open risks

## 6. Mandatory Audit Pack for Implementation Stages

- Groundhog math constitution custodian
- Groundhog formal methods auditor when invariants or transitions are involved
- OS systems planner when CLI, files, process model, or recovery are involved
- Karpathy editor when code shape is substantial enough to simplify
- quality auditor before phase close
- Groundhog recursive math auditor at stage close
- recursive software auditor at stage close

## 7. Retrospective Fields

Every stage closeout must record:

- what changed
- what evidence basis justified the change or non-change decision
- what was proven
- what remains uncertain
- what failed
- what design pressure appeared
- what next phase is now justified
- what the recursive math auditor concluded
- what the recursive software auditor concluded

## 8. Data-Led Promotion Rule

- Hypotheses may come from experts, but promotion decisions must be justified by recorded evidence.
- Any debug, rerun, or advance decision must cite the compressed historical success/failure basis it used.
- If the evidence basis is weak or missing, the loop must prefer `HOLD` or `STOP_AND_DEBUG` over speculative promotion.

## 9. Next-Stage Proposal Rule

- The Commander proposes the next stage spec only after the current stage has both evidence and audits.
- The next-stage proposal must be narrower than “continue implementation”.
- It must state exactly what new authority or new files it opens.
- In autonomous progression mode, the Commander may both close the current stage and open the next stage spec in the same execution window, but only after the PASS condition is recorded.

## 10. Kernel and Plug-In Rule

- Every stage spec must explicitly state whether it touches:
  - kernel
  - pluggable adapters
  - tools
  - CLI
  - validation only
- If a stage touches both kernel and pluggable code, the spec must show the boundary between them rather than treating them as one blob of work.

## 11. Beauty Rule

- Code compactness, naming quality, and abstraction restraint are first-class review concerns.
- A stage is not ready to close just because it is correct if its structure is still confused, inflated, or boundary-blurring.

## 12. Test Rule

- The stage spec must name the concrete test command before implementation starts.
- The default current repo command is:

```bash
./.venv/bin/python -m pytest -q
```

- Execution-window-specific hardware routing belongs in `handover/ops/ACTIVE_MISSION_CHARTER.md` or a stage-close runbook, not in this stable loop document.
- Any polling, supervision, or progress-watch behavior must be performed by agent roles such as `turingos_runtime_watcher`.
- For target-driven loops, `turingos_goal_poller` owns objective polling while `turingos_runtime_watcher` owns process-health polling.
- Do not introduce repository-resident watchdog or supervisor daemons in any language unless a future stage spec explicitly opens that surface.

## 13. Gemini Audit Operation Rule

- Groundhog Gemini roles must follow `handover/ops/GEMINI_GROUNDHOG_OPERATING_POLICY.md`.
- Default Groundhog Gemini command form is:

```bash
gemini --output-format stream-json -p "<bounded role packet>"
```

- Do not kill a Gemini preview-model process solely because it is silent for a while.
- Use native Gemini session evidence before deciding a run is hung.
- Short reflex timeout wrappers are forbidden for Groundhog Gemini audit work.

## 14. Stop Rule

- If an implementation pressure suggests changing the sealed constitution, the stage must stop.
- That is not a local refactor decision. It is a human-owner constitutional escalation.
