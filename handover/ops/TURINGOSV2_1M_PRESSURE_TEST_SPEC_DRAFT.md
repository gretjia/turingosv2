# TuringOSv2 1M Pressure Test Spec

Status: Draft
Date: 2026-03-10 UTC
Scope: Current-level live-model measurement for `turingosv2` without kernel modification

## 1. Purpose

- Measure what the current `turingosv2` system can actually sustain under the existing runtime, not what the raw models can do outside the system.
- Reuse the historical `1M test` as a measurement discipline only.
- Preserve enough raw run evidence to make later upgrades auditable against the exact failures and drifts observed here.
- Treat `100` consecutive headline runtime passes as the first-stage target, not as a terminal project goal.
- Treat upgrade work as a data-compression problem over historical success and failure evidence, not as an expert-opinion contest.

## 2. Authority Order

Primary authority:

- `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
- `handover/ops/MISSION_002_1M_TEST_SPEC.md`
- `handover/ops/MISSION_002_WHITEBOX_RULES.md`
- `handover/ops/MISSION_002_FAILURE_LEARNING.md`
- `handover/ops/TURINGOSV2_SELF_UPGRADE_INNER_PROPOSER_SPEC_DRAFT.md`
- `handover/ops/TURINGOSV2_SELF_UPGRADE_OUTER_PROMOTION_SPEC_DRAFT.md`
- `turingos/tasks/parity.py`
- `harnesses/mission002_py/mission002.py`
- `harnesses/mission002_py/parity_cases.py`
- `harnesses/mission002_py/llama_openai.py`

Learning-only historical references:

- `/home/zephryj/projects/turingos/src/bench/million-baseline-compare.ts`
- `/home/zephryj/projects/turingos/handover/audits/modelcompare/main_steps_baseline_20260228.md`

Conflict rule:

- If the old `turingos` benchmark method conflicts with the current `turingosv2` kernel/task boundary, `turingosv2` wins.
- The old repository contributes measurement discipline only, not runtime semantics, recovery hacks, or protocol debt.

## 3. Benchmark Question

- For the current `turingosv2` parity task and current proposal contract, how many deterministic cases can each official mode complete before the first semantic failure?
- Where is the first semantic break:
  - parse layer
  - proposal schema
  - verifier rejection
  - register drift
  - path drift
  - write drift
  - task-level wrong result
  - max-step exhaustion
- When failure happens, what exact raw evidence best explains the drift and supports later upgrades?

## 4. Historical Rules Preserved

- One shared deterministic case stream across all measured modes.
- One primary score: `consecutive_pass_before_first_fail`.
- One stop rule for official scoring: first semantic case failure ends the scored run for that mode.
- One first-fail artifact chain that preserves enough evidence for later audit and replay.

## 5. Historical Debt Rejected

- No arithmetic-only chat benchmark as the main test object.
- No reuse of old concept-version runtime behavior or capability semantics.
- No benchmark-local repair logic that silently makes the system look better than it is.
- No direct model baseline rerun merely for comparison. The user already knows the standalone model baselines.

## 6. Official Benchmark Object

- The official workload is the current parity task under `turingos/tasks/parity.py`.
- The official generator is the current deterministic parity case generator under `harnesses/mission002_py/parity_cases.py`.
- Case identity is fixed by:
  - `generator_version`
  - `global_seed`
  - `case_index`
- Each case is one fresh workspace tree plus one full unchanged `TuringOSKernel` run.

## 7. Official Modes

Primary current-level mode:

- `turingosv2_planner27_worker9`
  - planner lane on `Qwen 3.5 27B`
  - worker lane on `Qwen 3.5 9B`
  - same public proposal boundary for both lanes

Diagnostic sidecar modes:

- `turingosv2_single_27b`
- `turingosv2_single_9b`

Important boundary:

- These are `turingosv2` runtime modes, not standalone raw-model baselines.
- Historical direct-model baselines remain reference values only and are not rerun by this project.

## 8. Runtime and Proposal Constraints

- `explore_epsilon = 0`
- one fixed `max_steps` budget per official run
- one fixed retry budget for infra faults
- one fixed prompt contract version per official run
- canonical proposal keys remain:
  - `next_register`
  - `next_path`
  - `write_mode`
  - `write_content`
  - `halt`
  - `notes`
- adapters may normalize trivial transport shape only
- adapters may not infer missing semantics or repair a model proposal
- kernel semantics, task semantics, verifier semantics, and pricing behavior may not be changed for this project

Initial official run constants:

- `generator_version = mission002.parity.v1`
- `max_steps = 64`
- `retry_budget = 1`
- `prompt_version` must be declared in each run manifest and may change only through explicit non-kernel benchmark revisions

If those constants must change for legitimacy, the new constant set must be declared before the official scored run and recorded in the manifest.

## 9. Execution Topology

- `omega-vm` remains the only source-of-truth repository workspace.
- The Tailscale Mac node provides model serving and is currently known to have `32G` memory.
- `windows1` may be used as an auxiliary execution or model-serving node if later pressure requires more concurrent worker capacity.
- Preferred path:
  - run the benchmark from `omega-vm`
  - access Mac-hosted `llama.cpp` through SSH local forwards
- Preferred first-wave operating mode:
  - keep official scored runs low-concurrency and mode-serial
  - do not assume the Mac should carry multiple heavy measured lanes at once
- Approved fallback:
  - if tunnel behavior contaminates the measured run, use one revision-pinned non-git runner bundle on the Mac
  - that fallback still may not become a second shadow repository
- Approved scale-out option:
  - if later measured phases need extra worker concurrency or topology isolation, `windows1` may host auxiliary runtime load
  - any such use must be declared explicitly in the run manifest and must not create a second source-of-truth repo
- The run manifest must record:
  - Mac host alias
  - whether `windows1` participated
  - declared local forwarded ports
  - declared remote ports
  - declared model ids
  - `/v1/models` probe results
  - prompt version
  - repo revision or runner-bundle digest
  - any manually supplied launch flags known at run time

Hard boundary:

- no second git-managed `turingosv2` checkout on the Mac
- no second git-managed `turingosv2` checkout on `windows1`
- no kernel changes to make the benchmark easier
- no watchdog or supervisor daemon; any long run monitoring must remain an agent-level activity

## 10.1 AgentOS Supervision

- Commander owns final go/no-go decisions and code integration.
- `turingos_goal_poller` owns target polling for the explicit ladder and first-stage goal of `100` consecutive headline passes.
- `turingos_runtime_watcher` owns live process-health observation only.
- The current pressure-test project now also uses an explicit inner/outer split for bounded self-improvement:
  - inner proposer law: `handover/ops/TURINGOSV2_SELF_UPGRADE_INNER_PROPOSER_SPEC_DRAFT.md`
  - outer promotion law: `handover/ops/TURINGOSV2_SELF_UPGRADE_OUTER_PROMOTION_SPEC_DRAFT.md`
- `100` consecutive passes is an outer promotion gate, not an inner whitebox theorem.
- The goal poller must never be implemented as a resident program or daemon; it runs as an agent role over artifacts and process state.
- AgentOS is responsible for helping `turingosv2` itself grow bounded project intelligence.
- AgentOS models are intelligent expert participants, but their proper role in this project is scaffolding, critique, audit, evidence compression, and promotion rather than permanent substitution for `turingosv2`'s own inner proposer.
- The goal poller may issue only these verdicts:
  - `ADVANCE`
  - `HOLD`
  - `RERUN_SAME_RUNG`
  - `STOP_AND_DEBUG`
- The goal poller may issue `ADVANCE` only when:
  - the current rung has exact `PASS`
  - required artifacts exist
  - no anti-hardcoding violation is detected
  - the Commander agrees that the next rung stays within the current bounded scope

## 10. Gate Ladder

Phase A: topology preflight

- verify SSH reachability
- verify tunnel mapping
- verify `/v1/models`
- verify output root and manifest path

Phase B: smoke gates

- `1`
- `3`
- `10`

Phase C: staircase gates

- `25`
- `50`
- `100`
- `250`
- `500`

Phase D: long-run gates

- `1_000`
- `2_500`
- `5_000`
- `10_000`
- then continue upward only if the previous rung passed cleanly

Gate rule:

- after each completed rung, write artifacts and a short retrospective
- only advance if the current rung is `PASS`
- if the current rung ends in semantic failure, stop scored progression and preserve the first-fail chain
- if the current rung ends in infra blockage, preserve blocker evidence and rerun only after infra state is explicitly re-established
- the first-stage target is satisfied only when the headline mode `turingosv2_planner27_worker9` reaches exact `100` consecutive passes under one declared run configuration

## 10.2 Self-Upgrade Reading

- The pressure-test project may let `turingosv2` participate in bounded self-improvement work.
- This must be read as:
  - inner loop proposes bounded non-kernel revisions
  - outer AgentOS loop evaluates and promotes or rejects them
  - the outer AgentOS loop must base those decisions on recorded rung evidence, compressed failure/success patterns, and anti-hardcoding review, not on unaudited expert preference
- AgentOS is therefore the whitebox scaffolding and promotion layer, not the intended long-term replacement for `turingosv2`'s own bounded project intelligence
- If the outer loop directly authors too many fixes for too long, that must be recorded as bootstrap debt against the intended architecture.
- This must not be read as:
  - inner runtime redefining pass/fail truth
  - inner runtime auto-promoting its own revisions
  - benchmark score becoming whitebox law

## 11. Scoring

Primary score for each mode:

- `consecutive_pass_before_first_fail`

Secondary scores:

- highest completed gate
- per-case step count
- selected-agent sequence
- price ranking
- public broadcasts
- task diagnostics
- ledger integrity status
- failure class
- first semantic drift step

Interpretation rule:

- the primary reported project-level score is the current `turingosv2_planner27_worker9` score
- single-model runtime lanes are diagnostic decompositions, not the headline system score

## 12. Failure Classes

Semantic failure:

- parse failure after bounded infra retry
- schema-invalid proposal
- verifier rejection leading to case failure
- wrong terminal result
- wrong halt behavior
- max-step exhaustion

Infra blocker:

- model service unreachable
- tunnel failure
- transport timeout
- malformed provider payload that cannot be attributed to task semantics

Accounting rule:

- infra blockers are recorded separately from semantic failure counts
- official scored runs stop on first semantic failure, not on mere availability noise if the bounded retry budget recovers

## 13. Required Artifacts

Per run:

- `run_manifest.json`
- `overall_summary.json`
- `overall_summary.md`
- exact command manifest
- endpoint and model probe manifest
- topology manifest
- prompt/version manifest
- repo revision or runner-bundle digest

Per mode:

- `summary.json`
- `summary.md`
- gate retrospective note

Per passing case:

- `case_result.json`
- selected agents
- task diagnostics
- terminal state summary

Per sampled passing checkpoint:

- sampled raw request and response trace
- sampled ledger tail

Per non-pass case:

- `case_result.json`
- workspace snapshot
- full ledger copy when available
- ledger tail
- raw request payload
- raw model response
- parse or infra error if present
- full agent traces

First non-pass chain:

- first non-pass case artifact
- failure classification
- first-drift explanation in retrospective notes

## 14. Learning Objectives

- Identify the earliest step where current `turingosv2` runtime use diverges from the task protocol.
- Separate raw-model weakness from adapter/protocol weakness and from infra weakness.
- Build an append-only evidence chain that later prompt, adapter, and runtime missions can cite without hand-waving.
- Preserve enough operational metadata that future investigators know not only what failed, but under which topology and service configuration it failed.

## 15. Explicit Non-Goals

- rerunning historical standalone model baselines
- changing kernel semantics
- changing task truth conditions
- adding case-specific hard encoding just to pass benchmark gates
- silently widening the benchmark object beyond parity during this project
- using hidden repair models or evaluator models
- compressing away raw failure evidence in the name of cleaner summaries

Hard anti-cheating rule:

- No prompt, adapter, or harness change may hard-code expected paths, expected register states, expected answers, fixed case-specific todo lists, or any other case-local shortcut merely to pass the current benchmark corpus.
- Every accepted improvement must aim at better protocol-following and better real runtime generalization under the unchanged kernel and unchanged topology.

## 16. Immediate Execution Plan

The first execution wave for this project should be:

1. Preflight the Mac topology and model endpoints.
2. Run the smoke ladder for:
   - `turingosv2_planner27_worker9`
   - `turingosv2_single_27b`
   - `turingosv2_single_9b`
3. If a mode fails before clearing `10`, stop that mode and preserve the first-fail chain.
4. If a mode clears `10`, continue it through the staircase ladder.
5. After the first full execution wave, write a learning record that compares:
   - current scored depth
   - first failure class
   - first failure step
   - sidecar versus primary mode behavior

## 17. Success Condition For This Spec

- The project produces one trustworthy answer to: "what level is current `turingosv2` at right now under its real runtime boundary?"
- The answer is backed by raw artifacts, not by anecdotal prompts or selectively quoted traces.
