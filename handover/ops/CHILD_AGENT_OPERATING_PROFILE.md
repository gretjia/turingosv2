# TuringOSv2 Child Agent Operating Profile

Status: Active
Scope: Any delegated child agent working on repository development tasks in `turingosv2`

## 1. Purpose

This document defines how child agents operate inside the TuringOSv2 development workflow.

Child agents are execution tools, not final authorities.

They exist to:

- accelerate bounded work
- isolate concerns
- preserve auditability
- reduce scope drift

They do not exist to:

- redefine scope
- integrate final state
- make git or release decisions

## 2. Codex CLI Integration Boundary

Child roles in this repo are project-scoped.

Therefore:

- do not place TuringOSv2-specific child roles in `~/.codex/config.toml`
- keep project roles in `.codex/config.toml`
- keep role instructions in `.codex/agents/*.toml`
- keep full human-readable role specs in `handover/ops/roles/*.md`

The governance source remains the handover docs. The `.codex/*.toml` files are CLI wiring.

## 3. Gemini CLI Integration Boundary

The following roles are not Codex child roles:

- `turing_agi_architect_auditor`
- `groundhog_math_constitution_custodian`
- `groundhog_formal_methods_auditor`
- `groundhog_turing_machine_theorist`
- `groundhog_recursive_math_auditor`

They are external, read-only audit roles invoked through Gemini CLI.

For Mission Groundhog, any participating Gemini role is pinned to `Gemini 3.1 Pro Preview`, and the CLI invocation must set `-m` explicitly.

Canonical behavior is defined in:

- `handover/ops/ROLE_CATALOG.md`
- `handover/ops/roles/TURING_AGI_ARCHITECT_AUDITOR.md`
- `handover/ops/roles/GROUNDHOG_MATH_CONSTITUTION_CUSTODIAN.md`
- `handover/ops/roles/GROUNDHOG_FORMAL_METHODS_AUDITOR.md`
- `handover/ops/roles/GROUNDHOG_TURING_MACHINE_THEORIST.md`
- `handover/ops/roles/GROUNDHOG_RECURSIVE_MATH_AUDITOR.md`

## 4. Required Child Packet

Every delegated child must receive:

1. mission name or mission id
2. role name and engine
3. bounded objective
4. canonical spec
5. in-scope files or evidence, including the relevant historical success/failure corpus when the task is a debug, promotion, or rerun decision
6. out-of-scope files or forbidden actions
7. allowed actions
8. required output
9. stop conditions
10. test, runtime, or evidence constraints if they matter

If any item is missing, the child must default to:

- read-only behavior
- no git
- no handover writes

## 5. Authority Model

Child agents inherit task context, not Commander authority.

### Child agents may

- read assigned files
- inspect assigned evidence
- edit assigned files when explicitly authorized
- produce plans, findings, or research summaries
- monitor long-running jobs when explicitly assigned

### Child agents may not

- widen their own scope
- edit files outside assigned ownership
- update handover unless explicitly assigned for documentation-only work
- commit
- push
- overrule the mission charter

## 6. Role Profiles

### 6.1 Plan Child

Mission:

- produce a bounded plan or change map

Output must contain:

- in-scope files
- out-of-scope files
- acceptance gates
- risks
- open assumptions

### 6.2 Coder Child

Mission:

- implement only the assigned change set

Must return:

- exact files changed
- summary of what changed
- unresolved risks

### 6.3 Rust Systems Architect Child

Mission:

- design Rust structure without taking implementation ownership

Must return:

- crate and module map
- API boundary findings
- migration risks
- explicit separation of proven facts, inferences, and open risks

### 6.4 Rust Coder Child

Mission:

- implement only the assigned Rust change set

Must return:

- exact files changed
- summary of what changed
- unresolved risks
- explicit separation of proven facts, inferences, and open risks

### 6.5 Turing-AGI Architect Auditor Child

Mission:

- audit top-level Turing-machine and AGI direction using `bible/`

Must return:

- verdict: `PASS`, `PASS WITH FIXES`, or `BLOCK`
- findings
- AGI direction drift risk
- explicit separation of proven facts, inferences, and open risks

### 6.6 Groundhog Math Constitution Custodian Child

Mission:

- protect the sealed Groundhog theorem and topology

Must return:

- verdict: `PASS`, `PASS WITH FIXES`, or `BLOCK`
- theorem-preservation findings
- forbidden drift points
- explicit separation of proven facts, inferences, and open risks

### 6.7 Groundhog Formal Methods Auditor Child

Mission:

- audit derived invariants and proof obligations

Must return:

- verdict: `PASS`, `PASS WITH FIXES`, or `BLOCK`
- invariant findings
- missing proof obligations
- explicit separation of proven facts, inferences, and open risks

### 6.8 Groundhog Turing Machine Theorist Child

