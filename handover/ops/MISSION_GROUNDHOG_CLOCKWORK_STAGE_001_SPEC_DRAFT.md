# Mission Groundhog Clockwork Stage 001 Spec Draft

Status: Draft after multi-agent review, awaiting human confirmation
Date: 2026-03-12 UTC
Mission: Mission Groundhog - Final Clockwork Embodiment Stage 001 - pre-implementation design freeze

## 1. Stage Objective

- Freeze the first implementation boundary required by the sealed Final Clockwork constitution before any code changes begin.
- Convert the reviewed constitutional diff into an exact Stage 002 opening packet instead of a blurred implementation wave.
- Keep Stage 001 narrow enough that approval means one bounded next slice is frozen, not that coding may proceed broadly.

## 2. Authority Order

Constitutional authority:

1. `bible/GROUNDHOG_SEALED_CONSTITUTION.md`

Derived planning inputs:

1. `handover/ops/TURINGOSV2_FINAL_CLOCKWORK_CODE_DIFF_AUDIT_20260311.md`
2. `handover/ops/TURINGOSV2_RUST_ONLY_BLUEPRINT_AUDITED_20260311.md`
3. `handover/ops/TURINGOSV2_RUST_ONLY_MIGRATION_PLAN_20260311.md`
4. `handover/ops/MISSION_GROUNDHOG_EXECUTION_LOOP.md`
5. `handover/ops/MISSION_GROUNDHOG_IMPLEMENTATION_GUARDRAILS.md`
6. `handover/ops/TURINGOSV2_CONSTITUTION_AI_FRIENDLY_REWRITE_20260311.md`

## 3. Stage Decision

Stage 001 is the **pre-implementation design freeze for Final Clockwork boundary decisions**.

This stage is intentionally limited to:

- exact constitutional boundary decisions
- exact Stage 002 opening file family
- exact acceptance gates
- exact audit pack

This stage does **not** authorize code edits.

## 4. Why This Stage Comes First

The reviewed diff already says the strongest missing Final Clockwork surfaces are:

- once-only `Human -> law -> initAI -> predicates + mr`
- explicit `clock`
- explicit `mr_map / mr_reduce`
- explicit `tools_other` embodiment inside bottom `wtool`
- explicit linkage from committed `q_{t+1} == "halt"` to `HALT`

Those surfaces are still large enough to tempt an over-broad refactor. Stage 001 therefore freezes:

- what belongs to trusted Rust kernel physics
- what belongs to trusted once-only initialization surfaces
- what belongs to trusted runtime orchestration surfaces
- what remains outside as external harness or provider transport

## 5. Exact In-Scope Files For Stage 001

Stage 001 is document-scope only. Its exact in-scope files are:

- `handover/ops/ACTIVE_MISSION_CHARTER.md`
- `handover/ops/MISSION_GROUNDHOG_CLOCKWORK_STAGE_001_SPEC_DRAFT.md`
- `handover/ai-direct/LATEST.md`
- `handover/ai-direct/entries/`

## 6. Planned Target Files For Stage 002

Stage 001 does not open implementation code, but it must freeze the first implementation slice narrowly enough that Stage 002 can begin cleanly. The planned target set for Stage 002 is:

- `crates/turingos-kernel/src/lib.rs`
- `crates/turingos-kernel/src/initialization.rs`
- `crates/turingos-core/src/compiled_law.rs`
- `Cargo.toml`

Stage 002 should not open the broader legacy core file set until this smaller initialization-boundary slice is implemented and audited.

## 7. Explicitly Out Of Scope For Stage 001

- `crates/` implementation changes
- `turingos/`
- `harnesses/`
- provider implementation details
- benchmark ladder strategy
- parity prompt tuning
- Kimi sidecar integration work
- deleting more Python
- broad crate mergers

## 8. Stage 001 Output Packet

Stage 001 must produce:

1. a reviewed implementation boundary map
2. a reviewed Stage 002 opening file-family map
3. a reviewed slice order for the next implementation stage
4. a reviewed acceptance-gate set for the next implementation stage
5. a reviewed audit-pack requirement for the next implementation stage

