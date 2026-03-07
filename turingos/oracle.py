from __future__ import annotations

import random
from dataclasses import dataclass, field
from typing import Callable, Generic, TypeVar

T = TypeVar("T")


@dataclass(slots=True)
class RalphLoopResult(Generic[T]):
    candidate: T | None
    attempts: int
    accepted: bool


@dataclass(slots=True)
class RalphLoop(Generic[T]):
    """T2-style solve loop: cheap verifier, many candidate attempts."""

    max_attempts: int = 128
    rng: random.Random = field(default_factory=random.Random)

    def solve(
        self,
        generator: Callable[[random.Random], T],
        verifier: Callable[[T], bool],
    ) -> RalphLoopResult[T]:
        for attempt in range(1, self.max_attempts + 1):
            candidate = generator(self.rng)
            if verifier(candidate):
                return RalphLoopResult(candidate=candidate, attempts=attempt, accepted=True)
        return RalphLoopResult(candidate=None, attempts=self.max_attempts, accepted=False)
