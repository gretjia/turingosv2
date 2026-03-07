from __future__ import annotations

import random

from turingos.masking import MaskingPolicy
from turingos.models import AgentProposal, EvaluatedProposal, MachineState, SignalBundle, Transition
from turingos.scheduler import PriceBoard, ProposalScheduler
from turingos.signals import PriceStats


def test_masking_policy_hides_raw_formula_and_peers() -> None:
    masking = MaskingPolicy(max_public_messages=2, max_private_messages=2)
    state = MachineState(step=1, current_path="x.md", register={"phase": "scan"})
    view = masking.build_view(
        agent_id="a",
        state=state,
        current_content="123",
        public_messages=["typical error 1", "typical error 2", "typical error 3"],
        private_messages=["own error 1", "own error 2", "own error 3"],
        price_stats=PriceStats(attempts=10, accepts=9, total_reward=8.0),
    )
    assert view.public_broadcasts == ["typical error 2", "typical error 3"]
    assert view.private_feedback == ["own error 2", "own error 3"]
    assert view.price_hint == "high"
    assert not hasattr(view, "peer_proposals")
    assert not hasattr(view, "scoring_formula")


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
