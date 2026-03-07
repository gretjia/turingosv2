from __future__ import annotations

from pathlib import Path

import pytest

from turingos.tools import UnsafeCommandError, WhitelistedTerminalTool


def test_terminal_tool_allows_safe_commands(tmp_path: Path) -> None:
    (tmp_path / "hello.txt").write_text("hi", encoding="utf-8")
    tool = WhitelistedTerminalTool(workspace_root=tmp_path)
    result = tool.run("cat hello.txt")
    assert result.returncode == 0
    assert result.stdout == "hi"


def test_terminal_tool_blocks_unsafe_commands(tmp_path: Path) -> None:
    tool = WhitelistedTerminalTool(workspace_root=tmp_path)
    with pytest.raises(UnsafeCommandError):
        tool.run("rm -rf /")
    with pytest.raises(UnsafeCommandError):
        tool.run("cat ../secret")
