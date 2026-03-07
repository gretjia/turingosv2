from __future__ import annotations

import random

from turingos.verifiers import PredicateVerifier, RandomAuditVerifier


def test_predicate_verifier_collects_failures() -> None:
    def positive(x: object):
        return (isinstance(x, int) and x > 0, "public:positive", "positive:must be > 0")

    verifier = PredicateVerifier([positive])
    ok = verifier.verify(1)
    bad = verifier.verify(0)

    assert ok.hard_pass is True
    assert bad.hard_pass is False
    assert "positive:must be > 0" in bad.hard_fail_reasons[0]


def test_random_audit_verifier_catches_local_corruption_probabilistically() -> None:
    slots = [(f"slot-{i}", str(i)) for i in range(100)]
    corrupted = {f"slot-{i}" for i in range(10)}

    def checker(slot):
        name, _ = slot
        return name not in corrupted

    hits = 0
    for seed in range(200):
        verifier = RandomAuditVerifier(sample_size=5, rng=random.Random(seed))
        bundle = verifier.audit(slots, checker)
        if not bundle.hard_pass:
            hits += 1

    # Deterministic under fixed seeds; should catch many but not all.
    assert 60 <= hits <= 110
