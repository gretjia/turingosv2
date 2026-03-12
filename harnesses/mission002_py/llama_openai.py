from __future__ import annotations

import json
import time
import urllib.error
import urllib.request
from dataclasses import dataclass, field
from typing import Any

from harnesses.mission002_py.mission002_prompting import PROMPT_VERSION, build_mission002_messages
from turingos.models import AgentProposal, AgentView, Transition


class ModelInfraError(RuntimeError):
    pass


@dataclass(slots=True)
class ProposalTrace:
    step: int
    current_path: str
    parsed: bool
    raw_response: str
    request_payload: dict[str, Any]
    parse_error: str | None = None
    infra_error: str | None = None
    transition: dict[str, Any] | None = None


def _provider_kind(base_url: str) -> str:
    normalized = _normalize_base_url(base_url)
    if "api.kimi.com/coding" in normalized:
        return "kimi_coding"
    return "openai"


def probe_openai_models(base_url: str, api_key: str, timeout_seconds: float) -> list[str]:
    provider_kind = _provider_kind(base_url)
    if provider_kind == "kimi_coding":
        payload = _request_json_with_headers(
            method="GET",
            url=f"{_normalize_base_url(base_url)}/v1/models",
            timeout_seconds=timeout_seconds,
            headers=_kimi_coding_headers(api_key),
        )
    else:
        payload = _request_json(
            method="GET",
            url=f"{_normalize_base_url(base_url)}/v1/models",
            api_key=api_key,
            timeout_seconds=timeout_seconds,
        )
    data = payload.get("data", [])
    if not isinstance(data, list):
        return []
    models: list[str] = []
    for item in data:
        if not isinstance(item, dict):
            continue
        model_id = item.get("id")
        if isinstance(model_id, str) and model_id:
            models.append(model_id)
    return models


@dataclass(slots=True)
class LlamaOpenAIAgent:
    agent_id: str
    base_url: str
    model: str
    role_name: str = "worker"
    api_key: str = "local"
    timeout_seconds: float = 60.0
    max_retries: int = 1
    max_tokens: int = 512
    temperature: float = 0.0
    history: list[ProposalTrace] = field(default_factory=list)

    def propose(self, view: AgentView) -> AgentProposal:
        provider_kind = _provider_kind(self.base_url)
        messages = build_mission002_messages(view, role_name=self.role_name)
        request_payload = _request_payload_for_provider(
            provider_kind=provider_kind,
            model=self.model,
            messages=messages,
            temperature=self.temperature,
            max_tokens=self.max_tokens,
        )
        raw_response = ""
        infra_error: str | None = None
        for attempt in range(self.max_retries + 1):
            try:
                response = _request_completion(
                    provider_kind=provider_kind,
                    base_url=self.base_url,
                    api_key=self.api_key,
                    timeout_seconds=self.timeout_seconds,
                    payload=request_payload,
                )
                raw_response = _extract_content(response, provider_kind=provider_kind)
                transition = _parse_transition_text(raw_response)
                self.history.append(
                    ProposalTrace(
                        step=view.state.step,
                        current_path=view.state.current_path,
                        parsed=True,
                        raw_response=raw_response,
                        request_payload=_trace_request_payload(request_payload),
                        transition=_transition_payload(transition),
                    )
                )
                return AgentProposal(agent_id=self.agent_id, transition=transition, raw_response=raw_response)
            except ValueError as exc:
                parse_error = str(exc)
                transition = _invalid_transition(view, parse_error)
                self.history.append(
                    ProposalTrace(
                        step=view.state.step,
                        current_path=view.state.current_path,
                        parsed=False,
                        raw_response=raw_response,
                        request_payload=_trace_request_payload(request_payload),
                        parse_error=parse_error,
                        transition=_transition_payload(transition),
                    )
                )
                return AgentProposal(agent_id=self.agent_id, transition=transition, raw_response=raw_response)
            except ModelInfraError as exc:
                infra_error = str(exc)
                if attempt >= self.max_retries:
                    self.history.append(
                        ProposalTrace(
                            step=view.state.step,
                            current_path=view.state.current_path,
                            parsed=False,
                            raw_response=raw_response,
                            request_payload=_trace_request_payload(request_payload),
                            infra_error=infra_error,
                        )
                    )
                    raise
                time.sleep(min(1.0, 0.25 * (attempt + 1)))
        raise ModelInfraError(infra_error or "model call failed without a captured reason")

    def history_payload(self) -> list[dict[str, Any]]:
        return [
            {
                "step": item.step,
                "current_path": item.current_path,
                "parsed": item.parsed,
                "raw_response": item.raw_response,
                "request_payload": item.request_payload,
                "parse_error": item.parse_error,
                "infra_error": item.infra_error,
                "transition": item.transition,
            }
            for item in self.history
        ]


