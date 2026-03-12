# Mission Groundhog Phase 1 Architecture Packet

Status: Completed with audit PASS
Mission: Mission Groundhog
Phase: 1
Date: 2026-03-09 UTC

## 1. Rust Workspace Shape

- Mission Groundhog should open as a small Rust workspace, not as a mirror of the current Python tree.
- The minimal intended workspace shape is:
  - workspace root `Cargo.toml`
  - `crates/turingos-core/`
  - `crates/turingos-kernel/`
- Future crates are expected but remain closed in Phase 2:
  - `crates/turingos-task/`
  - `crates/turingos-cli/`
- The adapter seam is intentionally not frozen as a standalone crate in Phase 1. Its eventual home remains deferred until the pluggable-adapter phase.
- The Python reference under `turingos/` remains read-only context during the Rust embodiment opening.

## 2. Kernel Versus Plug-In Boundary Map

The architecture must preserve one hard split from the beginning:

- White-box kernel authority:
  - universe snapshot `Q_t`
  - read tool boundary
  - predicate gate
  - atomic commit engine
  - durable trace evidence
  - foreground run orchestration
- White-box task policy:
  - task-local read expectations
  - task-local transition validation
  - task-local success conditions
- Pluggable black-box boundary:
  - model adapters
  - future tool-using worker wrappers
  - prompt shaping
  - transport and model-provider concerns
- CLI and operational boundary:
  - workspace path binding
  - artifact path binding
  - exit status and log shape
  - explicit smoke-run routing

Non-negotiable rule:

- No plug-in or adapter gets a direct write path to committed future state.
- Black-box code may emit only an unverified intent envelope.

## 3. Canonical Core Types

### 3.1 Universe Snapshot `Q_t`

- Canonical Rust name: `UniverseSnapshot`
- Semantic content:
  - `register`: the committed control register `q_t`, typed as workload-owned `QState`
  - `head`: the committed attention pointer `HEAD_t`
  - `tape`: the committed workspace view `tape_t`
  - `ledger_tip`: the current append-only trace anchor for the committed universe lineage
  - `step`: monotonic timeline index
  - `halted`: committed halt bit
- Invariant:
  - `UniverseSnapshot` is committed reality, not a mutable working buffer.
  - the Python `dict[str, Any]` register shape is not an acceptable Rust embodiment target

### 3.2 Intent Envelope

- Canonical Rust name: `IntentEnvelope`
- Semantic content:
  - proposed next register payload
  - proposed external action payload
  - proposed next head location
  - write mode and optional write content
  - halt flag
  - notes
  - raw model provenance
- Invariant:
  - the intent envelope is unverified and inert
  - it is never named `NextState`
  - it carries zero authority to materialize `Q_{t+1}`

### 3.3 Read View

- Canonical Rust name: `ReadView`
- Semantic content:
  - the read-phase projection visible to a black-box generator
  - current content slice
  - public control metadata the kernel intentionally exposes
- Invariant:
  - `ReadView` is the only public plugin-facing input
  - it is derived from, but is not, the committed universe snapshot

### 3.4 Predicate Gate

- Canonical Rust name: `PredicateGate`
- Semantic contract:
  - evaluate `IntentEnvelope` against `UniverseSnapshot`
  - produce a total gate result over the required white-box predicates
- Invariant:
  - predicate evaluation is side-effect-free
  - failure keeps the current universe untouched

### 3.5 Atomic Commit/Abort Outcome

- Canonical Rust names:
  - `GatePass`
  - `CommitOutcome`
- Required outcome cases:
  - `Commit { next: UniverseSnapshot, record: CommitRecord }`
  - `Abort { preserved_step: u64, reasons: Vec<String>, record: RejectRecord }`
- Invariant:
  - commit mints a new owned universe snapshot
  - abort does not mutate the prior universe snapshot
  - only the kernel may forge the gate-pass capability required for commit

## 4. Proposed Crate and Module Map

### 4.1 `turingos-core`

Owns the theorem-near types and vocabulary:

- `UniverseSnapshot`
- `Head`
- `TapeState`
- workload-generic `QState` container shape
- `ReadView`
- `IntentEnvelope`
- `CommitOutcome`
- `CommitRecord`
- `RejectRecord`
- shared ids, hashes, and provenance payloads

