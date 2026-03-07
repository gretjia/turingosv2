from __future__ import annotations

from collections import Counter
from dataclasses import dataclass, field
from statistics import mean
from typing import Iterable


@dataclass(slots=True)
class PriceStats:
    attempts: int = 0
    accepts: int = 0
    total_reward: float = 0.0
    last_reward: float = 0.0

    @property
    def acceptance_rate(self) -> float:
        return self.accepts / self.attempts if self.attempts else 0.0

    @property
    def mean_reward(self) -> float:
        return self.total_reward / self.attempts if self.attempts else 0.0

    @property
    def price(self) -> float:
        # simple white-box pricing: reward for successful outcomes, not for reasons.
        return (self.acceptance_rate * 0.6) + (self.mean_reward * 0.4)


@dataclass(slots=True)
class ConsensusSignal:
    counts: Counter[str] = field(default_factory=Counter)

    @classmethod
    def from_answers(cls, answers: Iterable[str]) -> "ConsensusSignal":
        return cls(counts=Counter(answers))

    def majority_answer(self) -> str | None:
        if not self.counts:
            return None
        return self.counts.most_common(1)[0][0]

    def majority_strength(self) -> float:
        total = sum(self.counts.values())
        if total == 0:
            return 0.0
        _, c = self.counts.most_common(1)[0]
        return c / total


@dataclass(slots=True)
class StatisticalSignal:
    values: list[float]

    def mean(self) -> float:
        return mean(self.values) if self.values else 0.0

    def variance(self) -> float:
        if not self.values:
            return 0.0
        m = self.mean()
        return sum((v - m) ** 2 for v in self.values) / len(self.values)
