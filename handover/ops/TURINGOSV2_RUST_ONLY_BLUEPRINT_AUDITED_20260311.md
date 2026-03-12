# TuringOSv2 Rust-Only Blueprint

Status: Audited blueprint
Date: 2026-03-11 UTC
Scope:
- constitution-aligned detailed blueprint for fully deleting Python from theorem-bearing `turingosv2` paths
- parity checks, benchmark scripts, and real-world task drivers are treated as external disposable harnesses rather than part of the theorem-bearing repo core

## 1. Executive Decision

- Adopt the **Unix proposal** for universe embodiment and atomic commit:
  - one foreground Rust runner
  - one immutable on-disk universe lineage
  - one atomic publish point for `Q_{t+1}`
- Adopt the **Karpathy proposal** for code shape:
  - radically reduce concepts
  - make data flow obvious
  - keep trusted code extremely small
- Reject one dangerous simplification:
  - do **not** fold `rtool`, `predicates`, `wtool`, and `abort` into an opaque “kernel blob”
  - the sealed graph must stay visible in the implementation shape

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

## 3. Team Sequence

Primary designers first:
- Unix proposal: `Galileo`
- Karpathy proposal: `Peirce`

Then audited by:
- `Turing`
- `Ramanujan`
- `Erdos`
- `Sagan`

## 4. Audit Verdicts

### 4.1 Unix proposal

Verdict:
- `PASS`, with one required clarification absorbed into this blueprint

What survived audit:
- one foreground Rust binary is good
- immutable step directories are good
- `current` as an operational pointer is good
- external HTTP `delta` is good
- observer layer must remain downstream only

Required fix absorbed here:
- `current` is **not** `Q_t`
- only the immutable step snapshot is theorem-bearing `Q_t`

### 4.2 Karpathy proposal

Verdict:
- `FIX`, then accepted after constraint tightening

What survived audit:
- fewer concepts
- fewer crates over time
- one trusted library plus one app is a good target
- ledger remains witness, not `Q_t`
- Python may remain only as a migration oracle

Required fix absorbed here:
- even if crates collapse, the implementation must still keep these as first-class internal stages:
  - `rtool`
  - predicate product
  - `wtool`
  - abort

### 4.3 Cross-audit conclusion

- Best final shape:
  - **Unix universe/commit model**
  - **Karpathy minimal concept count**
  - **formal preservation of the sealed causal stages**

## 5. The Final Target Topology

```text
initialization once
  -> Human_Architect() => law
  -> initAI(law) => predicate matrix + map-reduce
  -> freeze both as white-box runtime inputs

foreground shell
  -> turingos run <run-dir> --endpoint ... --model ...
       -> clock tick
       -> load exact Q_t snapshot
       -> mr_map schedules the current read domain
       -> rtool(Q_t) => Input_t
       -> external delta(Input_t) => Output_t
       -> parse Intent or DeltaFault
       -> predicates(Output_t | Q_t)
          -> 0: abort, preserve exact Q_t
          -> 1: mr_reduce ∘ wtool(output | tape_t, HEAD_t, tools_other) => Q_{t+1}
       -> if q_{t+1} == halt, enter HALT
       -> otherwise publish exact Q_{t+1}

downstream only:
  inspect / replay / export / benchmark summaries
```

This is the only acceptable long-term theorem-bearing shape.

Outside this theorem-bearing shape:
- parity-check scripts
- benchmark orchestration
- real-world task harnesses
- ad hoc operator scripts

These may be written in Python by LLMs at run time, but they are not `turingosv2` itself and must not become repository authority.

## 6. Theorem-Bearing Boundaries

### 6.1 What must stay inside trusted Rust

- `Q_t = <q_t, HEAD_t, tape_t>`
- once-only constitutional initialization output
  - predicate matrix
  - map-reduce discipline
- runtime clock ownership
- `rtool`
- predicate-product evaluation
- `wtool`
- lawful execution of `tools_other`
- exact abort to old `Q_t`
- fail-closed parse and fault classification for black-box output

### 6.2 What may stay outside

- external model providers
- HTTP transport
- disposable benchmark shells
- disposable parity-check harnesses
- disposable real-world task harnesses
- report readers
- post-run observers

### 6.3 What must never be outsourced

- any live ownership of `q_t`
- any live ownership of `HEAD_t`
- any write-capable access to `tape_t`
- any authority to mutate predicate matrix or map-reduce runtime law after initialization
- predicate pass/fail authority
- commit / abort authority
- hidden repair of malformed or semantically incomplete model output

## 7. On-Disk Universe Model

Use one run directory as the physical embodiment of the white-box universe lineage:

```text
run/
  manifest.json
  current -> steps/000003
  ledger.jsonl
  summary.json
  steps/
    000000/
      q.json
      HEAD
      tape/
        ...
    000001/
      q.json
      HEAD
      tape/
        ...
  delta/
    000001/request.json
    000001/response.json
    000001/parsed_intent.json
    000001/classification.json
```