This crate should be small, stable, and nearly free of policy.

### 4.2 `turingos-kernel`

Owns the white-box transition machinery:

- `read`
  - read projection and head-local slice construction
- `contracts`
  - temporary kernel-private workload seam for Phase 2
- `predicate_gate`
  - predicate composition
  - verdict normalization
  - gate-pass construction
- `commit`
  - staged materialization
  - atomic commit/abort outcome construction
- `trace`
  - durable commit/reject record emission
  - trace integrity hooks
- `engine`
  - one foreground execution session

### 4.3 Future Closed Crates

- `turingos-task`
  - white-box task traits and task-local invariants
- `turingos-cli`
  - command-line entrypoints and human-facing reporting
- later pluggable adapter home
  - remains intentionally deferred until a future phase opens it

These remain closed until later phases explicitly open them.

Deferred after Phase 2:

- `scheduler`
  - later multi-agent selection and pricing work
- a possible `ledger` split
  - only if `trace` later needs to separate append-only storage from trace shaping

## 5. Mapping of Read, Generate, and Commit/Abort Phases

### 5.1 Read Phase

- Constitutional operator: `rtool`
- Rust home:
  - `turingos-kernel::read`
- Responsibility:
  - derive a read-only local slice from `UniverseSnapshot`
  - never mutate `UniverseSnapshot`
  - never smuggle hidden policy into the black-box boundary

### 5.2 Generate Phase

- Constitutional operator: `delta`
- Rust home:
  - a future pluggable-adapter home that is not frozen in Phase 1
- Responsibility:
  - consume `ReadView`
  - emit only `IntentEnvelope`
- Non-negotiable boundary:
  - no adapter may receive mutable access to committed kernel state
  - no adapter may bypass the predicate gate

### 5.3 Commit/Abort Phase

- Constitutional operators:
  - `PredicateGate`
  - `wtool`
- Rust home:
- `turingos-kernel::predicate_gate`
- `turingos-kernel::commit`
- `turingos-kernel::trace`
- Responsibility:
  - evaluate the full predicate product
  - construct the gate-pass capability only on exact pass
  - mint the next committed universe snapshot on commit
  - preserve the prior committed universe on abort
  - emit exactly one durable commit or rejection trace record

## 6. Rust Type and Ownership Plan

- `UniverseSnapshot` should be treated as committed data owned by the run loop.
- Read and predicate paths receive `&UniverseSnapshot`.
- The black-box boundary receives `ReadView`, not a writable snapshot handle.
- The commit engine takes:
  - `&UniverseSnapshot`
  - the original `IntentEnvelope`
  - a kernel-forged gate-pass capability
  - white-box tool context
- The commit engine returns a new owned `UniverseSnapshot` or an abort outcome.
- No adapter, tool wrapper, or scheduler path may hold the authority to forge the gate-pass capability.
- The run loop is foreground and bounded; it owns retry policy, not a background daemon.
- Phase 2 task policy stays kernel-private inside `contracts.rs`; no stable public task crate opens in Phase 2.

## 7. Invariant-Sensitive API Boundary Matrix

| Surface | Public to later phases | Forbidden now | Key invariant |
| --- | --- | --- | --- |
| `turingos-core::UniverseSnapshot` | Yes | in-place mutation helpers | committed reality is immutable authority |
| `turingos-core::IntentEnvelope` | Yes | commit helpers or file handles | unverified output is inert |
| `turingos-core::ReadView` | Yes | snapshot write handles | plugin input is projection only |
| `turingos-kernel::read` | Internal first | implicit write side effects | read is projection only |
| `turingos-kernel::predicate_gate` | Internal first | public gate-pass constructor | predicate product is pure |
| `turingos-kernel::commit` | Internal first | partial write interfaces | commit is atomic, abort is zero-mutation |
| future adapter seam | Not opened in Phase 2 | direct snapshot write access | black-box code cannot materialize state |
| CLI | Not opened in Phase 2 | theorem-bearing shortcuts | CLI orchestrates, not authorizes |

## 8. Unix Operational Shape Note

