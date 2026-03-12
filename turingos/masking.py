from __future__ import annotations

from dataclasses import dataclass

from .models import AgentView, MachineState


@dataclass(slots=True)
class MaskingPolicy:
    def build_view(
        self,
        *,
        agent_id: str,
        state: MachineState,
        current_content: str,
    ) -> AgentView:
        return AgentView(
            agent_id=agent_id,
            state=state.frozen(),
            current_content=current_content,
        )
