from __future__ import annotations

from dataclasses import dataclass
from typing import Iterable

from .models import AgentView, MachineState
from .signals import PriceStats


@dataclass(slots=True)
class MaskingPolicy:
    max_public_messages: int = 6
    max_private_messages: int = 3

    def price_hint(self, stats: PriceStats | None) -> str:
        if stats is None or stats.attempts < 2:
            return "neutral"
        if stats.price >= 0.8:
            return "high"
        if stats.price >= 0.4:
            return "medium"
        return "low"

    def build_view(
        self,
        *,
        agent_id: str,
        state: MachineState,
        current_content: str,
        public_messages: Iterable[str],
        private_messages: Iterable[str],
        price_stats: PriceStats | None,
    ) -> AgentView:
        # Deliberately omit other agents' proposals and the raw scoring formula.
        return AgentView(
            agent_id=agent_id,
            state=state,
            current_content=current_content,
            public_broadcasts=list(public_messages)[-self.max_public_messages :],
            private_feedback=list(private_messages)[-self.max_private_messages :],
            price_hint=self.price_hint(price_stats),
        )
