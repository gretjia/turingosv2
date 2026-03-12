# TuringOSv2 Self-Upgrade Inner Proposer Spec

Status: Draft
Date: 2026-03-10 UTC
Scope: Whitebox-readable inner-loop specification for bounded self-improvement work under the 1M pressure-test project

## 1. Purpose

- Define what it means for `turingosv2` to "improve itself" without violating the sealed constitution, the frozen kernel boundary, or the anti-hardcoding rule.
- Keep the inner loop bounded to proposal and execution surfaces rather than allowing it to redefine law, truth, or promotion.
- Treat `turingosv2` itself as the locus where bounded project intelligence should emerge, rather than treating the outer AgentOS loop as the permanent source of upgrades.
- Treat AgentOS as the support system that helps `turingosv2` give birth to that bounded intelligence, not as the intended long-term substitute for it.

## 2. Core Rule

- `100` consecutive headline passes is an outer objective and promotion gate.
- It is not the inner whitebox truth condition.
- The inner loop may optimize toward that objective only by producing bounded candidate revisions on declared non-kernel surfaces.
- The inner loop is expected to originate bounded upgrade hypotheses from recorded success/failure data.
- AgentOS may scaffold, constrain, and audit that process, but it must not replace the inner loop as the default origin of intelligence.
- AgentOS models may contribute intelligent critique and compression, but their role here is to help the inner proposer become more capable, not to permanently think in its place.

## 3. Inner Loop Authority

The inner proposer may treat these as fixed whitebox law:

- the current parity task protocol
- the current benchmark case stream and rung ladder
- the current run manifest and artifact requirements
- the anti-hardcoding rule
- the no-kernel-change rule
- the no-topology-change rule

The inner proposer may not reinterpret any of those laws.

## 4. Allowed Inner Surfaces

- prompt wording and prompt examples
- adapter transport parameters that do not change semantics
- benchmark harness wording or artifact packaging
- explicit run-policy packets prepared for outer review
- debugging hypotheses and patch proposals

## 5. Forbidden Inner Surfaces

- kernel semantics
- task truth conditions
- verifier behavior
- scoring rules
- failure accounting
- topology and host-role boundaries
- benchmark-local hardcoding
- hidden repair or benchmark-only rescue paths

## 6. Inner Loop Output Contract

Each self-upgrade cycle must produce:

- one bounded hypothesis
- one explicit evidence basis derived from recorded success/failure artifacts
- one exact claimed improvement surface
- one exact list of changed files
- one declared non-goal list
- one declared risk list
- one rerun plan against fixed rung inputs

## 7. Inner Loop Success Semantics

- Inner success means: a bounded candidate revision is ready for outer evaluation.
- Inner success does not mean: the system is allowed to declare itself improved.
- Only the outer promotion loop may accept or reject a candidate.

## 8. Anti-Goodhart Rule

- The inner proposer may not collapse "higher benchmark score" into "truer system behavior".
- It must preserve the distinction between:
  - protocol faithfulness
  - measured gate progress
  - benchmark-local coincidence

## 9. Whitebox-Owned Invariants

These invariants remain outside the inner proposer and must be treated as law:

- black-box output remains intent only
- commit authority remains whitebox-owned
- abort returns the exact prior world snapshot
- predicates and verifier gates remain authoritative
- benchmark pass/fail classification remains external

## 10. First Use In This Project

- Current target: `100` consecutive passes on the headline mode `turingosv2_planner27_worker9`
- Current active bounded loop:
  - propose non-kernel changes
  - run the declared rung
  - preserve artifacts
  - hand results to the outer promotion loop
- Directional correction:
  - the outer AgentOS loop should increasingly shift from “direct fixer” to “proposal environment, auditor, and promotion authority”
  - the intended steady state is that `turingosv2` originates the bounded proposal and AgentOS decides whether it is safe and justified to apply
  - any period where outer AgentOS authors most fixes directly should be treated as transitional bootstrap debt and reduced over time
