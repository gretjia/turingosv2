from __future__ import annotations

import argparse
import json
import random
import shutil
from pathlib import Path

from .agents.parity import DeterministicPolicyAgent, NoisyPolicyAgent
from .models import AgentView
from .runtime import TuringOSConfig, TuringOSKernel
from .tasks.parity import ParityTask


def _parity_policy(task: ParityTask, kernel_fs):
    def policy(view: AgentView):
        return task.expected_transition(view.state, view.current_content, kernel_fs)

    return policy


def run_parity_demo(args: argparse.Namespace) -> int:
    workspace = Path(args.workspace)
    if workspace.exists() and args.clean:
        shutil.rmtree(workspace)
    task = ParityTask()
    config = TuringOSConfig(max_steps=args.max_steps, explore_epsilon=args.epsilon, random_seed=args.seed)
    # Initialize kernel first so the workspace exists for the policy closure.
    kernel = TuringOSKernel(workspace_root=workspace, task=task, agents=[], config=config)
    policy = _parity_policy(task, kernel.fs)
    agents = [DeterministicPolicyAgent(agent_id="alpha", policy=policy)]
    for i in range(args.noisy_agents):
        agents.append(
            NoisyPolicyAgent(
                agent_id=f"noise-{i+1}",
                policy=policy,
                error_rate=args.error_rate,
                rng=random.Random(args.seed + i + 1),
            )
        )
    kernel.agents = agents
    result = kernel.run()
    payload = {
        "success": result.success,
        "steps": result.steps_executed,
        "selected_agents": result.selected_agents,
        "diagnostics": result.diagnostics,
    }
    print(json.dumps(payload, ensure_ascii=False, indent=2))
    return 0 if result.success else 1


def _register_optional_mission002_cli(subparsers: argparse._SubParsersAction[argparse.ArgumentParser]) -> None:
    try:
        from harnesses.mission002_py.mission002 import register_cli as register_mission002_cli
    except ModuleNotFoundError:
        return
    register_mission002_cli(subparsers)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="TuringOS demo CLI")
    sub = parser.add_subparsers(dest="cmd", required=True)

    parity = sub.add_parser("parity-demo", help="run the article parity task")
    parity.add_argument("--workspace", default="./examples/parity_workspace")
    parity.add_argument("--max-steps", type=int, default=64)
    parity.add_argument("--epsilon", type=float, default=0.0)
    parity.add_argument("--seed", type=int, default=7)
    parity.add_argument("--noisy-agents", type=int, default=3)
    parity.add_argument("--error-rate", type=float, default=0.35)
    parity.add_argument("--clean", action="store_true")
    parity.set_defaults(func=run_parity_demo)

    _register_optional_mission002_cli(sub)

    return parser


def main() -> int:
    parser = build_parser()
    args = parser.parse_args()
    return args.func(args)


if __name__ == "__main__":  # pragma: no cover
    raise SystemExit(main())
