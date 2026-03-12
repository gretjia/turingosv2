# Gemini CLI Multi-Agent Research

Date: 2026-03-10 UTC
Scope: Mission Groundhog post-round research on Gemini CLI headless usage, multi-agent operating patterns, long silent runs, and timeout diagnosis on the current Google Cloud VM

## 1. Executive Conclusion

- On this VM, the repeated Gemini CLI problem is not well described as “it times out all the time”.
- The stronger local evidence is:
  - Gemini headless sessions often go silent for long stretches
  - many of those silent windows still end in valid terminal answers
  - at least some of the silence corresponds to internal tool-call work or long preview-model reasoning, not network failure
- Therefore Groundhog should not treat silence alone as failure.
- The best current operating pattern is:
  - use one fresh headless Gemini process per bounded role task
  - prefer `gemini --output-format stream-json -p "<role packet>"`
  - keep prompts short, role-scoped, and file-bounded
  - use `--resume latest` only when continuing the exact same role thread
  - diagnose failures from native Gemini session artifacts under `~/.gemini/`, not from a repository watchdog

## 2. Official Findings

### 2.1 Headless Mode

Official Gemini CLI documentation defines `-p/--prompt` as the non-interactive headless path.

Source:

- https://raw.githubusercontent.com/google-gemini/gemini-cli/main/docs/cli/headless.md

Operational implication:

- Groundhog external Gemini roles should keep using headless `-p` for bounded audits and research roles.

### 2.2 Structured Output Is Better Than Plain Text For Monitoring

Official docs define structured headless output formats:

- `--output-format text`
- `--output-format json`
- `--output-format stream-json`

Sources:

- https://raw.githubusercontent.com/google-gemini/gemini-cli/main/docs/cli/headless.md
- local CLI help from `gemini --help`

Operational implication:

- For machine-observed multi-agent work, `stream-json` is the best default because it emits early lifecycle events before the final answer.
- `text` is acceptable for manual reading, but it makes long silence look indistinguishable from a hang.

### 2.3 Session Management

Official session docs expose:

- `--resume latest`
- session listing
- delete session controls

Source:

- https://raw.githubusercontent.com/google-gemini/gemini-cli/main/docs/cli/sessions.md

Operational implication:

- Use a fresh session for most Groundhog audit roles.
- Use `--resume latest` only for same-role continuation where context continuity is intentionally desired.
- Do not casually pile unrelated audit roles into one resumed Gemini session.

### 2.4 Context Control

Official docs define two strong context controls:

- hierarchical `GEMINI.md`
- `--include-directories`

Sources:

- https://raw.githubusercontent.com/google-gemini/gemini-cli/main/docs/core/gemini.md
- https://raw.githubusercontent.com/google-gemini/gemini-cli/main/docs/cli/headless.md

Operational implication:

- Best multi-agent behavior comes from narrow context, not giant prompts.
- Groundhog Gemini roles should receive only the exact files they need, plus constitution and current phase docs.

### 2.5 Turn Bounding

Official configuration docs define `maxSessionTurns`.

Source:

- https://raw.githubusercontent.com/google-gemini/gemini-cli/main/docs/cli/configuration.md

Operational implication:

- For bounded audit roles, low turn counts are preferable.
- This reduces accidental wandering or hidden long tool loops in role-shaped headless runs.

### 2.6 Built-In Telemetry Exists

Official telemetry docs describe built-in metrics, logs, and traces export through OpenTelemetry.

Source:

- https://raw.githubusercontent.com/google-gemini/gemini-cli/main/docs/core/telemetry.md

Operational implication:

- If Gemini debugging becomes a recurring engineering concern, prefer Gemini’s own telemetry/session artifacts over building a custom watcher.
- This stays compatible with the project rule that no repository-resident watchdog or supervisor program should be introduced.

## 3. Officially Visible Timeout And Connection Risks

The official repository already contains relevant issue traffic showing that some failures are real CLI/backend/integration issues, not just local operator error.

Examples:

- issue `#14110`: `streamGenerateContent` timing out after roughly 120 seconds
  - https://github.com/google-gemini/gemini-cli/issues/14110
- issue `#12362`: CLI losing connection after network changes
  - https://github.com/google-gemini/gemini-cli/issues/12362

Interpretation:

- Some timeout behavior is genuinely upstream or transport-related.
- But those official issues do not justify treating every long silent run as dead.

## 4. Local VM Evidence

### 4.1 Environment Facts

