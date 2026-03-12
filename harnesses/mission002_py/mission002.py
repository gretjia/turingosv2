from __future__ import annotations

import argparse
import json
import shutil
import sys
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Callable

from harnesses.mission002_py.artifacts import collect_workspace_snapshot, read_tail_lines, write_json, write_text
from harnesses.mission002_py.llama_openai import LlamaOpenAIAgent, ModelInfraError, probe_openai_models
from harnesses.mission002_py.mission002_prompting import PROMPT_VERSION
from harnesses.mission002_py.parity_cases import DEFAULT_GENERATOR_VERSION, ParityCase, generate_parity_case
from turingos.agents.parity import DeterministicPolicyAgent
from turingos.models import AgentView, MachineState, RunResult
from turingos.runtime import TuringOSConfig, TuringOSKernel
from turingos.tasks.parity import ParityTask


ModeId = str
AgentFactory = Callable[[ModeId, ParityTask, TuringOSKernel], list[object]]


MODE_SINGLE_27B = "turingosv2_single_27b"
MODE_SINGLE_9B = "turingosv2_single_9b"
MODE_DUAL = "turingosv2_planner27_worker9"
ALL_MODES = (MODE_SINGLE_27B, MODE_SINGLE_9B, MODE_DUAL)


@dataclass(slots=True)
class EndpointSpec:
    base_url: str
    model: str
    api_key: str = "local"
    timeout_seconds: float = 60.0
    max_retries: int = 1
    max_tokens: int = 512

    def manifest(self) -> dict[str, Any]:
        return {
            "base_url": self.base_url,
            "model": self.model,
            "timeout_seconds": self.timeout_seconds,
            "max_retries": self.max_retries,
            "max_tokens": self.max_tokens,
        }


@dataclass(slots=True)
class Mission002Config:
    modes: list[ModeId]
    start_case: int
    max_cases: int
    global_seed: int
    generator_version: str
    max_steps: int
    output_root: Path
    run_name: str
    clean: bool
    qwen27: EndpointSpec
    qwen9: EndpointSpec
    topology_host_alias: str = ""
    windows1_participated: bool = False
    qwen27_remote_target: str = ""
    qwen9_remote_target: str = ""
    qwen27_launch_flags: str = ""
    qwen9_launch_flags: str = ""
    topology_notes: str = ""
    repo_revision: str = ""


def register_cli(subparsers: argparse._SubParsersAction[argparse.ArgumentParser]) -> None:
    parser = subparsers.add_parser("benchmark-mission002", help="run the Mission 002 pure-baseline benchmark")
    parser.add_argument("--modes", default=",".join(ALL_MODES))
    parser.add_argument("--start-case", type=int, default=1)
    parser.add_argument("--max-cases", type=int, default=1)
    parser.add_argument("--global-seed", type=int, default=20260308)
    parser.add_argument("--generator-version", default=DEFAULT_GENERATOR_VERSION)
    parser.add_argument("--max-steps", type=int, default=64)
    parser.add_argument("--output-root", default="./benchmarks/mission002")
    parser.add_argument("--run-name", default="")
    parser.add_argument("--clean", action="store_true")

    parser.add_argument("--qwen27-base-url", default="http://127.0.0.1:18080")
    parser.add_argument("--qwen27-model", default="qwen3.5-27b-instruct-q4_k_m.gguf")
    parser.add_argument("--qwen27-api-key", default="local")

    parser.add_argument("--qwen9-base-url", default="http://127.0.0.1:18081")
    parser.add_argument("--qwen9-model", default="Qwen3.5-9B-Q4_K_M.gguf")
    parser.add_argument("--qwen9-api-key", default="local")

    parser.add_argument("--timeout-seconds", type=float, default=60.0)
    parser.add_argument("--retry-budget", type=int, default=1)
    parser.add_argument("--max-tokens", type=int, default=512)
    parser.add_argument("--topology-host-alias", default="")
    parser.add_argument("--windows1-participated", action="store_true")
    parser.add_argument("--qwen27-remote-target", default="")
    parser.add_argument("--qwen9-remote-target", default="")
    parser.add_argument("--qwen27-launch-flags", default="")
    parser.add_argument("--qwen9-launch-flags", default="")
    parser.add_argument("--topology-notes", default="")
    parser.add_argument("--repo-revision", default="")
    parser.set_defaults(func=run_from_cli)