Rules:
- each `steps/NNNNNN/` directory is one immutable `Q_t`
- `current` is only an operator pointer, not theorem authority
- `ledger.jsonl` is a witness sidecar, not part of `Q_t`
- abort never mutates `current`
- publish of `Q_{t+1}` happens only after the new snapshot is complete

## 8. Trusted Code Shape

Long-term target packaging:
- one small trusted Rust library
- one thin app crate/binary

But internal theorem-bearing stages must stay explicit:

- `initialization`
  - one-time `Human_Architect -> law -> initAI -> predicate matrix + mr`
- `snapshot`
  - `WorldState`, `Head`, `Tape`
- `clock`
  - one lawful heartbeat source for runtime evolution
- `read`
  - `Input`, `rtool`
- `mr_map`
  - lawful projection and routing of read domains
- `intent`
  - `Intent`, `Action`
- `delta`
  - external boundary, `DeltaFault`
- `task`
  - `Task`, `PredicateVerdict`
- `predicate-product`
  - explicit `∏ predicates`
- `commit`
  - `wtool`
  - bottom-only `tools_other` execution
- `mr_reduce`
  - lawful fold of accepted effects into `Q_{t+1}`
- `abort`
  - exact no-state-advance branch
- `finalize`
  - explicit `HALT` state transition
- `run`
  - bounded step/run loop
- `trace`
  - witness records only

Karpathy rule:
- package count may shrink
- theorem-bearing stage count may **not** disappear

## 9. Migration Rules

- Repository Python must not be deleted before Rust owns the same theorem-bearing responsibility.
- Repository Python may remain temporarily as:
  - migration oracle
  - compatibility reference
  - non-authoritative research utility
- Repository Python may not remain as:
  - live runtime truth
  - task truth
  - provider truth
  - official scored benchmark authority
- External Python is allowed only as:
  - disposable parity-check harness code
  - disposable benchmark drivers
  - disposable real-world task drivers
  - disposable diagnostic shells
- External Python must remain outside the theorem-bearing repository core.

## 10. Detailed Step Plan

### Step 1. Freeze Python theorem expansion

- No new theorem-bearing behavior lands first in Python.
- Rust becomes the only place where new white-box semantics are introduced.

### Step 2. Specify the exact Rust universe format

- Freeze:
  - `q.json`
  - `HEAD`
  - `tape/`
  - `delta/` witness files
  - `ledger.jsonl` role
- This must happen before implementation, not during it.

### Step 3. Remove repository dependence on Python task truth

- Stop treating in-repo parity Python as the intended long-term authority.
- Keep current parity Python only as bootstrap debt and compatibility evidence while Rust takes full theorem-bearing ownership.
- Do not promote parity-check harness logic into the Rust core unless it belongs to the constitutional kernel itself.

### Step 4. Port live parity runtime to Rust

- One Rust parity run must be able to advance `Q_t` end to end.
- Abort must preserve the old snapshot exactly.

### Step 5. Port provider boundary to Rust

- Add one thin HTTP `delta` client in Rust.
- Support:
  - local OpenAI-compatible `llama.cpp`
  - external providers such as Kimi Coding
- Provider output must remain serialized, parsed, and fail-closed.

### Step 6. Keep benchmark and real-world harnesses outside the core

- Do not port parity-check harnesses or real-world task harnesses into the trusted Rust core.
- Expose Rust run, replay, and report surfaces that disposable external Python can drive.
- Preserve comparability with current artifact shapes where needed, but keep the harness itself outside `turingosv2`.

### Step 7. Port CLI/operator path to Rust

- Add Rust operator commands:
  - `run`
  - `step`
  - `inspect`
  - `replay`
  - bounded benchmark entrypoints

### Step 8. Side-by-side equivalence window

- Run the same cases through:
  - Python path
  - Rust path
- Compare:
  - pass/fail
  - first-fail fingerprints
  - step traces
  - exported summaries

### Step 9. Stop using Python for official scored runs

- Official parity scored lanes move to Rust only.
- Python remains read-only oracle or migration utility if still needed.

### Step 10. Delete Python theorem-bearing surfaces

Delete only after Steps 1-9 pass:
- `turingos/runtime.py`
- `turingos/tasks/base.py`
- `turingos/tasks/parity.py`
- `harnesses/mission002_py/llama_openai.py`
- `harnesses/mission002_py/mission002_prompting.py`
- `harnesses/mission002_py/mission002.py`
- `harnesses/mission002_py/parity_cases.py`
- theorem-bearing parts of `turingos/cli.py`

## 11. Anti-Ugly Rules

Never introduce:
- daemon supervisors
- resident watchdogs
- queue systems
- hidden control planes
- plugin registries before the core is boring
- database-backed tape or ledger when files suffice
- provider-specific JSON structures inside the trusted core
- any path where external APIs can touch the universe directly

## 12. The One-Sentence Architecture Rule

The trusted Rust embodiment must read almost literally like the constitution:

- load `Q_t`
- `rtool(Q_t) -> Input_t`
- `delta(Input_t) -> Output_t`
- `predicates(Output_t | Q_t)`
- `wtool(<q_o,a_o> | tape_t, HEAD_t) -> Q_{t+1}`
- otherwise return exact old `Q_t`
