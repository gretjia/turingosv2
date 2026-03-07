from __future__ import annotations

import hashlib
import os
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable


class PathEscapeError(ValueError):
    pass


@dataclass(slots=True)
class WorkspaceFS:
    root: Path
    protected_filenames: set[str] | None = None
    hidden_listing_names: set[str] | None = None

    def __post_init__(self) -> None:
        self.root = self.root.resolve()
        self.root.mkdir(parents=True, exist_ok=True)
        if self.protected_filenames is None:
            self.protected_filenames = {".ls"}
        if self.hidden_listing_names is None:
            self.hidden_listing_names = {".ls", "parity.md", "result.md", "ledger.jsonl"}

    def resolve(self, rel_path: str) -> Path:
        if rel_path.startswith("/"):
            candidate = (self.root / rel_path.removeprefix("/")).resolve()
        else:
            candidate = (self.root / rel_path).resolve()
        try:
            candidate.relative_to(self.root)
        except ValueError as exc:
            raise PathEscapeError(f"path escapes workspace: {rel_path}") from exc
        return candidate

    def rel(self, rel_path: str) -> str:
        resolved = self.resolve(rel_path)
        return resolved.relative_to(self.root).as_posix()

    def exists(self, rel_path: str) -> bool:
        return self.resolve(rel_path).exists()

    def is_dir(self, rel_path: str) -> bool:
        return self.resolve(rel_path).is_dir()

    def read_text(self, rel_path: str, default: str = "") -> str:
        path = self.resolve(rel_path)
        if not path.exists():
            return default
        return path.read_text(encoding="utf-8")

    def write_text(self, rel_path: str, content: str) -> None:
        path = self.resolve(rel_path)
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content, encoding="utf-8")

    def mkdir(self, rel_path: str) -> None:
        self.resolve(rel_path).mkdir(parents=True, exist_ok=True)

    def list_dir(self, rel_path: str = ".") -> list[str]:
        path = self.resolve(rel_path)
        if not path.exists() or not path.is_dir():
            return []
        return sorted(p.name for p in path.iterdir())

    def generate_ls_files(self) -> None:
        for dirpath, dirnames, filenames in os.walk(self.root):
            dirnames[:] = sorted(dirnames)
            filenames = sorted(filenames)
            rel_dir = Path(dirpath).resolve().relative_to(self.root).as_posix()
            if rel_dir == ".":
                rel_dir = ""
            lines: list[str] = []
            for name in dirnames:
                if name.startswith("."):
                    continue
                lines.append(f"DIR {name}")
            for name in filenames:
                if name in self.hidden_listing_names or name.startswith("."):
                    continue
                lines.append(f"FILE {name}")
            listing = "\n".join(lines)
            target = (Path(rel_dir) / ".ls").as_posix() if rel_dir else ".ls"
            self.write_text(target, listing)

    def walk_files(self, suffix: str | None = None) -> list[str]:
        files: list[str] = []
        for path in self.root.rglob("*"):
            if not path.is_file():
                continue
            rel = path.relative_to(self.root).as_posix()
            if suffix and not rel.endswith(suffix):
                continue
            files.append(rel)
        return sorted(files)

    def search(self, pattern: str, suffix: str | None = None) -> list[str]:
        results: list[str] = []
        for rel in self.walk_files(suffix=suffix):
            if pattern in self.read_text(rel):
                results.append(rel)
        return results

    def file_digest(self, rel_path: str) -> str:
        content = self.read_text(rel_path)
        return hashlib.sha256(content.encode("utf-8")).hexdigest()

    def tree_digests(self, include_hidden: bool = False) -> dict[str, str]:
        digests: dict[str, str] = {}
        for rel in self.walk_files():
            if not include_hidden and Path(rel).name.startswith("."):
                continue
            digests[rel] = self.file_digest(rel)
        return digests

    def snapshot(self, include_hidden: bool = False) -> dict[str, str]:
        snapshot: dict[str, str] = {}
        for rel in self.walk_files():
            if not include_hidden and Path(rel).name.startswith("."):
                continue
            snapshot[rel] = self.read_text(rel)
        return snapshot

    def is_protected(self, rel_path: str) -> bool:
        return Path(rel_path).name in (self.protected_filenames or set())

    def ensure_paths(self, paths: Iterable[str]) -> None:
        for rel in paths:
            self.resolve(rel)
