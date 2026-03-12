from __future__ import annotations

import json
from pathlib import Path
from typing import Any


def ensure_dir(path: Path) -> None:
    path.mkdir(parents=True, exist_ok=True)


def write_json(path: Path, payload: dict[str, Any]) -> None:
    ensure_dir(path.parent)
    path.write_text(json.dumps(payload, ensure_ascii=True, indent=2, sort_keys=True), encoding="utf-8")


def write_text(path: Path, content: str) -> None:
    ensure_dir(path.parent)
    path.write_text(content, encoding="utf-8")


def collect_workspace_snapshot(workspace_root: Path) -> dict[str, str]:
    snapshot: dict[str, str] = {}
    if not workspace_root.exists():
        return snapshot
    for path in sorted(workspace_root.rglob("*")):
        if not path.is_file():
            continue
        rel = path.relative_to(workspace_root).as_posix()
        snapshot[rel] = path.read_text(encoding="utf-8")
    return snapshot


def read_tail_lines(path: Path, max_lines: int = 40) -> list[str]:
    if not path.exists():
        return []
    lines = path.read_text(encoding="utf-8").splitlines()
    return lines[-max_lines:]
