# TuringOSv2 Kimi Sidecar Runbook

Status: Active
Date: 2026-03-11 UTC
Scope: Faster diagnostic and self-upgrade sidecar runs for Mission 002 without changing kernel or topology authority

## 1. Purpose

Use Kimi as a faster external sidecar lane to discover failure laws sooner.

This is for:

- bounded reruns
- sidecar diagnosis
- faster evidence compression

This is not, by default, for:

- replacing the official scored parity baseline
- rewriting the verified headline ceiling
- hiding `turingosv2` failure laws behind a much stronger model

## 2. Why This Is Low-Cost In TuringOSv2

`turingosv2` already talks to remote model providers through one bounded adapter surface:

- `harnesses/mission002_py/llama_openai.py`

That adapter now supports two request families:

- OpenAI-compatible `chat/completions`
- Kimi Coding `messages`

Both keep the same benchmark boundary:

- `base_url`
- `api_key`
- `model`

So Kimi integration remains non-kernel work.

## 3. Official Doc Basis

Kimi Code's official docs for third-party coding agents currently describe:

- provider type:
  - `OpenAI Compatible`
- entrypoint:
  - `https://api.kimi.com/coding/v1`
- model:
  - `kimi-for-coding`
- note:
  - `Use legacy OpenAI API format`

Official reference:

- https://www.kimi.com/code/docs/

## 4. Local Environment Convention

Repo-local `turingosv2` should now prefer:

- `.env`

with:

- `KIMI_API_KEY`
- `KIMI_OPENAI_BASE_URL=https://api.kimi.com/coding`
- `KIMI_OPENAI_MODEL=kimi-for-coding`

Legacy `turingos` also used these names:

- `KIMI_API_KEY`
- `TURINGOS_API_BASE_URL`
- `TURINGOS_MODEL`

The sidecar script now prefers repo-local `.env` and still supports one explicit override:

- `KIMI_OPENAI_MODEL`

## 5. Current Reality Check

Fresh 2026-03-11 probe results:

- `https://api.moonshot.cn/v1/models`
  - `401 Invalid Authentication`
- `https://api.moonshot.cn/v1/chat/completions`
  - `401 Invalid Authentication`
- `https://api.kimi.com/coding/v1/models`
  - `200`
  - available model id: `kimi-for-coding`
- `https://api.kimi.com/coding/v1/messages`
  - `200`
  - valid response with the supplied Kimi key
- `https://api.kimi.com/coding/v1/chat/completions`
  - `403`
  - `Kimi For Coding is currently only available for Coding Agents ...`

Interpretation:

- the supplied key is valid
- it is valid for the Kimi Coding endpoint family
- it is not valid for Moonshot's public `api.moonshot.cn/v1` family
- for a custom `turingosv2` sidecar, the working path is Kimi Coding `messages`, not `chat/completions`

## 5.1 Model-Selection Rule

With the currently validated key, the live available model id is:

- `kimi-for-coding`

So the immediate sidecar path is not a model-family selection problem. It is a transport-family selection problem:

- use `kimi-for-coding`
- use the Kimi Coding `messages` API
- keep the official local-qwen scored lane as the headline authority

## 6. Script

Use:

- `scripts/mission002_kimi_sidecar.sh`

It:

- sources `/home/zephryj/projects/turingos/.env` by default
- expects `KIMI_API_KEY`
- defaults to `KIMI_OPENAI_MODEL=kimi-for-coding`
- routes the planner slot through Kimi
- keeps the worker slot on the current local 9B lane
- leaves kernel and topology authority unchanged
- targets `https://api.kimi.com/coding`

## 7. Recommended Usage

For faster evidence search, start with bounded runs only:

```bash
KIMI_OPENAI_MODEL='kimi-for-coding' \
START_CASE=4 \
MAX_CASES=1 \
RUN_NAME='mission002_kimi_case4_probe' \
bash /home/zephryj/projects/turingosv2/scripts/mission002_kimi_sidecar.sh
```

Only after repeated sidecar value is demonstrated should any larger diagnostic ladder be considered.

## 8. Control-Lane Rule

The existing `turingosv2_single_9b` lane remains useful, but as a sparse diagnostic control rather than as an every-rung mandatory companion.

Use the local 9B control lane when:

- a new failure law appears and model-versus-system attribution is unclear
- a sidecar Kimi success risks masking shallow protocol drift
- a bounded rerun needs a weaker-model contrast

Do not require the 9B control lane on every scored rung, because that would slow the main parity progression without improving the official ceiling itself.

## 9. Boundary Rule

Kimi sidecar runs are diagnostic by default.

They may help TuringOSv2 find problems faster, but they do not automatically replace the official local-qwen scored lane or its verified headline ceiling.
