# Mission Groundhog Implementation Guardrails

Status: Active
Mission: Mission Groundhog
Date: 2026-03-09 UTC

## 1. Core Engineering Rule

- Mission Groundhog is the Rust-first engineering embodiment of the sealed constitution in `bible/GROUNDHOG_SEALED_CONSTITUTION.md`.
- Engineering must derive from the constitution, not reinterpret it opportunistically.
- Upgrade decisions must be data-led: they must be grounded in compressed historical success and failure evidence, not in unaudited expert intuition alone.
- AgentOS exists to help `turingos` itself give rise to bounded project intelligence.
- The outer models used by AgentOS are intelligent participants, but that intelligence is for scaffolding, criticism, evidence compression, audit, and promotion control.
- AgentOS must not settle into the permanent role of primary thinker or primary author of upgrades for `turingos`.
- Transitional outer-authored fixes are bootstrap debt to be reduced, not the intended steady state.

## 2. Non-Negotiable Invariants

- `delta` emits an intent envelope, not a world mutation.
- `wtool` owns materialization of the next committed universe snapshot.
- predicate-product validation is the only gate between black-box intent and committed state transition.
- abort returns the exact prior universe snapshot.
- no hidden repair path may smuggle black-box output into committed state.
- no prompt, adapter, benchmark, or test change may hard-code case-local answers, paths, register states, or other benchmark-specific shortcuts merely to pass the current corpus.
- every accepted improvement must increase real protocol-following and real-world difficulty readiness under the unchanged kernel and unchanged topology.
- expert analysis may propose hypotheses, but promotion must remain subordinate to recorded run evidence and compressed failure/success patterns.

## 3. Rust Requirement

- Core Groundhog implementation language: Rust.
- New primary embodiment work for the sealed constitution should default to Rust rather than extending the Python reference by inertia.
- Rust work should favor:
  - explicit types for state snapshots and intent envelopes
  - small deterministic modules
  - explicit error channels
  - testable pure functions where possible
  - minimal hidden global state

## 4. Unix Requirement

- Groundhog software should look and behave like Unix-quality systems software:
  - clear process boundaries
  - explicit files and durable artifacts
  - small composable interfaces
  - no magical hidden daemons unless absolutely necessary
  - predictable exit codes
  - structured logs and reproducible commands
  - policy separated from mechanism
- Groundhog may not introduce watchdog or supervisor Python programs.
- Groundhog may not introduce repository-resident watchdog or supervisor daemons in any language unless a future stage spec explicitly opens that surface.
- Any watch, poll, or progress-monitoring behavior must be handled by an explicit agent role, normally `turingos_runtime_watcher` or another declared polling agent under Commander control.

## 5. Karpathy Requirement

- Groundhog code should be evaluated for simplification and taste, not only for raw functionality.
- Preferred traits:
  - fewer concepts
  - sharper names
  - short and legible control flow
  - obvious data movement
  - comments only where the code would otherwise hide a key idea
  - no abstraction inflation for its own sake

## 6. Git Requirement

- Git history must preserve provenance from constitution to derived engineering.
- The sealed constitution file must be treated as a protected anchor in review and commit slicing.
- Groundhog changes should prefer small, reviewable, logically atomic commits over giant refactor dumps.

## 7. Implementation Boundaries

- Allowed:
  - Rust type design
  - Rust crate and module decomposition
  - CLI and tooling design
  - proof-oriented tests
  - derived documentation
  - benchmarking and validation
- Forbidden:
  - mutating the sealed constitution
  - introducing hidden model-side repair paths
  - letting the black box directly mutate future state
  - disguising a derived engineering convenience as a mathematical authority

## 8. Preferred Decomposition

- Constitution lane:
  - theorem preservation
  - topology preservation
  - invariant vocabulary
- Rust systems lane:
  - snapshot type
  - intent envelope type
  - predicate gate
  - commit engine
  - read tool and write tool surfaces
- Unix lane:
  - CLI
  - files
  - logs
  - process boundaries
- Validation lane:
  - invariant tests
  - replay tests
  - commit/abort tests
  - integration traces

## 9. Escalation Rule

- If a proposed implementation is easier but contradicts the sealed constitution, the team must escalate instead of “temporarily” violating the theorem.
