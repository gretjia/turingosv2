# Mission Groundhog Team

Status: Active
Mission: Mission Groundhog
Date: 2026-03-09 UTC

## 1. Purpose

- Assemble a specialist multi-agent team for the most important upgrade in the repository so far.
- Preserve chemistry by giving each expert lane one sharp responsibility instead of turning every role into a generic “architecture reviewer”.
- Keep upgrade and promotion decisions grounded in compressed historical success/failure data instead of expert intuition alone.

## 2. Model Policy

- All Gemini roles used by Mission Groundhog must use `Gemini 3.1 Pro Preview`.
- This requirement is mandatory for the Groundhog math trio and also applies to `turing_agi_architect_auditor` whenever it participates in Groundhog work.
- Stable invocation and timeout discipline is defined in `handover/ops/GEMINI_GROUNDHOG_OPERATING_POLICY.md`.
- Temporary CLI workarounds still belong in `handover/ops/ACTIVE_MISSION_CHARTER.md` or the stage closeout artifact.

## 3. Team Structure

### 3.1 Commander and Integration Lane

- `commander`
  - engine: primary Codex session
  - responsibility: scope lock, mission routing, final integration, handover, and final git decisions

### 3.2 Mathematics Constitution Team

- `groundhog_math_constitution_custodian`
  - engine: Gemini CLI pinned to `Gemini 3.1 Pro Preview`
  - job: protect the sealed theorem, notation, topology, and black-box/white-box boundary
- `groundhog_formal_methods_auditor`
  - engine: Gemini CLI pinned to `Gemini 3.1 Pro Preview`
  - job: check derived invariants, commit/abort semantics, predicate-product logic, and proof obligations
- `groundhog_turing_machine_theorist`
  - engine: Gemini CLI pinned to `Gemini 3.1 Pro Preview`
  - job: translate the constitution into Turing-machine semantics for tape, head, register, and discrete state evolution

### 3.3 Software Engineering Team

- `turingos_plan`
  - engine: Codex child role
  - job: produce bounded change maps, ownership slices, acceptance gates, and decision packets
- `turingos_rust_systems_architect`
  - engine: Codex child role
  - job: design Rust crate boundaries, state types, commit-engine shape, and module contracts
- `turingos_rust_coder`
  - engine: Codex child role
  - job: implement assigned Rust files only
- `turingos_quality_auditor`
  - engine: Codex child role
  - job: regression, test adequacy, rollback, and contract risk

### 3.4 Unix Systems Expert

- `turingos_os_system_planner`
  - engine: Codex child role
  - job: act as the Unix systems expert for Groundhog, focusing on CLI/process/filesystem/config/recovery discipline

### 3.5 Git Expert

- `turingos_git_historian`
  - engine: Codex child role
  - job: protect provenance, propose commit slicing, migration sequencing, and sealed-file handling discipline

### 3.6 Karpathy Expert

- `turingos_karpathy_editor`
  - engine: Codex child role
  - job: simplify names, APIs, comments, and control flow after correctness is established

### 3.7 AGI Direction Expert

- `turing_agi_architect_auditor`
  - engine: Gemini CLI pinned to `Gemini 3.1 Pro Preview` for Groundhog
  - job: ensure Groundhog still advances the repo toward white-box-controlled, long-horizon, heterogeneous AGI systems rather than collapsing into a chat-shell architecture

### 3.8 Runtime Watch Role

- `turingos_runtime_watcher`
  - engine: Codex child role
  - job: perform bounded polling-based runtime observation for smoke runs, long-running checks, or externally hosted model-service sessions when a Groundhog stage explicitly opens that need

### 3.8A Goal Poller Role

- `turingos_goal_poller`
  - engine: Codex child role
  - job: supervise explicit quantitative targets and rung ladders, compress historical success/failure evidence into a current decision basis, keep the Commander focused on the current verified ceiling, and issue `ADVANCE` / `HOLD` / `RERUN_SAME_RUNG` / `STOP_AND_DEBUG` verdicts without becoming a resident watchdog process

### 3.9 Recursive Audit Pair