def _normalize_base_url(base_url: str) -> str:
    return base_url.rstrip("/")


def _request_payload_for_provider(
    *,
    provider_kind: str,
    model: str,
    messages: list[dict[str, Any]],
    temperature: float,
    max_tokens: int,
) -> dict[str, Any]:
    if provider_kind == "kimi_coding":
        return {
            "model": model,
            "messages": messages,
            "temperature": temperature,
            "max_tokens": max_tokens,
        }
    return {
        "model": model,
        "messages": messages,
        "temperature": temperature,
        "max_tokens": max_tokens,
    }


def _request_completion(
    *,
    provider_kind: str,
    base_url: str,
    api_key: str,
    timeout_seconds: float,
    payload: dict[str, Any],
) -> dict[str, Any]:
    if provider_kind == "kimi_coding":
        return _request_json_with_headers(
            method="POST",
            url=f"{_normalize_base_url(base_url)}/v1/messages",
            timeout_seconds=timeout_seconds,
            headers=_kimi_coding_headers(api_key),
            payload=payload,
        )
    return _request_json(
        method="POST",
        url=f"{_normalize_base_url(base_url)}/v1/chat/completions",
        api_key=api_key,
        timeout_seconds=timeout_seconds,
        payload=payload,
    )


def _kimi_coding_headers(api_key: str) -> dict[str, str]:
    headers = {
        "content-type": "application/json",
        "anthropic-version": "2023-06-01",
    }
    if api_key:
        headers["x-api-key"] = api_key
    return headers


def _request_json(
    *,
    method: str,
    url: str,
    api_key: str,
    timeout_seconds: float,
    payload: dict[str, Any] | None = None,
) -> dict[str, Any]:
    headers = {"content-type": "application/json"}
    if api_key:
        headers["authorization"] = f"Bearer {api_key}"
    data = None if payload is None else json.dumps(payload).encode("utf-8")
    request = urllib.request.Request(url=url, data=data, headers=headers, method=method)
    try:
        with urllib.request.urlopen(request, timeout=timeout_seconds) as response:
            body = response.read().decode("utf-8")
    except urllib.error.HTTPError as exc:
        detail = exc.read().decode("utf-8", errors="replace")
        raise ModelInfraError(f"HTTP {exc.code} from {url}: {detail}") from exc
    except (urllib.error.URLError, TimeoutError, OSError) as exc:
        raise ModelInfraError(f"request to {url} failed: {exc}") from exc
    try:
        decoded = json.loads(body)
    except json.JSONDecodeError as exc:
        raise ModelInfraError(f"endpoint returned invalid JSON from {url}: {exc}") from exc
    if not isinstance(decoded, dict):
        raise ModelInfraError(f"endpoint returned non-object JSON from {url}")
    return decoded


def _request_json_with_headers(
    *,
    method: str,
    url: str,
    timeout_seconds: float,
    headers: dict[str, str],
    payload: dict[str, Any] | None = None,
) -> dict[str, Any]:
    data = None if payload is None else json.dumps(payload).encode("utf-8")
    request = urllib.request.Request(url=url, data=data, headers=headers, method=method)
    try:
        with urllib.request.urlopen(request, timeout=timeout_seconds) as response:
            body = response.read().decode("utf-8")
    except urllib.error.HTTPError as exc:
        detail = exc.read().decode("utf-8", errors="replace")
        raise ModelInfraError(f"HTTP {exc.code} from {url}: {detail}") from exc
    except (urllib.error.URLError, TimeoutError, OSError) as exc:
        raise ModelInfraError(f"request to {url} failed: {exc}") from exc
    try:
        decoded = json.loads(body)
    except json.JSONDecodeError as exc:
        raise ModelInfraError(f"endpoint returned invalid JSON from {url}: {exc}") from exc
    if not isinstance(decoded, dict):
        raise ModelInfraError(f"endpoint returned non-object JSON from {url}")
    return decoded


