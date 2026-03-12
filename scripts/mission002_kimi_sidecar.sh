#!/usr/bin/env bash
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv2"
ENV_FILE="${KIMI_ENV_FILE:-${ROOT}/.env}"
LEGACY_ENV_FILE="/home/zephryj/projects/turingos/.env"

if [[ -f "${ENV_FILE}" ]]; then
  set -a
  # shellcheck disable=SC1090
  source "${ENV_FILE}"
  set +a
elif [[ -f "${LEGACY_ENV_FILE}" ]]; then
  set -a
  # shellcheck disable=SC1090
  source "${LEGACY_ENV_FILE}"
  set +a
fi

KIMI_OPENAI_BASE_URL="${KIMI_OPENAI_BASE_URL:-https://api.kimi.com/coding}"
KIMI_OPENAI_MODEL="${KIMI_OPENAI_MODEL:-kimi-for-coding}"
KIMI_API_KEY="${KIMI_API_KEY:-}"

if [[ -z "${KIMI_API_KEY}" ]]; then
  echo "KIMI_API_KEY is missing. Export it directly or provide it through ${LEGACY_ENV_FILE}." >&2
  exit 1
fi

if [[ -z "${KIMI_OPENAI_MODEL}" ]]; then
  echo "KIMI_OPENAI_MODEL is missing. Probe ${KIMI_OPENAI_BASE_URL}/v1/models and set a valid Kimi Coding model id." >&2
  exit 1
fi

START_CASE="${START_CASE:-1}"
MAX_CASES="${MAX_CASES:-1}"
MAX_STEPS="${MAX_STEPS:-64}"
MAX_TOKENS="${MAX_TOKENS:-768}"
TIMEOUT_SECONDS="${TIMEOUT_SECONDS:-60}"
RETRY_BUDGET="${RETRY_BUDGET:-1}"
GENERATOR_VERSION="${GENERATOR_VERSION:-mission002.parity.v1}"
GLOBAL_SEED="${GLOBAL_SEED:-20260308}"
RUN_NAME="${RUN_NAME:-mission002_kimi_sidecar_$(date -u +%Y%m%d_%H%M%SUTC)}"
QWEN9_BASE_URL="${QWEN9_BASE_URL:-http://127.0.0.1:18083}"
QWEN9_MODEL="${QWEN9_MODEL:-Qwen3.5-9B-Q4_K_M.gguf}"
QWEN9_API_KEY="${QWEN9_API_KEY:-local}"
TOPOLOGY_HOST_ALIAS="${TOPOLOGY_HOST_ALIAS:-zephrymac-studio}"
REPO_REVISION="$(git -C "${ROOT}" rev-parse HEAD)-dirty"

exec "${ROOT}/.venv/bin/python" -m turingos.cli benchmark-mission002 \
  --modes turingosv2_planner27_worker9 \
  --start-case "${START_CASE}" \
  --max-cases "${MAX_CASES}" \
  --global-seed "${GLOBAL_SEED}" \
  --generator-version "${GENERATOR_VERSION}" \
  --max-steps "${MAX_STEPS}" \
  --output-root "${ROOT}/benchmarks/mission002" \
  --run-name "${RUN_NAME}" \
  --clean \
  --qwen27-base-url "${KIMI_OPENAI_BASE_URL}" \
  --qwen27-model "${KIMI_OPENAI_MODEL}" \
  --qwen27-api-key "${KIMI_API_KEY}" \
  --qwen9-base-url "${QWEN9_BASE_URL}" \
  --qwen9-model "${QWEN9_MODEL}" \
  --qwen9-api-key "${QWEN9_API_KEY}" \
  --timeout-seconds "${TIMEOUT_SECONDS}" \
  --retry-budget "${RETRY_BUDGET}" \
  --max-tokens "${MAX_TOKENS}" \
  --topology-host-alias "${TOPOLOGY_HOST_ALIAS}" \
  --qwen27-remote-target "api.kimi.com/coding" \
  --qwen9-remote-target "127.0.0.1:8081" \
  --qwen27-launch-flags "kimi coding legacy messages provider" \
  --qwen9-launch-flags "--host 127.0.0.1 --port 8081 -c 8192 --reasoning-format none --reasoning-budget 0" \
  --topology-notes "Kimi Coding planner sidecar for faster bounded diagnosis; official scored baseline remains local-qwen authority unless explicitly promoted." \
  --repo-revision "${REPO_REVISION}"
