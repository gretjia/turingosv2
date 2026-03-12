#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PYTEST_BIN="${PYTEST_BIN:-$ROOT_DIR/.venv/bin/python}"

if [[ -n "${CARGO_BIN:-}" ]]; then
  CARGO_CMD="$CARGO_BIN"
else
  CARGO_CMD="$(command -v cargo)"
fi

cd "$ROOT_DIR"

"$CARGO_CMD" test -p turingos-kernel --test parity_golden_replay
"$CARGO_CMD" test -p turingos-observe --test parity_projection_fixture
"$PYTEST_BIN" -m pytest -q tests/test_parity_runtime.py
