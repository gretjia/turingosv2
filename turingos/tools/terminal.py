from __future__ import annotations

import shlex
import subprocess
from dataclasses import dataclass, field
from pathlib import Path


class UnsafeCommandError(ValueError):
    pass


@dataclass(slots=True)
class WhitelistedTerminalTool:
    workspace_root: Path
    allowed_commands: set[str] = field(default_factory=lambda: {"ls", "cat", "pwd", "echo", "touch", "mkdir"})

    def run(self, command: str) -> subprocess.CompletedProcess[str]:
        argv = shlex.split(command)
        if not argv:
            raise UnsafeCommandError("empty command")
        if argv[0] not in self.allowed_commands:
            raise UnsafeCommandError(f"command not allowed: {argv[0]}")
        for token in argv[1:]:
            if token in {";", "&&", "||", "|", ">", ">>", "<", "$(", "`"}:
                raise UnsafeCommandError("shell metacharacters are not allowed")
            if token.startswith("/"):
                raise UnsafeCommandError("absolute paths are not allowed")
            if ".." in token.split("/"):
                raise UnsafeCommandError("path traversal is not allowed")
        return subprocess.run(
            argv,
            cwd=str(self.workspace_root),
            text=True,
            capture_output=True,
            check=False,
        )