def run_from_cli(args: argparse.Namespace) -> int:
    config = Mission002Config(
        modes=_parse_modes(args.modes),
        start_case=max(1, args.start_case),
        max_cases=max(1, args.max_cases),
        global_seed=args.global_seed,
        generator_version=args.generator_version,
        max_steps=max(1, args.max_steps),
        output_root=Path(args.output_root),
        run_name=args.run_name or _default_run_name(),
        clean=args.clean,
        qwen27=EndpointSpec(
            base_url=args.qwen27_base_url,
            model=args.qwen27_model,
            api_key=args.qwen27_api_key,
            timeout_seconds=args.timeout_seconds,
            max_retries=max(0, args.retry_budget),
            max_tokens=max(64, args.max_tokens),
        ),
        qwen9=EndpointSpec(
            base_url=args.qwen9_base_url,
            model=args.qwen9_model,
            api_key=args.qwen9_api_key,
            timeout_seconds=args.timeout_seconds,
            max_retries=max(0, args.retry_budget),
            max_tokens=max(64, args.max_tokens),
        ),
        topology_host_alias=args.topology_host_alias,
        windows1_participated=bool(args.windows1_participated),
        qwen27_remote_target=args.qwen27_remote_target,
        qwen9_remote_target=args.qwen9_remote_target,
        qwen27_launch_flags=args.qwen27_launch_flags,
        qwen9_launch_flags=args.qwen9_launch_flags,
        topology_notes=args.topology_notes,
        repo_revision=args.repo_revision,
    )
    summary = run_benchmark(config)
    print(json.dumps(summary, ensure_ascii=True, indent=2, sort_keys=True))
    return 0 if all(item["status"] == "PASS" for item in summary["mode_summaries"]) else 1


def run_benchmark(config: Mission002Config, *, agent_factory: AgentFactory | None = None) -> dict[str, Any]:
    run_dir = config.output_root / config.run_name
    if run_dir.exists() and config.clean:
        shutil.rmtree(run_dir)
    run_dir.mkdir(parents=True, exist_ok=True)

    endpoint_manifests = {
        "qwen27": config.qwen27.manifest(),
        "qwen9": config.qwen9.manifest(),
    }
    preflight = _preflight_models(config) if agent_factory is None else {}
    manifest = {
        "mission": "Mission 002",
        "run_name": config.run_name,
        "modes": config.modes,
        "start_case": config.start_case,
        "max_cases": config.max_cases,
        "global_seed": config.global_seed,
        "generator_version": config.generator_version,
        "max_steps": config.max_steps,
        "argv": sys.argv,
        "prompt_version": PROMPT_VERSION,
        "repo_revision": config.repo_revision,
        "endpoint_manifest": endpoint_manifests,
        "preflight_models": preflight,
        "topology_manifest": {
            "host_alias": config.topology_host_alias,
            "windows1_participated": config.windows1_participated,
            "qwen27_remote_target": config.qwen27_remote_target,
            "qwen9_remote_target": config.qwen9_remote_target,
            "qwen27_launch_flags": config.qwen27_launch_flags,
            "qwen9_launch_flags": config.qwen9_launch_flags,
            "notes": config.topology_notes,
        },
    }
    write_json(run_dir / "run_manifest.json", manifest)

    mode_summaries: list[dict[str, Any]] = []
    for mode in config.modes:
        summary = _run_mode(config=config, mode=mode, run_dir=run_dir, agent_factory=agent_factory)
        mode_summaries.append(summary)

    overall = {
        "mission": "Mission 002",
        "run_name": config.run_name,
        "output_dir": str(run_dir),
        "mode_summaries": mode_summaries,
    }
    write_json(run_dir / "overall_summary.json", overall)
    write_text(run_dir / "overall_summary.md", _overall_summary_markdown(overall))
    return overall


def _run_mode(
    *,
    config: Mission002Config,
    mode: ModeId,
    run_dir: Path,
    agent_factory: AgentFactory | None,
) -> dict[str, Any]:
    mode_dir = run_dir / mode
    mode_dir.mkdir(parents=True, exist_ok=True)
    passed = 0
    attempted = 0
    first_non_pass_case: int | None = None
    first_non_pass_artifact: str | None = None
    status = "PASS"
    reason = ""

    for offset in range(config.max_cases):
        case_index = config.start_case + offset
        case = generate_parity_case(
            global_seed=config.global_seed,
            case_index=case_index,
            generator_version=config.generator_version,
        )
        attempted += 1
        case_dir = mode_dir / f"case_{case_index:06d}"
        workspace_dir = case_dir / "workspace"
        result = _run_case(
            config=config,
            mode=mode,
            case=case,
            case_dir=case_dir,
            workspace_dir=workspace_dir,
            agent_factory=agent_factory,
        )
        case_status = result["status"]
        if case_status == "PASS":
            passed += 1
            continue
        status = case_status
        reason = result.get("reason", "")
        first_non_pass_case = case_index
        first_non_pass_artifact = str(case_dir / "case_result.json")
        break

    summary = {
        "mode": mode,
        "attempted": attempted,
        "passed": passed,
        "failed": 0 if status == "PASS" else 1,
        "first_non_pass_case": first_non_pass_case,
        "consecutive_pass_before_first_fail": passed,
        "status": status,
        "reason": reason,
        "first_fail_artifact": first_non_pass_artifact,
    }
    write_json(mode_dir / "summary.json", summary)
    write_text(mode_dir / "summary.md", _mode_summary_markdown(summary))
    return summary


