from __future__ import annotations

import random
from dataclasses import dataclass, field

from .models import EvaluatedProposal
from .signals import PriceStats


@dataclass(slots=True)
class PriceBoard:
    stats: dict[str, PriceStats] = field(default_factory=dict)

    def get(self, agent_id: str) -> PriceStats:
        if agent_id not in self.stats:
            self.stats[agent_id] = PriceStats()
        return self.stats[agent_id]

    def update(self, agent_id: str, *, accepted: bool, reward: float) -> None:
        s = self.get(agent_id)
        s.attempts += 1
        if accepted:
            s.accepts += 1
        s.total_reward += reward
        s.last_reward = reward

    def ranking(self) -> list[tuple[str, float]]:
        return sorted(((agent_id, stat.price) for agent_id, stat in self.stats.items()), key=lambda x: x[1], reverse=True)


@dataclass(slots=True)
class ProposalScheduler:
    epsilon: float = 0.1
    rng: random.Random = field(default_factory=random.Random)

    def scheduler_score(self, proposal: EvaluatedProposal, prior_price: float) -> float:
        if not proposal.signals.hard_pass:
            return float("-inf")
        return proposal.signals.score + prior_price

    def select(self, proposals: list[EvaluatedProposal], board: PriceBoard) -> EvaluatedProposal:
        valid = [p for p in proposals if p.signals.hard_pass]
        if not valid:
            # all failed. choose the least-bad proposal to preserve diagnostics.
            return max(proposals, key=lambda p: p.signals.score)

        for proposal in valid:
            prior = board.get(proposal.agent_id).price
            proposal.scheduler_score = self.scheduler_score(proposal, prior)

        if self.epsilon > 0 and self.rng.random() < self.epsilon:
            return self.rng.choice(valid)
        return max(valid, key=lambda p: p.scheduler_score)
