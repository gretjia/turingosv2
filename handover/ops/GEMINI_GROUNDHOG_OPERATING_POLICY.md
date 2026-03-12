# Gemini Groundhog Operating Policy

Status: Active
Mission: Mission Groundhog
Date: 2026-03-10 UTC

## 1. Purpose

- Standardize how Groundhog uses Gemini CLI.
- Prevent premature process killing during long preview-model reasoning.
- Keep Gemini diagnostics inside native Gemini session and telemetry surfaces rather than repository-resident watchdog code.

## 2. Model Policy

- All Groundhog Gemini roles remain pinned by mission policy to `Gemini 3.1 Pro Preview`.
- Persist the default model through Gemini settings rather than per-command `--model` flags:
  - user-level path: `~/.gemini/settings.json`
  - required key:

```json
{
  "model": {
    "name": "gemini-3.1-pro-preview"
  }
}
```

- If the local CLI cannot safely honor explicit `-m` but observed session artifacts still show `gemini-3.1-pro-preview`, that observed model string remains the required proof point.
- The current Groundhog environment still treats explicit `--model gemini-3.1-pro-preview` as non-authoritative because Gemini CLI can emit the known payload bug:
  - `Invalid JSON payload received. Unknown name "model": Proto field is not repeating, cannot start list.`
- When a stage-close Gemini verdict is authority-bearing, do not trust one probe mismatch as final proof of model drift.
  - observed on this VM: a headless probe can transiently return `gemini-2.5-pro` even while `~/.gemini/settings.json` still pins `gemini-3.1-pro-preview`
  - required handling:
    - run a fresh headless probe immediately before the authoritative audit
    - if one probe mismatches, allow a small bounded retry window
    - Groundhog default: up to `3` fresh probes with a short pause
    - only verdicts preceded by a successful `gemini-3.1-pro-preview` probe count toward stage close

## 3. Default Command Form

Groundhog Gemini roles should default to:

```bash
/usr/bin/gemini --output-format stream-json -p "<bounded role packet>"
```

Use plain `text` output only when a human is reading directly and structured progress events are not needed.

On this Google Cloud VM, do not prefer the user-wrapper command at `~/.local/bin/gemini` for Groundhog role work.

- current wrapper contents:
  - `exec /usr/bin/gemini -y --model gemini-3.1-pro-preview "$@"`
- why it is non-preferred:
  - it injects the explicit `--model` flag path that can still trigger the known Gemini CLI payload bug in this environment
- Groundhog command policy therefore prefers the direct binary:
  - `/usr/bin/gemini`

## 4. Session Policy

- Prefer one fresh Gemini process per bounded role task.
- Use `--resume latest` only when all of the following remain true:
  - same role
  - same workspace
  - same bounded task family
- Do not resume one Gemini session across unrelated audit roles.

## 5. Prompt Policy

Gemini role packets should be:

- short
- verdict-shaped
- file-bounded
- mission-bounded

Avoid:

- giant multipurpose prompts
- broad repo scans when the role only needs a narrow file set
- stacking unrelated asks into one preview-model call

## 6. Silence Policy

- Long silence is not by itself proof of failure.
- Groundhog must not kill a Gemini process merely because no human-readable text has appeared yet.
- A run is considered plausibly alive when any of the following is true:
  - `stream-json` emitted an `init` event
  - a session artifact appears under `~/.gemini/tmp/<project>/chats/`
  - the observed model string is present
  - the session timestamp continues advancing

## 7. Timeout Policy

- Do not use aggressive short timeout wrappers for Groundhog Gemini roles.
- For bounded audit prompts, allow roughly `2-3 minutes` before treating the run as likely stuck.
- Only after the diagnosis steps below should an outer `timeout` wrapper be treated as justified automation safety.

## 8. Diagnosis Order

Before killing a Gemini run, check in this order:

1. Is the process still alive?
2. Did `stream-json` emit `init` or later events?
3. Did a new session file appear under `~/.gemini/tmp/<project>/chats/`?
4. Does the session file show the expected model?
5. Are session timestamps still advancing?
6. Does a fresh headless identity probe still return the pinned model?

Recommended probe:

```bash
/usr/bin/gemini --output-format text -p \
  'State the exact active model id for this session and nothing else.'
```

Only if these checks stop showing life should the run be treated as likely hung.

## 8.1 Model Mismatch Rule

- A one-off headless model mismatch is not automatically stage-fatal.
- First treat it as an environment-integrity warning.
- Then run a bounded fresh-probe sequence before an authority-bearing audit:
  - up to `3` probes
  - short pause between probes
- If the bounded retry window still fails to observe `gemini-3.1-pro-preview`, record the mismatch and treat the affected audit as non-authoritative for stage close.

## 9. Native Observability Rule

- Prefer native Gemini evidence:
  - `stream-json` events
  - session files under `~/.gemini/tmp/`
  - history under `~/.gemini/history/`
  - built-in telemetry if later enabled
- Do not add repository-resident watchdog or supervisor programs for Gemini process management.

## 10. Source Of Truth

- Stable operational policy lives in this file.
- Research evidence and dated observations live in:
  - `handover/ops/GEMINI_CLI_MULTI_AGENT_RESEARCH_20260310.md`
