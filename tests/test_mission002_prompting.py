from __future__ import annotations

import json

from harnesses.mission002_py.mission002_prompting import PROMPT_VERSION, build_mission002_messages
from turingos.models import AgentView, FrozenMachineState


def _make_view() -> AgentView:
    return AgentView(
        agent_id="planner27",
        state=FrozenMachineState(
            step=3,
            current_path="dir_00_43/.ls",
            register={
                "phase": "scan",
                "todo": ["dir_00_43/.ls", "dir_01_89/.ls"],
                "pending": None,
                "answer": None,
                "completed": [".ls"],
                "files": {
                    "parity": "parity.md",
                    "result": "result.md",
                    "root_listing": ".ls",
                },
            },
            halted=False,
        ),
        current_content="DIR dir_02_98\nFILE n_00_967.md\nFILE n_02_858.md\nFILE n_03_521.md",
    )


def test_mission002_prompt_includes_scan_invariants_and_examples() -> None:
    messages = build_mission002_messages(_make_view(), role_name="planner")
    worker_messages = build_mission002_messages(_make_view(), role_name="worker")

    assert PROMPT_VERSION == "mission002.prompt.v10"
    system_prompt = messages[0]["content"]
    worker_system_prompt = worker_messages[0]["content"]
    payload = json.loads(messages[1]["content"])

    assert "When current_path ends with .ls, every DIR name must become a child path ending with '/.ls'." in system_prompt
    assert "remaining_old_todo_without_current_path + discovered_md_files + discovered_child_ls_paths" in system_prompt
    assert "Within one current .ls discovery batch, discovered FILE paths must stay ahead of discovered child DIR/.ls paths" in system_prompt
    assert "that discovered file must stay in todo even when next_path remains an older pending item" in system_prompt
    assert "On phase=scan for a numeric .md file, write_mode must be keep and write_content must be null." in system_prompt
    assert "odd -> 1, even -> 0" in system_prompt
    assert "notes must be one short plain-text line with no newlines" in system_prompt
    assert "no self-corrections" in system_prompt
    assert "do not append anything to completed" in system_prompt
    assert "keep todo unchanged" in system_prompt
    assert "Do not leave phase='apply_pending' after applying the bit." in system_prompt
    assert "listing has only FILE lines and no DIR lines" in system_prompt
    assert "Do not blend the current step with the next phase." in system_prompt
    assert "Do not speculate beyond the visible state or pre-execute the next phase." in worker_system_prompt
    assert "Use the last decimal digit only" in system_prompt
    assert "do not overwrite parity.md and do not perform apply_pending early" in system_prompt
    assert "move to parity.md only when the recomputed todo is empty" in system_prompt
    assert "overwrite the current file parity.md with '0'" in system_prompt
    assert "alpha/nested/.ls" in system_prompt
    assert "todo=['x.md','a/b/y.md']" in system_prompt
    assert "todo=['x.md','alpha/.ls']" in system_prompt
    assert "next_path='a/.ls'" in system_prompt
    assert "todo=['older.md','a/b/new1.md','a/b/new2.md']" in system_prompt
    assert "pending={'bit': 1, 'source': 'a/n_odd.md'}" in system_prompt
    assert "pending={'bit': 0, 'source': 'a/n_even.md'}" in system_prompt
    assert "current todo=['a/b/.ls'], listing='FILE z.md'" in system_prompt
    assert "todo=['a/b/z.md']" in system_prompt
    assert "do not jump to parity.md" in system_prompt
    assert set(payload["visible_state"].keys()) == {"agent_id", "state", "current_content"}