def _run_case(
    *,
    config: Mission002Config,
    mode: ModeId,
    case: ParityCase,
    case_dir: Path,
    workspace_dir: Path,
    agent_factory: AgentFactory | None,
) -> dict[str, Any]:
    if case_dir.exists():
        shutil.rmtree(case_dir)
    case_dir.mkdir(parents=True, exist_ok=True)

    task = ParityTask(tree=case.tree)
    kernel = TuringOSKernel(
        workspace_root=workspace_dir,
        task=task,
        agents=[],
        config=TuringOSConfig(
            max_steps=config.max_steps,
            explore_epsilon=0.0,
            random_seed=case.case_seed,
        ),
    )
    agents = (
        agent_factory(mode, task, kernel)
        if agent_factory is not None
        else _build_llm_agents(mode=mode, config=config)
    )
    kernel.agents = agents

    result: RunResult | None = None
    status = "PASS"
    reason = ""
    error_type = ""
    try:
        result = kernel.run()
        if not result.success:
            status = "FAIL"
            error_type = "semantic_failure"
            reason = "kernel run completed without task success"
    except ModelInfraError as exc:
        status = "BLOCKED_BY_INFRA"
        error_type = "infra_failure"
        reason = str(exc)
    except RuntimeError as exc:
        status = "FAIL"
        error_type = "semantic_failure"
        reason = str(exc)

    payload = _case_payload(
        case=case,
        mode=mode,
        kernel=kernel,
        result=result,
        status=status,
        reason=reason,
        error_type=error_type,
    )
    if status != "PASS":
        ledger_path = workspace_dir / kernel.config.ledger_filename
        if ledger_path.exists():
            write_text(case_dir / "ledger_full.jsonl", ledger_path.read_text(encoding="utf-8"))
    write_json(case_dir / "case_result.json", payload)
    return payload


def _case_payload(
    *,
    case: ParityCase,
    mode: ModeId,
    kernel: TuringOSKernel,
    result: RunResult | None,
    status: str,
    reason: str,
    error_type: str,
) -> dict[str, Any]:
    assert kernel.fs is not None
    assert kernel.state is not None
    workspace_root = Path(kernel.workspace_root)
    diagnostics = {
        "task": kernel.task.diagnostics(kernel.fs),
        "ledger_ok": kernel.ledger.verify_integrity() if kernel.ledger else False,
        "price_ranking": kernel.board.ranking(),
        "public_broadcasts": kernel.broadcasts.maybe_publish_typical_errors(),
    }
    payload = {
        "mode": mode,
        "status": status,
        "reason": reason,
        "error_type": error_type,
        "case": case.manifest(),
        "runtime_config": {
            "max_steps": kernel.config.max_steps,
            "explore_epsilon": kernel.config.explore_epsilon,
            "random_seed": kernel.config.random_seed,
        },
        "state": _state_payload(kernel.state),
        "result": None
        if result is None
        else {
            "success": result.success,
            "steps_executed": result.steps_executed,
            "selected_agents": result.selected_agents,
            "final_files": result.final_files,
            "diagnostics": result.diagnostics,
        },
        "selected_agents": list(kernel.selected_agents),
        "task_diagnostics": diagnostics,
        "workspace_snapshot": collect_workspace_snapshot(workspace_root),
        "ledger_tail": read_tail_lines(workspace_root / kernel.config.ledger_filename),
        "agent_traces": _agent_traces(kernel.agents),
    }
    return payload


