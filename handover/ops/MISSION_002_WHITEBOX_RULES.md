# Mission 002 Top-Level Whitebox Rules

Status: Active design basis for the `turingosv2-1M test` project
Derived from: `handover/ops/MISSION_002_1M_TEST_SPEC.md`

## 1. Authority Rule

- The benchmark whitebox layer is subordinate to the current `turingosv2` runtime contract.
- Task semantics are authoritative only through the current task and runtime interfaces:
  - `Task.setup`
  - `Task.initial_state`
  - `Task.expected_transition`
  - `Task.verify_proposal`
  - `Task.is_success`
  - `Task.diagnostics`
  - `TuringOSKernel`
- The whitebox layer may package runs and enforce reporting rules, but it may not reinterpret task truth conditions.

## 2. Deterministic Case-Stream Rule

- Every benchmark case must be reproducible from declared public inputs only.
- The case stream is determined by `(generator_version, global_seed, case_index)`.
- All official modes must consume the same ordered case stream.
- Difficulty adaptation during an official run is forbidden.

## 3. Single Source-of-Truth Rule

- `omega-vm` is the authoritative benchmark workspace and repository root.
- The Mac node is a model-serving dependency, not a second development workspace.
- If a Mac-local runner bundle is ever needed, it must be a minimal copied execution bundle in a non-git directory.

## 4. Proposal Envelope Rule

- The benchmark accepts only a machine-parseable proposal envelope.
- The canonical proposal must contain:
  - `next_register`
  - `next_path`
  - `write_mode`
  - `write_content`
  - `halt`
  - `notes`
- Deterministic normalization is allowed only for trivial transport formatting, such as removing one surrounding code fence.
- Semantic repair is forbidden:
  - no field inference
  - no hidden defaults that change task meaning
  - no auto-fixing malformed paths
  - no auto-completing missing register fields from task internals

## 5. No Hidden State-Synthesis Rule

- The adapter may not call `expected_transition` or any equivalent helper in order to fill missing model output.
- The adapter may not use a second model to repair, rewrite, or complete a first model's proposal under any condition.
- The adapter may not use task-specific shortcuts that bypass the model's need to produce the current transition.

## 6. Whitebox Pricing Rule

- The top layer only does whitebox pricing, not blackbox valuation.
- Allowed top-layer actions:
  - schema validation
  - verifier execution
  - signal accounting
  - price accounting
  - deterministic scheduler control
  - artifact logging
- Forbidden top-layer actions:
  - evaluator-LLM scoring
  - opaque “reasoning quality” judgments
  - hidden human preference ranking
  - hidden rescue logic for “almost right” outputs

## 7. Equal-Interface Rule

- All agents in a benchmark mode must enter through the same `AgentView` / `AgentProposal` boundary.
- Role differences such as planner versus worker may exist only through declared prompting, model choice, and public mode configuration.
- No agent may receive privileged hidden state, privileged verifier outputs, or undeclared side channels.

## 8. Deterministic Runtime-Config Rule

- Official benchmark runs must set `explore_epsilon = 0`.
- Official benchmark runs must declare one fixed `max_steps` value for the run.
- Termination conditions are fixed and public:
  - success
  - first semantic case failure
  - blocked-by-infra abort
  - manually declared stop gate for smoke runs

## 9. Failure-Accounting Rule

- Proposal-level hard fails, case-level semantic fails, and infra blockers must be recorded as different classes.
- A proposal hard fail from one agent does not automatically fail the case if another valid proposal can still be selected under the declared mode.
- A case semantically fails at the first case where the run no longer satisfies the benchmark success condition.
- Infra faults may consume a small bounded retry budget, but once that budget is exhausted the run must be marked `BLOCKED_BY_INFRA`, not silently retried forever.

## 10. Artifact-Completeness Rule

- Every official run must emit a public artifact set that is sufficient for later audit.
- The minimum artifact set is:
  - run manifest
  - mode configuration
  - model and endpoint manifest
  - case range and seed information
  - summary report
  - first-fail artifact or blocker evidence
  - workspace snapshot for the relevant failure or blocker
  - ledger tail
  - selected-agent trace

## 11. Audit-Trail Rule

- The benchmark must preserve the whitebox-visible evidence already exposed by the runtime where possible:
  - selected agents
  - score and price effects
  - public broadcasts
  - task diagnostics
  - ledger integrity result
- Official benchmark packaging may summarize these fields, but it may not silently discard them.

## 12. Mission-002 Purity Rule

- Mission 002 is a baseline mission, not a repair mission.
- The benchmark may add harnesses, adapters, CLI entrypoints, tests, and handover docs.
- The benchmark may not change kernel semantics to make models look better.
- The benchmark may not introduce a new task family and still call the result the “pure baseline” for current `turingosv2`.

## 13. Historical-Continuity Rule

- The old `1M test` remains the source of measurement discipline only.
- Historical direct-model lanes may be useful as sidecar comparisons, but they are outside the authority boundary for Mission 002 whitebox design.
- Mission 002 whitebox design is therefore anchored to the current `turingosv2` operating-system-style task interface, not to the old concept-version arithmetic prompt format.
