from __future__ import annotations

import copy
import random
from dataclasses import dataclass, field
from typing import Callable

from ..models import AgentProposal, AgentView, Transition


PolicyFn = Callable[[AgentView], Transition]


@dataclass(slots=True)
class DeterministicPolicyAgent:
    agent_id: str
    policy: PolicyFn

    def propose(self, view: AgentView) -> AgentProposal:
        transition = self.policy(view)
        return AgentProposal(
            agent_id=self.agent_id,
            transition=transition,
            raw_response=f"policy phase={view.state.register.get('phase')}",
        )


@dataclass(slots=True)
class NoisyPolicyAgent:
    agent_id: str
    policy: PolicyFn
    error_rate: float = 0.25
    rng: random.Random = field(default_factory=random.Random)

    def _mutate(self, transition: Transition) -> Transition:
        t = copy.deepcopy(transition)
        choice = self.rng.choice(["path", "write_mode", "write_content", "register", "halt"])
        if choice == "path":
            # stay in place or jump to a wrong but in-bounds path.
            t.next_path = ".ls" if t.next_path != ".ls" else "parity.md"
        elif choice == "write_mode":
            t.write_mode = "overwrite" if t.write_mode == "keep" else "keep"
            if t.write_mode == "keep":
                t.write_content = None
            elif t.write_content is None:
                t.write_content = "garbage"
        elif choice == "write_content":
            if t.write_mode == "keep":
                t.write_mode = "overwrite"
            current = t.write_content or "0"
            t.write_content = "1" if current != "1" else "0"
        elif choice == "register":
            reg = copy.deepcopy(t.next_register)
            reg["phase"] = "scan" if reg.get("phase") != "scan" else "finalize"
            t.next_register = reg
        elif choice == "halt":
            t.halt = not t.halt
        return t

    def propose(self, view: AgentView) -> AgentProposal:
        transition = self.policy(view)
        if self.rng.random() < self.error_rate:
            transition = self._mutate(transition)
            raw = f"noisy phase={view.state.register.get('phase')} mutated"
        else:
            raw = f"noisy phase={view.state.register.get('phase')} exact"
        return AgentProposal(agent_id=self.agent_id, transition=transition, raw_response=raw)
