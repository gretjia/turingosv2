from __future__ import annotations

import random
import pytest

from turingos.masking import MaskingPolicy
from turingos.models import AgentProposal, EvaluatedProposal, MachineState, SignalBundle, Transition
from turingos.runtime import TuringOSConfig
from turingos.scheduler import PriceBoard, ProposalScheduler


def test_masking_policy_projects_constitutional_input_only() -> None:
    masking = MaskingPolicy()
    state = MachineState(step=1, current_path="x.md", register={"phase": "scan"})
    view = masking.build_view(
        agent_id="a",
        state=state,
        current_content="123",
    )
    assert view.agent_id == "a"
    assert view.current_content == "123"
    state.register["phase"] = "mutated"
    assert view.state.register["phase"] == "scan"


def test_runtime_config_rejects_invalid_proposal_bypass() -> None:
    with pytest.raises(ValueError, match="abort_on_no_valid_proposal must remain True"):
        TuringOSConfig(abort_on_no_valid_proposal=False)


def test_scheduler_prefers_higher_utility_with_price_prior() -> None:
    board = PriceBoard()
    board.update("low", accepted=True, reward=0.1)
    for _ in range(5):
        board.update("high", accepted=True, reward=2.0)

    low = EvaluatedProposal(
        proposal=AgentProposal("low", Transition(next_register={}, next_path="a")),
        signals=SignalBundle(hard_pass=True, score=1.0),
    )
    high = EvaluatedProposal(
        proposal=AgentProposal("high", Transition(next_register={}, next_path="a")),
        signals=SignalBundle(hard_pass=True, score=1.0),
    )
    scheduler = ProposalScheduler(epsilon=0.0, rng=random.Random(0))
    chosen = scheduler.select([low, high], board)
    assert chosen.agent_id == "high"
