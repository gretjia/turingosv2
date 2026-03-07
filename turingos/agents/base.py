from __future__ import annotations

from typing import Protocol

from ..models import AgentProposal, AgentView


class Agent(Protocol):
    agent_id: str

    def propose(self, view: AgentView) -> AgentProposal:
        ...
