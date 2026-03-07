from __future__ import annotations

from pathlib import Path

import pytest

from turingos.fs import PathEscapeError, WorkspaceFS


def test_workspace_fs_roundtrip_and_ls_generation(tmp_path: Path) -> None:
    fs = WorkspaceFS(tmp_path)
    fs.mkdir("a")
    fs.write_text("a/1.md", "1")
    fs.write_text("root.md", "42")
    fs.generate_ls_files()

    assert fs.read_text("a/1.md") == "1"
    assert "FILE root.md" in fs.read_text(".ls")
    assert "FILE 1.md" in fs.read_text("a/.ls")
    assert sorted(fs.walk_files(suffix=".md")) == ["a/1.md", "root.md"]


def test_workspace_fs_blocks_escape(tmp_path: Path) -> None:
    fs = WorkspaceFS(tmp_path)
    with pytest.raises(PathEscapeError):
        fs.resolve("../outside.txt")