def _extract_content(payload: dict[str, Any], *, provider_kind: str = "openai") -> str:
    if provider_kind == "kimi_coding":
        content = payload.get("content")
        if not isinstance(content, list):
            raise ModelInfraError("kimi coding response did not include content blocks")
        text_parts: list[str] = []
        for item in content:
            if not isinstance(item, dict):
                continue
            text = item.get("text")
            if isinstance(text, str):
                text_parts.append(text)
        joined = "".join(text_parts)
        if joined:
            return joined
        raise ModelInfraError("kimi coding response did not include text content")
    choices = payload.get("choices")
    if not isinstance(choices, list) or not choices:
        raise ModelInfraError("chat completion response did not include choices")
    first = choices[0]
    if not isinstance(first, dict):
        raise ModelInfraError("chat completion choice was not an object")
    message = first.get("message")
    if not isinstance(message, dict):
        raise ModelInfraError("chat completion choice did not include a message object")
    content = message.get("content", "")
    if isinstance(content, str):
        return content
    if isinstance(content, list):
        text_parts: list[str] = []
        for item in content:
            if not isinstance(item, dict):
                continue
            text = item.get("text")
            if isinstance(text, str):
                text_parts.append(text)
        return "".join(text_parts)
    raise ModelInfraError("chat completion message content was not text")


def _strip_one_code_fence(raw: str) -> str:
    text = raw.strip()
    if not text.startswith("```") or not text.endswith("```"):
        return text
    lines = text.splitlines()
    if len(lines) < 3:
        return text
    if not lines[0].startswith("```") or lines[-1] != "```":
        return text
    return "\n".join(lines[1:-1]).strip()


def _parse_transition_text(raw: str) -> Transition:
    normalized = _strip_one_code_fence(raw)
    payload = json.loads(normalized)
    if not isinstance(payload, dict):
        raise ValueError("proposal payload must be a JSON object")

    required = {"next_register", "next_path", "write_mode", "write_content", "halt", "notes"}
    missing = sorted(required - set(payload))
    if missing:
        raise ValueError(f"proposal missing required keys: {', '.join(missing)}")

    next_register = payload["next_register"]
    next_path = payload["next_path"]
    write_mode = payload["write_mode"]
    write_content = payload["write_content"]
    halt = payload["halt"]
    notes = payload["notes"]

    if not isinstance(next_register, dict):
        raise ValueError("next_register must be an object")
    if not isinstance(next_path, str) or not next_path:
        raise ValueError("next_path must be a non-empty string")
    if write_mode not in {"keep", "overwrite"}:
        raise ValueError("write_mode must be 'keep' or 'overwrite'")
    if write_content is not None and not isinstance(write_content, str):
        raise ValueError("write_content must be a string or null")
    if not isinstance(halt, bool):
        raise ValueError("halt must be a boolean")
    if not isinstance(notes, str):
        raise ValueError("notes must be a string")

    return Transition(
        next_register=next_register,
        next_path=next_path,
        write_mode=write_mode,
        write_content=write_content,
        halt=halt,
        notes=notes,
    )


def _invalid_transition(view: AgentView, reason: str) -> Transition:
    return Transition(
        next_register=view.state.register,
        next_path="../invalid_transition",
        write_mode="keep",
        write_content=None,
        halt=False,
        notes=f"{PROMPT_VERSION}: {reason}"[:240],
    )


def _transition_payload(transition: Transition) -> dict[str, Any]:
    return {
        "next_register": transition.next_register,
        "next_path": transition.next_path,
        "write_mode": transition.write_mode,
        "write_content": transition.write_content,
        "halt": transition.halt,
        "notes": transition.notes,
    }


def _trace_request_payload(payload: dict[str, Any]) -> dict[str, Any]:
    return {
        "model": payload["model"],
        "temperature": payload["temperature"],
        "max_tokens": payload["max_tokens"],
        "messages": payload["messages"],
    }