- Groundhog should preserve a foreground, command-driven execution model.
- One invoked command owns one bounded execution session.
- Monitoring is observational only and belongs to polling agents, not resident daemons.
- Runtime artifacts must stay in explicit workspace or handover paths:
  - workspace state
  - append-only ledger
  - reproducible traces
  - closeout evidence
- For the current execution window, hardware-dependent smoke runs may use the Mac node only; that routing remains an operational note, not architectural law.

## 9. Git Provenance and Migration Sequencing Note

- `bible/GROUNDHOG_SEALED_CONSTITUTION.md` remains the protected provenance anchor.
- Once Phase 1 is opened, `MISSION_GROUNDHOG_PHASE_001_SPEC_DRAFT.md` also becomes frozen provenance.
- Any post-open change to the Phase 1 spec must happen only through an explicit amendment artifact, never by rewriting the opened spec in place.
- Recommended commit slicing:
  - finalize/open Phase 1 spec
  - add this architecture packet
  - apply audit-driven corrections to this packet only
  - add `handover/ops/MISSION_GROUNDHOG_PHASE_001_CLOSEOUT.md`
  - add one append-only entry matching `handover/ai-direct/entries/*_groundhog_phase001_closeout.md`
- Migration order from Python reference to Rust embodiment:
  - Phase 1: architecture and proof packet only
  - Phase 2: workspace scaffold plus core types
  - later phases: task crate, adapters, CLI, benchmark integration

## 10. Early Validation and Proof-Obligation Plan

### 10.1 Type-Level Invariants

- `UniverseSnapshot` is committed white-box state.
- `IntentEnvelope` is unverified black-box output.
- only the kernel may forge the gate-pass capability admissible to commit.
- `CommitOutcome::Abort` implies zero mutation of the prior committed universe.

### 10.2 Proof Obligations

- Read phase:
  - projection is deterministic and read-only
- Generate phase:
  - adapters are topologically isolated from committed state
- Commit phase:
  - only a kernel-forged gate-pass capability may reach the commit engine
  - commit yields a new universe snapshot
- Abort phase:
  - failed gate leaves the prior universe unchanged
  - failed gate still yields an append-only rejection record for observability
  - zero-mutation abort is structurally enforced because commit logic reads the prior universe through an immutable borrow

### 10.3 Phase 2 Validation Start

- compile-time type checks for snapshot versus intent separation
- unit tests for predicate-gate normalization
- unit tests for commit versus abort outcome formation
- checks that successful commit emits exactly one durable `CommitRecord`
- checks that abort emits exactly one durable `RejectRecord` without mutating the prior committed universe

## 11. Exact Intended Phase 2 Opening Slice

Phase 2 may branch/open only from the commit that already contains:

- a normalized Phase 1 closeout verdict of exact `PASS`
- `handover/ops/MISSION_GROUNDHOG_PHASE_001_CLOSEOUT.md`
- one append-only entry matching `handover/ai-direct/entries/*_groundhog_phase001_closeout.md`

Phase 2 should open only:

- workspace `Cargo.toml`
- `crates/turingos-core/`
- `crates/turingos-kernel/`

And inside that slice only:

- `crates/turingos-core/Cargo.toml`
- `crates/turingos-core/src/lib.rs`
- `crates/turingos-core/src/ids.rs`
- `crates/turingos-core/src/snapshot.rs`
  - owns `UniverseSnapshot<QState>`
- `crates/turingos-core/src/intent.rs`
  - owns `ReadView` and `IntentEnvelope`
- `crates/turingos-core/src/predicate.rs`
  - owns predicate verdict types only
- `crates/turingos-core/src/outcome.rs`
  - owns `CommitOutcome`, `CommitRecord`, and `RejectRecord`
- `crates/turingos-core/src/error.rs`
- `crates/turingos-kernel/Cargo.toml`
- `crates/turingos-kernel/src/lib.rs`
- `crates/turingos-kernel/src/config.rs`
- `crates/turingos-kernel/src/contracts.rs`
- `crates/turingos-kernel/src/read.rs`
- `crates/turingos-kernel/src/predicate_gate.rs`
- `crates/turingos-kernel/src/commit.rs`
- `crates/turingos-kernel/src/engine.rs`
- `crates/turingos-kernel/src/trace.rs`
- no adapter work
- no CLI work
- no benchmark migration
- no scheduler work
