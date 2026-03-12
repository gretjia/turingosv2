# Mission 002 1M Test Spec

Status: Confirmed by user on 2026-03-08 UTC
Mission: Mission 002 - TuringOSv2 pure-baseline 1M benchmark with Mac llama.cpp

## 1. Purpose

- Use the current `turingosv2` codebase as the first pure baseline derived from `bible/`, before any benchmark-driven kernel repair or optimization.
- Learn the measurement discipline of the historical `1M test`, but do not import the concept-version runtime debt into `turingosv2`.
- Keep the historical `1,000,000` target as the comparison scale while treating Mission 002 as the baseline-construction and first real-run phase.

## 2. What Is Preserved From the Historical 1M Test

- One shared case stream across all tested modes.
- One stop rule: stop on first semantic case failure.
- One primary metric: consecutive passed cases before first failure.
- One first-fail artifact chain for auditability.

## 3. What Intentionally Changes For TuringOSv2

- The benchmark object is no longer a one-shot arithmetic chat prompt.
- The benchmark object is the current `turingosv2` operating-system-style task execution path.
- Mission 002 uses the existing parity task family as the first pure workload instead of inventing a new task protocol.
- Historical direct-model baselines may be added later as sidecar comparisons, but they are not the authority for Mission 002 whitebox design.

## 4. Benchmark Object

- The authoritative task family for Mission 002 is the existing parity task implemented in `turingos/tasks/parity.py`.
- Each case is a fresh deterministic workspace tree plus one full `TuringOSKernel` run.
- Case variety comes from the generated parity tree only: directory layout, numeric file distribution, and seed-derived content.
- Mission 002 does not change kernel semantics, task semantics, or verifier semantics in order to make the benchmark easier.

## 5. Case Definition

- A case is identified by a stable case index and a deterministic generator version.
- The case generator must map `(global_seed, case_index, generator_version)` to exactly one parity tree.
- All modes must receive the exact same ordered case sequence.
- Official benchmark runs must use one fixed `max_steps` budget per run and set `explore_epsilon = 0`.
- Mission 002 smoke gates are `1`, `3`, and `10` cases before any longer run is attempted.

## 6. Agent Contract

- Models only participate through the existing `Agent.propose(view) -> AgentProposal` boundary.
- The raw model output must represent the full transition semantics required by the current runtime:
  - `next_register`
  - `next_path`
  - `write_mode`
  - `write_content`
  - `halt`
  - `notes`
- Adapters may translate protocol format, but they may not silently infer or synthesize missing transition fields from task internals.
- Mission 002 is a pure baseline, so there is no hidden auto-correction, no shadow planner outside the declared mode, and no semantic repair pass.

## 7. Core Benchmark Modes

- `turingosv2_single_27b`
  - one agent
  - model: `Qwen 3.5 27B`
- `turingosv2_single_9b`
  - one agent
  - model: `Qwen 3.5 9B`
- `turingosv2_planner27_worker9`
  - two agents under the same unchanged kernel
  - planner prompt on `Qwen 3.5 27B`
  - worker prompt on `Qwen 3.5 9B`
  - role separation may exist in prompt design, but not through privileged hidden interfaces

## 8. Execution Topology

- `omega-vm` is the only source-of-truth repository workspace.
- The Mac node provides model serving only.
- The default topology is `omega-vm` runner plus SSH tunnel to the Mac-hosted `llama.cpp` services.
- No second full git-managed `turingosv2` checkout is allowed on the Mac.
- A dedicated non-git minimal runner bundle on the Mac is allowed only if smoke runs show the tunnel topology is too unstable for valid benchmark execution.

## 9. Measurement

- Primary metric:
  - `consecutive_pass_before_first_fail`
- Secondary metrics:
  - per-case step count
  - selected-agent sequence
  - price ranking
  - public broadcasts
  - task diagnostics
  - ledger integrity status
  - first-fail artifact
- The historical `1,000,000` number remains the target scale, even if Mission 002 only reaches smoke or limited long-run depths.

## 10. Failure Accounting

- Proposal hard fail:
  - raw output cannot be parsed into the declared proposal envelope
  - schema-invalid transition
  - verifier-rejected transition
  - path escape or other boundary violation
- Case fail:
  - no valid proposal can be selected
  - wrong terminal result
  - incorrect halt behavior
  - `max_steps` exhausted without success
- `BLOCKED_BY_INFRA`:
  - model service unreachable after bounded retry
  - tunnel failure
  - transport or timeout failure not attributable to task semantics
- Infra blockers are recorded separately and do not get merged into semantic fail counts.

## 11. Required Artifacts

- run manifest with model ids, endpoint topology, generator version, seed, and config
- summary report
- first-fail artifact when a semantic failure occurs
- workspace snapshot for the failing case
- ledger tail and selected-agent trace
- exact command manifest for reproducibility

## 12. Non-Goals

- changing `turingosv2` kernel semantics to improve benchmark outcomes
- creating a new task family for Mission 002
- maintaining a second shadow repository on the Mac
- using an evaluator LLM to score proposal quality
- replacing whitebox pricing with blackbox valuation