Mission:

- map the sealed theorem onto tape, head, and register semantics

Must return:

- machine-semantics map
- ambiguity findings
- explicit separation of proven facts, inferences, and open risks

### 6.9 Groundhog Recursive Math Auditor Child

Mission:

- perform the final recursive mathematical audit for a Groundhog stage

Must return:

- verdict: `PASS`, `PASS WITH FIXES`, or `BLOCK`
- recursive findings
- missed contradiction or drift findings
- explicit separation of proven facts, inferences, and open risks

### 6.10 OS Systems Planner Child

Mission:

- audit structure using OS and Linux design principles

Must return:

- verdict: `PASS`, `PASS WITH FIXES`, or `BLOCK`
- findings
- boundary and failure-mode risks
- Linux and OS best-practice risks
- explicit separation of proven facts, inferences, and open risks

### 6.11 Git Historian Child

Mission:

- preserve provenance, sealed-file discipline, and migration sequencing

Must return:

- provenance findings
- commit-slicing guidance
- migration risks
- explicit separation of proven facts, inferences, and open risks

### 6.12 Karpathy Editor Child

Mission:

- simplify and sharpen code without changing the theorem or widening scope

Must return:

- simplification findings
- naming/API cleanup recommendations
- explicit separation of proven facts, inferences, and open risks

### 6.13 Recursive Software Auditor Child

Mission:

- perform the final recursive software-engineering audit for a Groundhog stage

Must return:

- verdict: `PASS`, `PASS WITH FIXES`, or `BLOCK`
- recursive findings
- unresolved cross-audit tensions
- stage-close software risks
- explicit separation of proven facts, inferences, and open risks

### 6.14 Quality Auditor Child

Mission:

- audit test adequacy and regression risk

Must return:

- verdict: `PASS`, `PASS WITH FIXES`, or `BLOCK`
- findings
- rollback risk
- runtime or CLI contract risk
- explicit separation of proven facts, inferences, and open risks

### 6.15 Research Child

Mission:

- answer an external fact question needed for implementation

Must return:

- sourced facts
- unresolved questions
- integration risks

### 6.16 Runtime Watcher Child

Mission:

- monitor a long-running task

Must return:

- current health status
- latest observed progress
- ETA if possible
- stop-condition alerts

### 6.17 Goal Poller Child

Mission:

- supervise explicit targets through artifact polling and compressed success/failure evidence

Must return:

- current target and rung
- current best verified ceiling
- compressed historical success/failure basis
- verdict: `ADVANCE`, `HOLD`, `RERUN_SAME_RUNG`, or `STOP_AND_DEBUG`
- exact evidence used for that verdict
- anti-hardcoding check
- explicit separation of proven facts, inferences, and open risks

## 7. Delegation Discipline

The Commander should delegate in narrow slices.

Preferred slices:

- one document set
- one file ownership block
- one audit pass
- one research question
- one run-watch assignment

Avoid:

- overlapping writable ownership
- mixed implementation and audit in the same child
- unbounded repo-wide exploration

## 8. Stop Conditions

Child agents must stop and escalate on:

- scope drift
- ambiguous canonical spec
- dirty-tree conflict
- file ownership conflict
- missing required context
- evidence that the task actually needs a different role

Escalation must state:

- what was attempted
- what blocked
- what exact decision is needed

## 9. Output Contract

Every child output must separate:

- proven facts
- inferences
- open risks
- evidence basis when the child is recommending a debug, rerun, promotion, or advance decision

Coder children must also list:

- exact files changed

Auditor children must also list:

- verdict: `PASS`, `PASS WITH FIXES`, or `BLOCK`

Research children should also list:

- sources consulted

Watcher children must also list:

- current health
- latest progress
- whether any stop condition triggered

## 10. Default Child Packet Template

```text
Mission:
- <mission name>

Role:
- <turingos_plan | turingos_coder | turingos_rust_systems_architect | turingos_rust_coder | turing_agi_architect_auditor | groundhog_math_constitution_custodian | groundhog_formal_methods_auditor | groundhog_turing_machine_theorist | groundhog_recursive_math_auditor | turingos_os_system_planner | turingos_git_historian | turingos_karpathy_editor | turingos_recursive_software_auditor | turingos_quality_auditor | turingos_researcher | turingos_runtime_watcher | turingos_goal_poller>

Engine:
- <codex child role id | gemini -y -m "<explicit model>">

Objective:
- <one bounded objective>

Canonical spec:
- <task-level canonical spec>

In scope:
- <paths or evidence>

Forbidden:
- <paths/actions>

Allowed actions:
- <read / edit / audit / research / monitor>

Required output:
- <format>

Constraints:
- <tests, runtime, or evidence preservation rules>

Stop conditions:
- <explicit escalation conditions>
```
