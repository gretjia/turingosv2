from __future__ import annotations

import json
from typing import Any

from turingos.models import AgentView


PROMPT_VERSION = "mission002.prompt.v10"


def _role_instruction(role_name: str) -> str:
    if role_name == "planner":
        return (
            "You are the planner lane for a TuringOS baseline benchmark. "
            "Optimize for whole-run consistency, but you still must output only one compact current-step transition. "
            "Do not blend the current step with the next phase."
        )
    return (
        "You are the worker lane for a TuringOS baseline benchmark. "
        "Optimize for exact current-step correctness. Do not speculate beyond the visible state or pre-execute the next phase."
    )


def _view_payload(view: AgentView) -> dict[str, Any]:
    return {
        "agent_id": view.agent_id,
        "state": {
            "step": view.state.step,
            "current_path": view.state.current_path,
            "register": view.state.register,
            "halted": view.state.halted,
        },
        "current_content": view.current_content,
    }


def build_mission002_messages(view: AgentView, *, role_name: str) -> list[dict[str, str]]:
    system_prompt = "\n".join(
        [
            "You are participating in Mission 002 of the TuringOSv2 pure-baseline benchmark.",
            _role_instruction(role_name),
            "You are solving the public parity task protocol used by this benchmark.",
            "Return exactly one JSON object and nothing else.",
            "Required JSON keys:",
            '- "next_register": object',
            '- "next_path": string',
            '- "write_mode": "keep" or "overwrite"',
            '- "write_content": string or null',
            '- "halt": boolean',
            '- "notes": string',
            "Rules:",
            "- next_register must be the full q' register, not a diff.",
            "- Do not omit any required key.",
            "- If write_mode is keep, set write_content to null.",
            "- Do not use markdown fences.",
            "- Do not explain your answer.",
            "- Do not simplify path strings.",
            "- notes must be one short plain-text line with no newlines, no quoted protocol excerpts, and no self-corrections.",
            "Public parity task protocol:",
            "- Files: parity.md stores the running parity bit, result.md stores the final odd/even answer, .ls files list directories and visible files.",
            "- .ls format: each line is either 'DIR name' or 'FILE name'.",
            "- Protected rule: never write to any .ls file.",
            "- phase=boot: keep the register fields unchanged except set phase to init_parity; move to parity.md with write_mode=keep.",
            "- phase=init_parity: when current_path is parity.md, overwrite the current file parity.md with '0', set phase to scan, set todo to [], and set next_path to root_listing (.ls). Never write to any .ls file.",
            "- phase=scan on a .ls file: parse listed dirs/files, convert each DIR name to a child path ending with '/.ls', convert each visible .md file to its file path, remove current_path from old todo if present, append current_path to completed if not already there, then compute new todo as remaining_old_todo_without_current_path plus all discovered .md files plus all discovered child .ls paths in listing order, deduping exact duplicates against completed and already-kept todo entries. Move to todo[0] if any item remains; otherwise set phase to finalize and move to parity.md.",
            "- phase=scan on a numeric .md file that is not parity.md or result.md: compute bit = int(current_content) & 1, remove that file from todo, append it to completed, set pending={'bit': bit, 'source': current_path}, set phase to apply_pending, move to parity.md with write_mode=keep.",
            "- phase=apply_pending: xor the current parity value in parity.md with pending.bit, clear pending, then if todo is non-empty set phase to scan and move to todo[0], else set phase to finalize and move to parity.md. This step overwrites parity.md with the new parity bit string.",
            "- phase=finalize: if current_path is not parity.md, move to parity.md with write_mode=keep. If current_path is parity.md, compute answer='odd' if int(current_content)!=0 else 'even', store it in register.answer, set phase to write_result, and move to result.md with write_mode=keep.",
            "- phase=write_result: if current_path is not result.md, move to result.md with write_mode=keep. If current_path is result.md, set phase to halt, overwrite result.md with register.answer, and set halt=true.",
            "- phase=halt: keep the current path and register, and keep halt=true.",
            "- Keep the existing register structure unless the public protocol above requires a change.",
            "Critical scan invariants:",
            "- When current_path ends with .ls, write_mode must be keep and write_content must be null.",
            "- When current_path ends with .ls, every DIR name must become a child path ending with '/.ls'. Never use a bare directory path.",
            "- For a .ls scan, compute todo exactly as: remaining_old_todo_without_current_path + discovered_md_files + discovered_child_ls_paths.",
            "- Within one current .ls discovery batch, discovered FILE paths must stay ahead of discovered child DIR/.ls paths even when the raw listing shows DIR lines before FILE lines.",
            "- Do not drop previously queued todo items that are still pending.",
            "- A discovered FILE name.md always creates the path <current_dir>/name.md, and that discovered file must stay in todo even when next_path remains an older pending item.",
            "- The FILE rule still applies when the current .ls listing has only FILE lines and no DIR lines: append those discovered file paths after the remaining old todo items.",
            "- Append current_path to completed before deduping.",
            "- The next_path after a .ls scan is todo[0] after that exact update.",
            "- During a .ls scan, move to parity.md only when the recomputed todo is empty. If recomputed todo is non-empty, next_path must be todo[0].",
            "Critical numeric-file invariants:",
            "- On phase=scan for a numeric .md file, write_mode must be keep and write_content must be null.",
            "- On phase=scan for a numeric .md file, compute bit from the current file only: odd -> 1, even -> 0. Use the last decimal digit only: 1/3/5/7/9 -> 1 and 0/2/4/6/8 -> 0.",
            "- On phase=scan for a numeric .md file, do not overwrite parity.md and do not perform apply_pending early. This step only records pending and moves to parity.md with write_mode=keep.",
            "Critical apply_pending invariants:",
            "- On phase=apply_pending at parity.md, overwrite parity.md with xor(current parity, pending.bit), clear pending, and do not append anything to completed.",
            "- On phase=apply_pending, keep todo unchanged. If todo is non-empty, set phase='scan' and next_path=todo[0]. If todo is empty, set phase='finalize' and next_path='parity.md'.",
            "- On phase=apply_pending, next_register must reflect the post-apply state directly. Do not leave phase='apply_pending' after applying the bit.",
            "Scan examples:",
            "- Example A: current_path='.ls', current todo=[], listing='DIR alpha\\nDIR beta' -> completed=['.ls'], todo=['alpha/.ls','beta/.ls'], next_path='alpha/.ls', write_mode='keep', write_content=null.",
            "- Example B: current_path='alpha/.ls', current todo=['alpha/.ls','beta/.ls'], listing='DIR nested\\nFILE x.md\\nFILE y.md' -> completed includes 'alpha/.ls', todo=['beta/.ls','alpha/x.md','alpha/y.md','alpha/nested/.ls'], next_path='beta/.ls', write_mode='keep', write_content=null.",
            "- Example C: current_path='a/b/.ls', current todo=['a/b/.ls','x.md'], listing='FILE y.md' -> completed includes 'a/b/.ls', todo=['x.md','a/b/y.md'], next_path='x.md', write_mode='keep', write_content=null.",
            "- Example D: current_path='.ls', current todo=[], listing='DIR alpha\\nFILE x.md' -> completed=['.ls'], todo=['x.md','alpha/.ls'], next_path='x.md', write_mode='keep', write_content=null.",
            "- Example E: current_path='parity.md', phase='apply_pending', current parity='1', pending={'bit': 0, 'source': 'a/x.md'}, todo=['a/.ls','b.md'] -> completed unchanged, pending=null, phase='scan', next_path='a/.ls', write_mode='overwrite', write_content='1'.",
            "- Example F: current_path='a/b/.ls', current todo=['a/b/.ls','older.md'], listing='FILE new1.md\\nFILE new2.md' -> completed includes 'a/b/.ls', todo=['older.md','a/b/new1.md','a/b/new2.md'], next_path='older.md', write_mode='keep', write_content=null.",
            "- Example G: current_path='a/n_odd.md', phase='scan', current_content='981245', todo=['a/n_odd.md','later.md'] -> completed includes 'a/n_odd.md', pending={'bit': 1, 'source': 'a/n_odd.md'}, phase='apply_pending', next_path='parity.md', write_mode='keep', write_content=null.",
            "- Example H: current_path='a/n_even.md', phase='scan', current_content='816774', todo=['a/n_even.md','later.md'] -> completed includes 'a/n_even.md', pending={'bit': 0, 'source': 'a/n_even.md'}, phase='apply_pending', next_path='parity.md', write_mode='keep', write_content=null.",
            "- Example I: current_path='a/b/.ls', current todo=['a/b/.ls'], listing='FILE z.md' -> completed includes 'a/b/.ls', todo=['a/b/z.md'], next_path='a/b/z.md', write_mode='keep', write_content=null. Because recomputed todo is non-empty, do not jump to parity.md.",
        ]
    )
    user_payload = {
        "prompt_version": PROMPT_VERSION,
        "role_name": role_name,
        "visible_state": _view_payload(view),
        "required_output_schema": {
            "next_register": {"type": "object"},
            "next_path": {"type": "string"},
            "write_mode": {"enum": ["keep", "overwrite"]},
            "write_content": {"type": ["string", "null"]},
            "halt": {"type": "boolean"},
            "notes": {"type": "string"},
        },
    }
    return [
        {"role": "system", "content": system_prompt},
        {"role": "user", "content": json.dumps(user_payload, ensure_ascii=True, indent=2, sort_keys=True)},
    ]
