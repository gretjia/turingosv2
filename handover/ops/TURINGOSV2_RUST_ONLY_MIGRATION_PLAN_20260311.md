# TuringOSv2 Rust-Only Migration Plan

Status: Audited proposal
Date: 2026-03-11 UTC
Scope:
- detailed plan for retiring the Python runtime/harness path and converging to a Rust-only white-box embodiment with external LLM APIs as black-box `delta`
- parity checks and real-world task execution are treated as external harnesses, not as theorem-bearing `turingosv2` repository core

## 1. Executive Verdict

- Long-term target: **yes**
  - `turingosv2` should converge to an extremely small Rust-owned white-box core.
  - External LLM APIs should remain outside as black-box proposal generators only.
- Immediate full deletion of Python: **no**
  - Python still carries live theorem-adjacent responsibilities that Rust does not yet fully replace.
- Team-audited recommendation:
  - stop expanding repository Python's theorem-bearing surface now
  - migrate theorem-bearing functionality to Rust in bounded stages
  - retire repository Python only after Rust owns the runtime/provider/operator truth end to end
  - keep parity and real-world harnesses outside the theorem-bearing repo core

## 2. Highest Authorities

1. `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
2. the sealed Final Clockwork Edition:
   - `Human_Architect --once--> law`
   - `initAI(law) --once--> predicate matrix and map-reduce`
   - `clock -> mr_map -> rtool -> input -> delta -> output -> product predicates`
   - `product predicates = 1 -> mr_reduce ∘ wtool(output | tape_t, HEAD_t, tools_other) -> Q_t+1`
   - `product predicates = 0 -> exact prior Q_t`
   - `if q_t+1 == halt -> HALT`
3. `handover/ops/MISSION_GROUNDHOG_PROJECT_SPEC.md`
4. `handover/ops/MISSION_GROUNDHOG_IMPLEMENTATION_GUARDRAILS.md`

## 3. Team Audit Summary

### 3.1 groundhog_turing_machine_theorist

Verdict:
- `PASS WITH STAGED MIGRATION`

Findings:
- A Rust-only white-box owner is topologically cleaner than the current Rust+Python split.
- The constitution permits external APIs to act as `delta`, but they must never own `Q_t`, predicates, or `wtool`.
- Required in Rust:
  - closed `Q_t = <q_t, HEAD_t, tape_t>`
  - once-only constitutional initialization outputs
    - compiled predicate matrix
    - compiled `mr` discipline
  - runtime `clock` ownership
  - `rtool`
  - explicit `mr_map`
  - predicate-product gate
  - atomic `wtool`
  - explicit `mr_reduce`
  - lawful execution of `tools_other` through the bottom write path only
  - exact abort to old `Q_t`
  - explicit finalization and `HALT`

### 3.2 groundhog_math_constitution_custodian

Verdict:
- `PASS WITH CONSTITUTIONAL CONDITIONS`

Findings:
- A minimal Rust-only white-box embodiment is mathematically preferable because it reduces theorem-bearing owners to one.
- External transport/provider faults are acceptable only if they remain outside commit authority.
- Side channels must not widen theorem-bearing input beyond the sealed `q_t + rtool(tape_t, HEAD_t)` pair.

### 3.3 turingos_rust_systems_architect

Verdict:
- `PASS WITH STAGED EXECUTION`

Findings:
- The final shape should be:
  - Rust kernel and snapshot model
  - Rust adapter boundary and provider shims
  - Rust CLI/operator entry
- Repository Python should not be deleted until Rust fully replaces:
  - runtime
  - provider adapter surface
  - operator authority

## 4. Current Reality: Why Python Still Exists

Today repository Python still owns real repository behavior in:

- reference runtime
  - `turingos/runtime.py`
- task protocol and parity task
  - `turingos/tasks/base.py`
  - `turingos/tasks/parity.py`
- provider adapter and prompt contract
  - `harnesses/mission002_py/llama_openai.py`
  - `harnesses/mission002_py/mission002_prompting.py`
- benchmark harness and artifact production
  - `harnesses/mission002_py/mission002.py`
  - `harnesses/mission002_py/parity_cases.py`
- CLI glue
  - `turingos/cli.py`

Rust already owns the core constitutional skeleton in:

- `crates/turingos-core/`
- `crates/turingos-kernel/`
- `crates/turingos-task/`
- `crates/turingos-adapter/`
- `crates/turingos-observe/`

## 5. Non-Negotiable Migration Rules

- Do not delete repository Python before Rust replaces the same theorem-bearing responsibility.
- Do not outsource any of the following:
  - `Q_t`
  - `rtool`
  - predicates
  - `wtool`
  - commit / abort authority
- External LLM APIs may only do:
  - receive serialized `Input_t`
  - return suspended `⟨q_o, a_o⟩`
- Runtime ownership of `clock`, `mr_map`, `mr_reduce`, and `tools_other` execution must remain on the WHITEBOX side.
- No migration step may widen black-box input or add hidden repair paths.
- Repository Python retirement must reduce theorem-bearing duplication, not create a Rust wrapper around Python truth.
- External harness Python is allowed, but it must stay disposable and out of the theorem-bearing repository core.

## 6. Target End State

- One Rust-owned white-box embodiment of:
  - `Q_t`
  - one-time constitutional initialization output
    - predicate matrix
    - map-reduce discipline
  - runtime clock ownership
  - `rtool`
  - predicate gate
  - `wtool`
  - controlled invocation of `tools_other`
  - run/report/observe surfaces
- External LLM providers treated as replaceable black-box transports only.
- Benchmark and real-world test harnesses driven by Rust surfaces from outside the core.
- Repository Python removed from theorem-bearing runtime paths.

## 7. Detailed Migration Stages

### Stage 0. Freeze Python Expansion

Goal:
- stop adding new theorem-bearing logic to Python

Actions:
- Rust-first rule remains mandatory
- repository Python is limited to bootstrap debt, compatibility shims, and temporary research utility
- external Python harnesses remain allowed but stay outside repository authority

Exit criteria:
- no new core behavior lands first in Python

### Stage 1. Complete Python Constitution Closure

Goal:
- finish making the current Python path constitution-safe while it still exists

Actions:
- keep sealing drift such as split-`Q_t` and any widened black-box input
- ensure Python remains fail-closed during the transition period

Exit criteria:
- no known constitution-level `FIX` remains in the Python path

### Stage 2. Remove repository dependence on Python task truth

Goal:
- stop letting in-repo Python task code act as the de facto authority path

Actions:
- keep current parity Python only as bootstrap debt and compatibility evidence
- move any theorem-bearing runtime truth needed by the kernel into Rust-owned surfaces
- avoid promoting parity-specific harness logic into the Rust core

Exit criteria:
- no repository Python task file remains on the theorem-bearing runtime authority path

### Stage 3. Port Live Runtime Path to Rust

Goal:
- make Rust the live theorem-bearing runtime

Actions:
- drive live steps from `turingos-kernel`
- keep `Q_t` closed in Rust
- ensure abort returns exact old snapshot

Exit criteria:
- one live run can be executed end to end without repository Python runtime authority

### Stage 4. Port Provider Boundary to Rust

Goal:
- remove Python from black-box provider invocation on the live path

Actions:
- implement Rust provider adapters for:
  - local OpenAI-compatible `llama.cpp`
  - external Kimi Coding / other HTTP providers as needed
- keep provider outputs serialized and fail-closed

Exit criteria:
- Rust can call live providers directly for bounded runs

### Stage 5. Keep harnesses outside the core and expose Rust operator surfaces

Goal:
- make the Rust core invokable by disposable external harnesses instead of absorbing those harnesses

Actions:
- expose stable run/replay/report entrypoints
- preserve current evidence chain shape
- allow parity and real-world tasks to be driven by external Python without making that Python repository truth

Exit criteria:
- official runs can be launched without repository Python harness code

### Stage 6. Port CLI and Operator Surface to Rust

Goal:
- remove repository Python as the operator entrypoint

Actions:
- Rust CLI for:
  - bounded rerun
  - scored rung run
  - artifact export
  - replay / validation commands

Exit criteria:
- no theorem-bearing operator workflow depends on `turingos/cli.py`

### Stage 7. Side-by-Side Constitutional Equivalence Window

Goal:
- prove Rust replacement matches or improves the repository Python path before deletion

Actions:
- run matched external task harnesses through both paths
- compare:
  - pass/fail
  - first-fail fingerprints
  - run artifacts
  - replay/export summaries

Exit criteria:
- Rust path is accepted as the sole theorem-bearing authority for the execution kernel

### Stage 8. Deprecate repository Python runtime path

Goal:
- move repository Python from active runtime to compatibility-only status

Actions:
- mark repository Python runtime/harness deprecated in docs
- stop using repository Python for official scored runs
- keep only temporary compatibility scripts if still needed

Exit criteria:
- no official or diagnostic run depends on repository Python runtime authority

### Stage 9. Delete repository Python theorem-bearing surfaces

Goal:
- remove Python from the active embodiment

Delete only after prior stages pass:
- `turingos/runtime.py`
- `turingos/tasks/base.py`
- `turingos/tasks/parity.py`
- `harnesses/mission002_py/llama_openai.py`
- `harnesses/mission002_py/mission002_prompting.py`
- `harnesses/mission002_py/mission002.py`
- `harnesses/mission002_py/parity_cases.py`
- theorem-bearing CLI glue in `turingos/cli.py`

May survive temporarily:
- research or migration scripts that are explicitly non-authoritative

Exit criteria:
- Rust is the only theorem-bearing runtime path

## 8. Risks

- Biggest risk:
  - deleting Python too early would create missing functionality, not simplification
- Second risk:
  - keeping Python alive too long preserves duplicate theorem-bearing surfaces
- Third risk:
  - provider migration may accidentally introduce hidden repair logic if output parsing is made too permissive

## 9. Recommendation

- Strategic decision:
  - **yes, converge to Rust-only**
- Operational decision:
  - **do not delete Python immediately**
- Immediate next phase to open:
  - Rust-first generic execution boundary and operator surface, with parity and real-world harnesses kept outside the repository core

## 10. Definition of Success

The plan succeeds when:

- Rust alone owns the white-box universe
- external APIs remain pure black-box `delta`
- parity scored runs and real-world test harnesses no longer depend on repository Python runtime authority
- Python can be removed without losing any theorem-bearing capability