- Host: `omega-vm`
- OS: Debian/Linux on Google Cloud
- Machine type: `e2-standard-4`
- Zone: `us-central1-c`
- Gemini CLI version: `0.32.1`

### 4.2 Observed Model

Recent local Gemini session artifacts under `~/.gemini/tmp/turingosv2/chats/` show:

- observed model string: `gemini-3.1-pro-preview`

This matters because Mission Groundhog policy pins Gemini work to `Gemini 3.1 Pro Preview`.

### 4.3 Session Duration Sample

From 17 local `turingosv2` Gemini sessions on 2026-03-10 UTC:

- minimum duration: `9.676s`
- median duration: `38.317s`
- maximum duration: `162.337s`

Long successful examples:

- one session ran `162.337s` and still ended in a valid `PASS WITH FIXES`
- one session ran `116.414s` and still ended in `PASS`
- one session ran `106.516s` and still ended successfully

### 4.4 Silent Windows Inside Successful Runs

Local session traces show long quiet gaps before valid completion:

- one successful audit had a `99.359s` gap between Gemini messages before continuing
- another had a `108.581s` silent gap and still returned `PASS`

Conclusion:

- Long silence on this VM is not sufficient evidence of timeout.
- At least some silence reflects preview-model reasoning or internal tool execution.

### 4.5 Stream-JSON Behavior

Local test command:

```bash
gemini --output-format stream-json -p 'Reply with exactly OK.'
```

Observed behavior:

- immediate `init` event
- immediate user echo event
- final assistant/result events after about `6.1s`

Conclusion:

- `stream-json` gives Groundhog a native way to distinguish “session started and model selected” from “no visible progress at all”.

## 5. Best Current Operating Pattern For Groundhog

### 5.1 Role Invocation Pattern

Use one fresh process per bounded Gemini role:

```bash
gemini --output-format stream-json -p "<bounded role packet>"
```

Recommended use:

- math constitution audits
- formal methods audits
- Turing-machine semantics audits
- AGI direction audits
- recursive math audits

### 5.2 Prompt Shape

Keep prompts:

- short
- verdict-shaped
- file-bounded
- mission-bounded

Avoid:

- giant multipurpose prompts
- multiple unrelated asks in one Gemini call
- broad repository scans unless truly needed

### 5.3 Silence Handling

Treat a Gemini run as likely still alive when:

- a session file appears under `~/.gemini/tmp/<project>/chats/`
- the observed model is `gemini-3.1-pro-preview`
- `stream-json` emitted `init`
- recent session timestamps continue to advance

Treat it as more likely hung or failed when:

- there is no new session artifact
- no `init` event appears in `stream-json`
- the process exits non-zero
- the session shows no timestamp movement for an evidence-based threshold

### 5.4 Timeout Discipline

Do not use aggressive short timeouts for preview-model audits.

Current practical recommendation:

- allow roughly `2-3 minutes` for bounded audit prompts before declaring a likely hang
- prefer `stream-json` plus local session inspection before killing the run
- only then apply outer `timeout` wrappers for automation safety

### 5.5 Session Reuse

Use `--resume latest` only when:

- it is the same role
- the same workspace
- the same bounded task family

Do not resume across:

- different audit roles
- different Groundhog stages
- code review versus mathematical audit modes

### 5.6 Context Hygiene

Prefer:

- repo-local `GEMINI.md` guidance
- exact file lists
- current phase spec + constitution + touched files

Avoid:

- feeding all of `handover/` every time
- mixing frozen authority docs with many historical drafts when the role does not need them

## 6. What Groundhog Should Change Operationally

Groundhog should now standardize the following for Gemini roles:

1. Default headless command form:

```bash
gemini --output-format stream-json -p "<role packet>"
```

2. Default diagnosis order:

- check process still alive
- check `stream-json` init/result events
- check latest session artifact in `~/.gemini/tmp/<project>/chats/`
- only then decide whether the run is genuinely stuck

3. Default timeout posture:

- no premature kill during the first long silent window
- use evidence-based kill thresholds, not reflex

4. Default context discipline:

- smaller prompts
- narrower file sets
- one role per process

## 7. Bottom Line

- Gemini CLI on this GCP VM does show real long silent runs.
- But the local evidence does not support the claim that those are usually network timeouts.
- For Groundhog, the safer interpretation is:
  - some silence is genuine long reasoning or tool work
  - some timeouts are real upstream/transport issues
  - the right response is better instrumentation and operating discipline, not tighter impatience