def _build_llm_agents(*, mode: ModeId, config: Mission002Config) -> list[object]:
    if mode == MODE_SINGLE_27B:
        return [
            LlamaOpenAIAgent(
                agent_id="qwen27",
                base_url=config.qwen27.base_url,
                model=config.qwen27.model,
                api_key=config.qwen27.api_key,
                timeout_seconds=config.qwen27.timeout_seconds,
                max_retries=config.qwen27.max_retries,
                max_tokens=config.qwen27.max_tokens,
                role_name="worker",
            )
        ]
    if mode == MODE_SINGLE_9B:
        return [
            LlamaOpenAIAgent(
                agent_id="qwen9",
                base_url=config.qwen9.base_url,
                model=config.qwen9.model,
                api_key=config.qwen9.api_key,
                timeout_seconds=config.qwen9.timeout_seconds,
                max_retries=config.qwen9.max_retries,
                max_tokens=config.qwen9.max_tokens,
                role_name="worker",
            )
        ]
    if mode == MODE_DUAL:
        return [
            LlamaOpenAIAgent(
                agent_id="planner27",
                base_url=config.qwen27.base_url,
                model=config.qwen27.model,
                api_key=config.qwen27.api_key,
                timeout_seconds=config.qwen27.timeout_seconds,
                max_retries=config.qwen27.max_retries,
                max_tokens=config.qwen27.max_tokens,
                role_name="planner",
            ),
            LlamaOpenAIAgent(
                agent_id="worker9",
                base_url=config.qwen9.base_url,
                model=config.qwen9.model,
                api_key=config.qwen9.api_key,
                timeout_seconds=config.qwen9.timeout_seconds,
                max_retries=config.qwen9.max_retries,
                max_tokens=config.qwen9.max_tokens,
                role_name="worker",
            ),
        ]
    raise ValueError(f"unsupported Mission 002 mode: {mode}")


def _preflight_models(config: Mission002Config) -> dict[str, Any]:
    probes: dict[str, Any] = {}
    seen: dict[tuple[str, str], dict[str, Any]] = {}
    for label, spec in (("qwen27", config.qwen27), ("qwen9", config.qwen9)):
        key = (spec.base_url, spec.model)
        if key not in seen:
            seen[key] = {
                "base_url": spec.base_url,
                "declared_model": spec.model,
                "available_models": probe_openai_models(spec.base_url, spec.api_key, spec.timeout_seconds),
            }
        probes[label] = seen[key]
    return probes


def _agent_traces(agents: list[object]) -> dict[str, Any]:
    traces: dict[str, Any] = {}
    for agent in agents:
        agent_id = getattr(agent, "agent_id", agent.__class__.__name__)
        if hasattr(agent, "history_payload"):
            traces[agent_id] = agent.history_payload()
        else:
            traces[agent_id] = []
    return traces


def _state_payload(state: MachineState) -> dict[str, Any]:
    return {
        "step": state.step,
        "current_path": state.current_path,
        "register": state.register,
        "halted": state.halted,
    }


def _parse_modes(raw: str) -> list[ModeId]:
    parsed = [item.strip() for item in raw.split(",") if item.strip()]
    if not parsed:
        return [MODE_SINGLE_27B]
    invalid = [item for item in parsed if item not in ALL_MODES]
    if invalid:
        raise ValueError(f"unsupported Mission 002 modes: {', '.join(invalid)}")
    return parsed


def _default_run_name() -> str:
    stamp = datetime.now(timezone.utc).strftime("%Y%m%d_%H%M%SUTC")
    return f"mission002_{stamp}"


def _mode_summary_markdown(summary: dict[str, Any]) -> str:
    lines = [
        f"# {summary['mode']}",
        "",
        f"- Status: {summary['status']}",
        f"- Attempted: {summary['attempted']}",
        f"- Passed: {summary['passed']}",
        f"- Consecutive pass before first fail: {summary['consecutive_pass_before_first_fail']}",
    ]
    if summary.get("first_non_pass_case") is not None:
        lines.append(f"- First non-pass case: {summary['first_non_pass_case']}")
    if summary.get("reason"):
        lines.append(f"- Reason: {summary['reason']}")
    if summary.get("first_fail_artifact"):
        lines.append(f"- First fail artifact: {summary['first_fail_artifact']}")
    return "\n".join(lines) + "\n"


def _overall_summary_markdown(overall: dict[str, Any]) -> str:
    lines = [
        "# Mission 002 Overall Summary",
        "",
        f"- Run name: {overall['run_name']}",
        f"- Output dir: {overall['output_dir']}",
        "",
        "## Modes",
        "",
    ]
    for item in overall["mode_summaries"]:
        lines.append(
            f"- {item['mode']}: status={item['status']}, attempted={item['attempted']}, passed={item['passed']}, consecutive={item['consecutive_pass_before_first_fail']}"
        )
    return "\n".join(lines) + "\n"


def deterministic_agent_factory(mode: ModeId, task: ParityTask, kernel: TuringOSKernel) -> list[object]:
    def policy(view: AgentView):
        return task.expected_transition(view.state, view.current_content, kernel.fs)

    if mode == MODE_DUAL:
        return [
            DeterministicPolicyAgent(agent_id="planner27", policy=policy),
            DeterministicPolicyAgent(agent_id="worker9", policy=policy),
        ]
    agent_id = "alpha27" if mode == MODE_SINGLE_27B else "alpha9"
    return [DeterministicPolicyAgent(agent_id=agent_id, policy=policy)]