## 9. Boundary Decisions To Freeze

### 9.1 Trusted Rust kernel

Must remain trusted and explicit:

- `WorldState`
- `ReadTool`
- `PredicateGate`
- `WriteTool`
- reject branch preserving exact prior `Q_t`
- committed `q_{t+1} == "halt"` linkage to `HALT`

### 9.2 Trusted once-only initialization surfaces

Must be frozen as exact once-only initialization surfaces and must not remain a vague future abstraction:

- human-provided tentative `law` as the once-only WHITEBOX input
- compiled predicates `P` as once-only `initAI` output
- compiled routing and reduction substrate `mr` as once-only `initAI` output

### 9.3 Trusted clock/map/reduce stage surfaces

Must be treated as first-class WHITEBOX stages and must not live in `harnesses/`:

- `clock`
- `mr_map`
- `mr_reduce`

### 9.4 External and untrusted

Must remain outside the trusted core:

- provider transport
- benchmark orchestration
- disposable parity harnesses
- disposable real-world harnesses

## 10. Proposed Next Implementation Slice After Approval

If this draft is approved, the next implementation stage should start with:

### Slice A: Initialization boundary

- define where `law`, compiled predicates, and compiled `mr` live
- define whether they are artifacts, types, or boot-time sealed structures

### Slice B: Clock and map surface

- define the trusted `clock`
- define the trusted `mr_map` read-routing surface
- ensure `clock`, `mr_map`, and `mr_reduce` do not live in `harnesses/`

### Slice C: Reduce and bottom tools

- define how `tools_other` can only awaken through bottom `wtool`
- define the explicit `mr_reduce` fold point

### Slice D: HALT linkage

- define how committed `q_{t+1} == "halt"` surfaces the constitutional `HALT` condition

Stage 001 does not implement these slices. It only freezes that this is the correct order.

## 11. Acceptance Gates

Approval of Stage 001 means these boundary decisions are frozen:

1. only the sealed constitution is mathematical authority; all other documents are planning inputs only
2. Stage 001 is document-scope only and does not authorize implementation code changes
3. Stage 002 begins with the initialization boundary, not a broad implementation wave
4. `clock`, `mr_map`, and `mr_reduce` are trusted WHITEBOX runtime surfaces and must not live in `harnesses/`
5. `P` and `mr` are once-only compiled outputs of `initAI`
6. reject means preserving exact prior `Q_t`
7. `HALT` is surfaced only through committed `q_{t+1} == "halt"`
8. external harnesses, provider transport, and benchmark strategy remain outside the trusted core

## 12. Required Review Lenses

- mathematics and constitutional fidelity
- topology and OS boundary discipline
- minimalism and code-architecture sharpness
- stage-loop and sequencing discipline

## 13. Required Tests

- No implementation tests run in Stage 001; verification is document-scope only.
- Verification command: `git -C /home/zephryj/projects/turingosv2 diff --name-only -- crates turingos harnesses`
- Verification must confirm that no files under `crates/`, `turingos/`, or `harnesses/` changed in this stage.
- Verification must confirm that no autonomous progression window is enabled for Stage 001.

## 14. Open Risks

- Stage 002 may still be tempted to widen from initialization boundary into `clock`, map, reduce, and `tools_other` in one wave.
- The exact representation of compiled `P` and compiled `mr` may still create unnecessary artifact complexity if Stage 002 does not stay narrow.
- `ReadView` may still prove wider than the Final Clockwork reading allows once Stage 002 touches the initialization and routing boundary.

## 15. Commander Recommendation

- Approve this draft as the bounded planning stage for the first Final Clockwork pre-implementation freeze.
- Do not skip directly to coding.
- Use the reviewed Stage 001 packet to open the next implementation stage cleanly and narrowly.
- No autonomous progression window is enabled for Stage 001; human confirmation is required before any Stage 002 implementation work opens.