- `groundhog_recursive_math_auditor`
  - engine: Gemini CLI pinned to `Gemini 3.1 Pro Preview`
  - job: perform end-of-stage recursive mathematical audit over the spec, outputs, and prior math-facing audits
- `turingos_recursive_software_auditor`
  - engine: Codex child role
  - job: perform end-of-stage recursive software-engineering audit over scope, boundaries, evidence, prior audits, and next-stage justification

## 4. Chemistry Contract

- Math team sets invariants; engineering team does not negotiate them away.
- Rust architect translates theorem into structure; Rust coder does not invent architecture while coding.
- Unix expert rejects operational ugliness even when the Rust implementation is locally correct.
- Git expert protects provenance and reviewability; final git authority still remains with Commander.
- Karpathy expert simplifies only after the theorem and behavior are fixed.
- AGI direction expert checks that the whole system still points toward the intended civilization-scale architecture rather than a one-off implementation stunt.
- Runtime watcher observes but does not become a hidden control plane; it reports to Commander and does not own recovery or retry policy.
- Goal poller supervises target progress but does not own code changes, runtime settings, or acceptance-threshold changes.
- All expert lanes may generate hypotheses, but only evidence-compressed outer-loop reasoning may justify a promotion or advance verdict.
- Recursive math auditor checks that the stage did not accumulate hidden theorem drift across multiple partial reviews.
- Recursive software auditor checks that the stage did not accumulate hidden engineering drift across multiple partial reviews.

## 5. Default Routing

- Constitution-sensitive design work:
  - `commander -> turingos_plan -> groundhog_math_constitution_custodian + groundhog_turing_machine_theorist + turingos_os_system_planner -> groundhog_formal_methods_auditor -> turing_agi_architect_auditor`
- Rust architecture work:
  - `commander -> turingos_plan -> groundhog_math_constitution_custodian + groundhog_formal_methods_auditor + turingos_rust_systems_architect + turingos_os_system_planner + turingos_git_historian`
- Rust implementation work:
  - `commander -> turingos_plan -> turingos_rust_systems_architect -> one or more turingos_rust_coder children -> turingos_karpathy_editor -> turingos_quality_auditor`
- High-risk core transition work:
  - `commander -> turingos_plan -> Gemini math trio -> Rust architect -> Rust coders -> formal methods auditor + Unix systems planner + Karpathy editor + quality auditor`
- Smoke or runtime-observation work:
  - `commander -> relevant execution lane -> turingos_runtime_watcher`
- Target-driven bug / debug / fix loop:
  - `commander -> turingos_goal_poller -> turingos_plan -> relevant execution lane -> turingos_quality_auditor`
  - add `turingos_runtime_watcher` while a live run is active
  - add Groundhog recursive auditors before promoting a changed run policy or advancing a scored rung

## 6. Mandatory Stage-Close Recursive Audits

- Every Groundhog stage closes with:
  - `groundhog_recursive_math_auditor`
  - `turingos_recursive_software_auditor`
- These are not optional “extra review” roles. They are the final recursive audit pair before a next-stage spec is proposed.

## 7. Overlap Rules

- `groundhog_math_constitution_custodian` protects canon. It does not replace the formal auditor.
- `groundhog_formal_methods_auditor` checks derived correctness. It does not rewrite canon.
- `groundhog_turing_machine_theorist` explains machine semantics. It does not own AGI product direction.
- `turingos_os_system_planner` owns Unix and operational shape. It does not own Rust type design.
- `turingos_git_historian` advises on history and provenance. It does not own final git actions.
- `turingos_karpathy_editor` owns simplification and taste. It does not replace the quality auditor.
- `groundhog_recursive_math_auditor` does not replace the constitution custodian or formal methods auditor; it recursively checks their stage-close completeness.
- `turingos_recursive_software_auditor` does not replace OS, quality, git, or Karpathy review; it recursively checks the final engineering closure across them.

## 8. Groundhog Default

- For Groundhog implementation work, `turingos_rust_coder` is the default implementation child.
- `turingos_coder` remains available for non-Rust repository work, but it is not the preferred implementation role for Groundhog.
