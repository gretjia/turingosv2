# Mission Groundhog Constitution Index

Status: Active
Mission: Mission Groundhog
Date: 2026-03-09 UTC

## 1. Canonical Constitution

- Sealed file: `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
- Authority class: immutable constitutional source
- Scope: mathematical law, topology law, commit/abort law, and black-box/white-box boundary law for Mission Groundhog and all derived engineering work

## 2. Freeze Rules

- No agent may edit the sealed constitution after this commit of record.
- No formatting-only cleanup is allowed on the sealed constitution.
- No “clarifying paraphrase” may be inserted into the sealed constitution itself.
- Any commentary, interpretation, Rust mapping, proof note, or audit must live outside the sealed file.
- If a future human owner wants a new constitution, that must be a new sealed file plus a new handover supersession record. The old constitution remains frozen.

## 3. Authority Order

For Mission Groundhog, authority is ordered as follows:

1. `bible/GROUNDHOG_SEALED_CONSTITUTION.md`
2. `handover/ops/ACTIVE_MISSION_CHARTER.md`
3. `handover/ops/MISSION_GROUNDHOG_IMPLEMENTATION_GUARDRAILS.md`
4. `handover/ops/MISSION_GROUNDHOG_TEAM.md`
5. `README.md`
6. derived implementation docs and code

If any derived engineering document conflicts with the sealed constitution, the constitution wins and the engineering work must stop.

## 4. Derived Documents

- `handover/ops/ACTIVE_MISSION_CHARTER.md`
- `handover/ops/MISSION_GROUNDHOG_IMPLEMENTATION_GUARDRAILS.md`
- `handover/ops/MISSION_GROUNDHOG_TEAM.md`
- future Rust design docs, proof notes, and implementation records under `handover/`

## 5. Practical Meaning

- The sealed constitution fixes the meaning of:
  - `Q_t`
  - `rtool`
  - `delta`
  - `wtool`
  - predicate-product validation
  - atomic commit versus abort
- It also fixes one critical negative rule:
  - no path may exist by which the black-box model directly writes the next universe state

## 6. Mission Groundhog Interpretation Rule

- Mission Groundhog is allowed to engineer, encode, prove, test, and operationalize the constitution.
- Mission Groundhog is not allowed to mutate the constitution while doing so.
