from __future__ import annotations

import copy
from dataclasses import dataclass, field
from pathlib import PurePosixPath
from typing import Any

from ..fs import WorkspaceFS
from ..models import MachineState, SignalBundle, Transition


def _join(base: str, name: str) -> str:
    if not base or base == ".":
        return name
    return f"{base}/{name}"


def _dir_of(path: str) -> str:
    parent = str(PurePosixPath(path).parent)
    return "" if parent == "." else parent


def _parse_listing(content: str) -> tuple[list[str], list[str]]:
    dirs: list[str] = []
    files: list[str] = []
    for raw in content.splitlines():
        line = raw.strip()
        if not line:
            continue
        kind, _, name = line.partition(" ")
        if kind == "DIR":
            dirs.append(name)
        elif kind == "FILE":
            files.append(name)
    return dirs, files


@dataclass(slots=True)
class ParityTask:
    """Filesystem parity task from the article, wrapped as a deterministic benchmark."""

    tree: dict[str, Any] = field(default_factory=dict)
    name: str = "parity"
    parity_path: str = "parity.md"
    result_path: str = "result.md"
    expected_answer: str = ""

    def __post_init__(self) -> None:
        if not self.tree:
            self.tree = {
                "32342323.md": "32342323",
                "1": {"100.md": "100"},
                "2": {"200.md": "200"},
                "3": {"300.md": "300"},
                "4": {},
            }

    def setup(self, fs: WorkspaceFS) -> None:
        self._materialize_tree(fs, self.tree)
        fs.generate_ls_files()
        self.expected_answer = self._compute_expected_answer(fs)

    def _materialize_tree(self, fs: WorkspaceFS, tree: dict[str, Any], base: str = "") -> None:
        for name, value in tree.items():
            rel = _join(base, name)
            if isinstance(value, dict):
                fs.mkdir(rel)
                self._materialize_tree(fs, value, rel)
            else:
                fs.write_text(rel, str(value))

    def _numeric_files(self, fs: WorkspaceFS) -> list[str]:
        files = []
        for rel in fs.walk_files(suffix=".md"):
            name = PurePosixPath(rel).name
            if name in {self.parity_path, self.result_path}:
                continue
            files.append(rel)
        return sorted(files)

    def _compute_expected_answer(self, fs: WorkspaceFS) -> str:
        parity = 0
        for rel in self._numeric_files(fs):
            value = int(fs.read_text(rel).strip())
            parity ^= (value & 1)
        return "odd" if parity else "even"

    def initial_state(self) -> MachineState:
        return MachineState(
            step=0,
            current_path=".ls",
            register={
                "phase": "boot",
                "todo": [],
                "pending": None,
                "answer": None,
                "completed": [],
                "files": {
                    "parity": self.parity_path,
                    "result": self.result_path,
                    "root_listing": ".ls",
                },
            },
            halted=False,
        )

    def _completed(self, reg: dict[str, Any], path: str) -> dict[str, Any]:
        new_reg = copy.deepcopy(reg)
        completed = list(new_reg.get("completed", []))
        if path not in completed:
            completed.append(path)
        new_reg["completed"] = completed
        return new_reg

    def _dedupe_preserve(self, items: list[str], completed: list[str]) -> list[str]:
        seen = set(completed)
        output: list[str] = []
        for item in items:
            if item in seen:
                continue
            if item in output:
                continue
            output.append(item)
        return output

    def expected_transition(self, state: MachineState, current_content: str, fs: WorkspaceFS) -> Transition:
        reg = copy.deepcopy(state.register)
        phase = reg.get("phase")
        todo = list(reg.get("todo", []))
        pending = reg.get("pending")
        parity_path = reg["files"]["parity"]
        result_path = reg["files"]["result"]
        current_path = state.current_path

        if phase == "boot":
            reg["phase"] = "init_parity"
            return Transition(next_register=reg, next_path=parity_path, write_mode="keep")

        if phase == "init_parity":
            reg["phase"] = "scan"
            reg["todo"] = []
            return Transition(
                next_register=reg,
                next_path=reg["files"]["root_listing"],
                write_mode="overwrite",
                write_content="0",
            )

        if phase == "scan":
            if current_path.endswith(".ls"):
                dirs, files = _parse_listing(current_content)
                base_dir = _dir_of(current_path)
                discovered_files = [_join(base_dir, name) for name in files if name.endswith(".md")]
                discovered_dirs = [_join(_join(base_dir, name), ".ls") for name in dirs]
                remaining = [item for item in todo if item != current_path]
                completed = list(reg.get("completed", []))
                if current_path not in completed:
                    completed.append(current_path)
                reg["completed"] = completed
                reg["todo"] = self._dedupe_preserve(remaining + discovered_files + discovered_dirs, completed)
                if reg["todo"]:
                    return Transition(next_register=reg, next_path=reg["todo"][0], write_mode="keep")
                reg["phase"] = "finalize"
                return Transition(next_register=reg, next_path=parity_path, write_mode="keep")

            if current_path.endswith(".md") and PurePosixPath(current_path).name not in {self.parity_path, self.result_path}:
                value = int(current_content.strip())
                bit = value & 1
                remaining = [item for item in todo if item != current_path]
                reg = self._completed(reg, current_path)
                reg["todo"] = remaining
                reg["pending"] = {"bit": bit, "source": current_path}
                reg["phase"] = "apply_pending"
                return Transition(next_register=reg, next_path=parity_path, write_mode="keep")

            # defensive fallback: route back to first todo or finalize.
            if todo:
                return Transition(next_register=reg, next_path=todo[0], write_mode="keep")
            reg["phase"] = "finalize"
            return Transition(next_register=reg, next_path=parity_path, write_mode="keep")

        if phase == "apply_pending":
            bit = int((pending or {}).get("bit", 0))
            current = int((current_content or "0").strip() or "0")
            new_value = str(current ^ bit)
            reg["pending"] = None
            if reg.get("todo"):
                reg["phase"] = "scan"
                next_path = reg["todo"][0]
            else:
                reg["phase"] = "finalize"
                next_path = parity_path
            return Transition(next_register=reg, next_path=next_path, write_mode="overwrite", write_content=new_value)

        if phase == "finalize":
            if current_path != parity_path:
                return Transition(next_register=reg, next_path=parity_path, write_mode="keep")
            answer = "odd" if int((current_content or "0").strip() or "0") else "even"
            reg["answer"] = answer
            reg["phase"] = "write_result"
            return Transition(next_register=reg, next_path=result_path, write_mode="keep")

        if phase == "write_result":
            if current_path != result_path:
                return Transition(next_register=reg, next_path=result_path, write_mode="keep")
            reg["phase"] = "halt"
            return Transition(
                next_register=reg,
                next_path=result_path,
                write_mode="overwrite",
                write_content=str(reg.get("answer") or self.expected_answer),
                halt=True,
            )

        if phase == "halt":
            return Transition(next_register=reg, next_path=current_path, write_mode="keep", halt=True)

        raise ValueError(f"unknown phase: {phase}")

    def verify_proposal(self, state: MachineState, current_content: str, proposal: Transition, fs: WorkspaceFS) -> SignalBundle:
        expected = self.expected_transition(state, current_content, fs)
        mismatches: list[str] = []
        public: list[str] = []
        private: list[str] = []

        def add(kind: str, detail: str) -> None:
            mismatches.append(f"{kind}:{detail}")

        if proposal.next_path != expected.next_path:
            add("wrong_path", f"expected next_path={expected.next_path}, got {proposal.next_path}")
            public.append("典型错误：读写头移动到了错误路径。")

        if proposal.write_mode != expected.write_mode:
            add("wrong_write_mode", f"expected write_mode={expected.write_mode}, got {proposal.write_mode}")
            public.append("典型错误：写入模式不符合协议。")

        if (proposal.write_content or "") != (expected.write_content or ""):
            add(
                "wrong_write_content",
                f"expected write_content={expected.write_content!r}, got {proposal.write_content!r}",
            )
            public.append("典型错误：写入内容与白盒规则不一致。")

        if proposal.halt != expected.halt:
            add("wrong_halt", f"expected halt={expected.halt}, got {proposal.halt}")
            public.append("典型错误：停机判定错误。")

        if proposal.next_register != expected.next_register:
            add("wrong_register", "next_register diverged from expected policy")
            public.append("典型错误：状态寄存器 q' 漂移。")

        # Generic boundary checks.
        try:
            fs.resolve(proposal.next_path)
        except Exception as exc:
            add("path_escape", str(exc))
            public.append("典型错误：访问越界路径。")

        if fs.is_protected(state.current_path) and proposal.write_mode != "keep":
            add("write_protected", f"attempted to write protected file {state.current_path}")
            public.append("典型错误：试图写入受保护文件。")

        if not mismatches:
            progress = len(expected.next_register.get("completed", [])) / max(1, len(self._numeric_files(fs)) + len(fs.search("DIR", suffix=".ls")) + 1)
            return SignalBundle(
                hard_pass=True,
                score=1.0 + progress,
                score_components={"exact_match": 1.0, "progress": progress},
                public_feedback=[],
                private_feedback=[],
                error_fingerprints=[],
            )

        private.extend(mismatches)
        fingerprints = [m.split(":", 1)[0] for m in mismatches]
        return SignalBundle(
            hard_pass=False,
            score=-float(len(mismatches)),
            hard_fail_reasons=mismatches,
            public_feedback=sorted(set(public)),
            private_feedback=private,
            error_fingerprints=sorted(set(fingerprints)),
        )

    def is_success(self, state: MachineState, fs: WorkspaceFS) -> bool:
        if not state.halted:
            return False
        result = fs.read_text(self.result_path).strip()
        return result == self.expected_answer

    def diagnostics(self, fs: WorkspaceFS) -> dict:
        return {
            "expected_answer": self.expected_answer or self._compute_expected_answer(fs),
            "numeric_files": self._numeric_files(fs),
            "result": fs.read_text(self.result_path).strip(),
            "parity": fs.read_text(self.parity_path).strip(),
        }
