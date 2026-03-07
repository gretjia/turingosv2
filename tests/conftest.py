from __future__ import annotations

import random
from pathlib import Path

import pytest

from turingos.agents.parity import DeterministicPolicyAgent, NoisyPolicyAgent
from turingos.models import AgentView
from turingos.runtime import TuringOSConfig, TuringOSKernel
from turingos.tasks.parity import ParityTask


@pytest.fixture()
def parity_kernel_factory(tmp_path: Path):
    def make(*, noisy_agents: int = 0, error_rate: float = 0.35, epsilon: float = 0.0, seed: int = 7):
        task = ParityTask()
        config = TuringOSConfig(max_steps=64, explore_epsilon=epsilon, random_seed=seed)
        kernel = TuringOSKernel(workspace_root=tmp_path / f"ws_{seed}_{noisy_agents}", task=task, agents=[], config=config)

        def policy(view: AgentView):
            return task.expected_transition(view.state, view.current_content, kernel.fs)

        agents = [DeterministicPolicyAgent(agent_id="alpha", policy=policy)]
        for i in range(noisy_agents):
            agents.append(
                NoisyPolicyAgent(
                    agent_id=f"noise-{i+1}",
                    policy=policy,
                    error_rate=error_rate,
                    rng=random.Random(seed + i + 1),
                )
            )
        kernel.agents = agents
        return kernel

    return make
