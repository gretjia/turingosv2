# TuringOSv2 Constitution AI-Friendly Rewrite

Status: Derived explanatory note
Date: 2026-03-11 UTC
Authority:
- Source of truth remains `bible/GROUNDHOG_SEALED_CONSTITUTION.md`

Non-authority notice:
- This file does not supersede, edit, or reinterpret the sealed constitution.
- It only rewrites the notation into a form that is easier for AI systems and
  implementation docs to read.
- The sealed file remains the only constitutional source.

## 1. Replacement Rule

This derived note stops using the symbols `∘` and `•` in explanatory material.

Use these words instead:

- `WHITEBOX` = deterministic, safe, theorem-bearing mechanism
- `BLACKBOX` = noisy, creative, untrusted proposal mechanism

## 2. Final Clockwork Form

### Initialization Phase

```math
TENTATIVE_LAW[WHITEBOX]
\Longleftarrow_{\text{once}}
HumanArchitect_WHITEBOX()
```

```math
\langle
PREDICATE_MATRIX[WHITEBOX],\ MAP_REDUCE[WHITEBOX]
\rangle
\Longleftarrow_{\text{once}}
InitAI_BLACKBOX(TENTATIVE_LAW[WHITEBOX])
```

### World State

```math
WORLD_t[WHITEBOX] := \langle q_t,\ HEAD_t,\ tape_t \rangle
```

### Clocked Read and Map Phase

```math
INPUT_t[WHITEBOX]
:=
ReadTool_WHITEBOX(\langle q_t,\ tape_t,\ HEAD_t \rangle)
\quad\text{driven by}\quad
MAP_REDUCE_{map}[WHITEBOX]
```

Equivalent reading:

```math
INPUT_t[WHITEBOX] = \langle q_i,\ s_i \rangle
```

### Pure Generate Phase

```math
PROPOSAL_t[BLACKBOX]
:=
\langle q_o,\ a_o \rangle
=
Delta_BLACKBOX(INPUT_t[WHITEBOX])
```

### Predicate Product Phase

```math
PREDICATE_PRODUCT_t[WHITEBOX]
:=
\prod\nolimits_{WHITEBOX}\mathbf{p}
\big(
PROPOSAL_t[BLACKBOX] \mid WORLD_t[WHITEBOX]
\big)
\in \{ PASS, REJECT \}
```

### Commit / Abort / Reduce Phase

```math
WORLD_{t+1}[WHITEBOX]
=
\begin{cases}
MAP_REDUCE_{reduce}[WHITEBOX]
\circ
WriteTool_WHITEBOX(
PROPOSAL_t[BLACKBOX]
\mid
tape_t,\ HEAD_t,\ tools_{other}[WHITEBOX]
)
& \text{if } PREDICATE_PRODUCT_t[WHITEBOX] = PASS
\\
WORLD_t[WHITEBOX]
& \text{if } PREDICATE_PRODUCT_t[WHITEBOX] = REJECT
\end{cases}
```

### Finalization Phase

```math
HALT[WHITEBOX]()
\Longleftarrow
\text{if } q_{t+1} \equiv "halt"
```

## 3. Minimal Reading Guide

- Human provides a tentative law once, not continuous micro-management.
- `InitAI_BLACKBOX` may compile the law once, but it does not keep live
  legislative power after initialization.
- The live world is always `WORLD_t[WHITEBOX]`.
- The model never sees the live world directly.
- The model only sees a WHITEBOX projection of the current world.
- The model never writes the next world directly.
- The model only emits a suspended BLACKBOX proposal.
- `tools_other` belong to the WHITEBOX write path, not to the BLACKBOX model.
- Only WHITEBOX predicate product plus WHITEBOX write-tool may materialize the
  next committed world.
- If predicate product fails, the system returns the exact prior world without
  patching or partial application.

## 4. Rust-Syntax Alignment

The current Rust tree should be read against the Clockwork constitution with
this mapping:

### Constitution-bearing Rust carriers

```text
WORLD_t[WHITEBOX]
  = WorldState<QState>
  = <register_t, head_t, tape_t>

CURRENT_COMMITTED_SNAPSHOT
  = UniverseSnapshot<QState>
  = WorldState<QState> + SnapshotWitness
```

### Read carrier

```text
INPUT_t[WHITEBOX]
  = ReadView<QState>
  = <register_t, current_content_t>
```

Current implementation note:

- `ReadView<QState>` still carries `head`.
- Under the strict Clockwork reading, the theorem-bearing input is still only:
  - `register_t`
  - `ReadTool_WHITEBOX(tape_t, head_t)`
- Therefore `head` must be treated as implementation routing metadata, not as
  additional BLACKBOX authority.

### Proposal carrier

```text
PROPOSAL_t[BLACKBOX]
  = IntentEnvelope<QState>
  = <proposed_register_t, proposed_action_t>

proposed_action_t
  = <proposed_head_t, write_mode_t, write_content_t, halt_t>
```

Current implementation note:

- `notes` and `action_payload` are auxiliary BLACKBOX-side fields.
- They must never be read as committed world truth.
- `proposed_register` is not `register_{t+1}`.
- `proposed_head` is not `HEAD_{t+1}`.

### Predicate carrier

```text
PREDICATE_PRODUCT_t[WHITEBOX]
  = kernel_reasons(intent)
    AND
    TaskContract::evaluate(world, intent)
  = PredicateVerdict
```

### Commit carrier

```text
WORLD_{t+1}[WHITEBOX]
  = commit_snapshot(snapshot, intent, gate_pass)
```

Current Clockwork gap note:

- The current Rust core does not yet embody:
  - `InitAI_BLACKBOX`
  - `MAP_REDUCE[WHITEBOX]`
  - `clock`
  - `tools_other[WHITEBOX]` inside the live write path
- Those remain constitutional targets, not yet completed Rust embodiments.

## 5. Guardrails For Derived Docs And Code Comments

- Do not let `q_o` appear as committed next state outside `WriteTool_WHITEBOX`.
- Do not let `a_o` appear as already executed action outside
  `WriteTool_WHITEBOX`.
- Do not widen theorem-bearing input beyond the WHITEBOX read projection.
- Do not place `tools_other` on the BLACKBOX side in any derived architecture
  diagram or comment.
- Do not treat current Rust implementation gaps as if they were already solved
  by the code.

## 6. Usage Rule

This notation should be preferred in:

- implementation notes
- handover audits
- code comments
- architecture overviews for AI readers

The original sealed notation remains mandatory in the sealed constitution file.
