from __future__ import annotations

from harnesses.mission002_py import llama_openai as llama_openai_module
from harnesses.mission002_py.llama_openai import LlamaOpenAIAgent
from turingos.models import AgentView, FrozenMachineState


def _make_view() -> AgentView:
    return AgentView(
        agent_id="planner27",
        state=FrozenMachineState(
            step=2,
            current_path=".ls",
            register={"phase": "scan", "todo": ["parity.md"]},
            halted=False,
        ),
        current_content="DIR 1\nFILE 100.md",
    )


def test_llama_openai_agent_parses_code_fenced_json(monkeypatch) -> None:
    agent = LlamaOpenAIAgent(
        agent_id="planner27",
        base_url="http://127.0.0.1:18080",
        model="qwen27",
        role_name="planner",
    )

    def fake_request_json(**_: object) -> dict:
        return {
            "choices": [
                {
                    "message": {
                        "content": """```json
{"next_register":{"phase":"scan","todo":["parity.md"]},"next_path":"parity.md","write_mode":"keep","write_content":null,"halt":false,"notes":"exact"}
```"""
                    }
                }
            ]
        }

    monkeypatch.setattr(llama_openai_module, "_request_json", fake_request_json)
    proposal = agent.propose(_make_view())

    assert proposal.transition.next_path == "parity.md"
    assert proposal.transition.notes == "exact"
    assert agent.history[-1].parsed is True


def test_llama_openai_agent_turns_schema_failure_into_invalid_transition(monkeypatch) -> None:
    agent = LlamaOpenAIAgent(
        agent_id="worker9",
        base_url="http://127.0.0.1:18081",
        model="qwen9",
        role_name="worker",
    )

    def fake_request_json(**_: object) -> dict:
        return {"choices": [{"message": {"content": '{"next_path":"parity.md"}'}}]}

    monkeypatch.setattr(llama_openai_module, "_request_json", fake_request_json)
    proposal = agent.propose(_make_view())

    assert proposal.transition.next_path == "../invalid_transition"
    assert agent.history[-1].parsed is False
    assert "missing required keys" in (agent.history[-1].parse_error or "")


def test_kimi_coding_agent_parses_message_blocks(monkeypatch) -> None:
    agent = LlamaOpenAIAgent(
        agent_id="kimi",
        base_url="https://api.kimi.com/coding",
        model="kimi-for-coding",
        api_key="kimi-key",
        role_name="planner",
    )

    def fake_request_json_with_headers(**_: object) -> dict:
        return {
            "content": [
                {
                    "type": "text",
                    "text": """```json
{"next_register":{"phase":"scan","todo":["parity.md"]},"next_path":"parity.md","write_mode":"keep","write_content":null,"halt":false,"notes":"kimi coding ok"}
```""",
                }
            ]
        }

    monkeypatch.setattr(llama_openai_module, "_request_json_with_headers", fake_request_json_with_headers)
    proposal = agent.propose(_make_view())

    assert proposal.transition.next_path == "parity.md"
    assert proposal.transition.notes == "kimi coding ok"
    assert agent.history[-1].parsed is True
