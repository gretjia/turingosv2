from __future__ import annotations

import random

from turingos.oracle import RalphLoop


def test_ralph_loop_solves_easy_t2_problem() -> None:
    loop = RalphLoop[int](max_attempts=128, rng=random.Random(3))

    def generator(rng: random.Random) -> int:
        return rng.randint(10, 20)

    def verifier(candidate: int) -> bool:
        return candidate == 17

    result = loop.solve(generator, verifier)
    assert result.accepted is True
    assert result.candidate == 17
    assert result.attempts <= 128
