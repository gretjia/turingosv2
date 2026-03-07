from __future__ import annotations

from collections import Counter, defaultdict, deque
from dataclasses import dataclass, field


@dataclass(slots=True)
class BroadcastHub:
    """Top-level white-box signal broadcaster.

    Public broadcasts contain only coarse error categories and price-like summaries.
    Private feedback is agent-local and may be more specific.
    """

    public_errors: Counter[str] = field(default_factory=Counter)
    public_messages: dict[str, str] = field(default_factory=dict)
    private_feedback: dict[str, deque[str]] = field(default_factory=lambda: defaultdict(lambda: deque(maxlen=6)))
    public_price_messages: deque[str] = field(default_factory=lambda: deque(maxlen=8))
    threshold: int = 2

    def record_private(self, agent_id: str, message: str) -> None:
        if not message:
            return
        self.private_feedback[agent_id].append(message)

    def record_public_error(self, fingerprint: str, message: str) -> None:
        if not fingerprint:
            return
        self.public_errors[fingerprint] += 1
        if message:
            self.public_messages[fingerprint] = message

    def maybe_publish_typical_errors(self) -> list[str]:
        published: list[str] = []
        for fingerprint, count in self.public_errors.items():
            if count >= self.threshold and fingerprint in self.public_messages:
                published.append(self.public_messages[fingerprint])
        return sorted(set(published))

    def publish_price_summary(self, summary: str) -> None:
        if summary:
            self.public_price_messages.append(summary)

    def visible_to(self, agent_id: str) -> tuple[list[str], list[str]]:
        public = self.maybe_publish_typical_errors() + list(self.public_price_messages)
        private = list(self.private_feedback.get(agent_id, []))
        return public[-8:], private[-4:]
