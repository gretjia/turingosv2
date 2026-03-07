from __future__ import annotations

import random
from dataclasses import dataclass, field
from typing import Callable, Iterable, Sequence

from .models import SignalBundle

Predicate = Callable[[object], tuple[bool, str, str]]
AuditSlot = tuple[str, str]


@dataclass(slots=True)
class PredicateVerifier:
    predicates: Sequence[Predicate]

    def verify(self, value: object) -> SignalBundle:
        reasons: list[str] = []
        private: list[str] = []
        public: list[str] = []
        for predicate in self.predicates:
            ok, public_msg, private_msg = predicate(value)
            if not ok:
                reasons.append(private_msg or public_msg or predicate.__name__)
                if public_msg:
                    public.append(public_msg)
                if private_msg:
                    private.append(private_msg)
        return SignalBundle(
            hard_pass=not reasons,
            score=1.0 if not reasons else -float(len(reasons)),
            hard_fail_reasons=reasons,
            public_feedback=public,
            private_feedback=private,
            error_fingerprints=[reason.split(":", 1)[0] for reason in reasons],
        )


@dataclass(slots=True)
class RandomAuditVerifier:
    """T5-style verifier: sample random local slots to catch hidden errors."""

    sample_size: int
    rng: random.Random = field(default_factory=random.Random)

    def audit(
        self,
        slots: Sequence[AuditSlot],
        checker: Callable[[AuditSlot], bool],
    ) -> SignalBundle:
        if not slots:
            return SignalBundle(hard_pass=True, score=1.0)
        size = min(self.sample_size, len(slots))
        sampled = self.rng.sample(list(slots), size)
        failures: list[str] = []
        for slot in sampled:
            if not checker(slot):
                failures.append(slot[0])
        return SignalBundle(
            hard_pass=not failures,
            score=1.0 if not failures else -float(len(failures)),
            hard_fail_reasons=[f"audit:{name}" for name in failures],
            public_feedback=["典型错误：随机抽查发现局部证据不一致。"] if failures else [],
            private_feedback=[f"audit failed at {name}" for name in failures],
            error_fingerprints=["audit_mismatch"] if failures else [],
        )


@dataclass(slots=True)
class ExactTransitionVerifier:
    """Demo helper: accept iff the transition equals the expected white-box policy."""

    comparator: Callable[[object, object], tuple[bool, str | None]]

    def verify(self, expected: object, actual: object) -> SignalBundle:
        ok, detail = self.comparator(expected, actual)
        if ok:
            return SignalBundle(hard_pass=True, score=1.0, score_components={"exact": 1.0})
        msg = detail or "transition_mismatch"
        return SignalBundle(
            hard_pass=False,
            score=-1.0,
            hard_fail_reasons=[msg],
            public_feedback=["典型错误：提案与白盒策略不一致。"],
            private_feedback=[msg],
            error_fingerprints=[msg.split(":", 1)[0]],
        )
