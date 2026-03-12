from __future__ import annotations

import copy
from dataclasses import dataclass, field
from typing import Any, Literal

WriteMode = Literal["keep", "overwrite"]


@dataclass(slots=True)
class MachineState:
    """Persistent kernel state q plus the current head path d."""

    step: int
    current_path: str
    register: dict[str, Any]
    halted: bool = False

    def clone(self) -> "MachineState":
        return MachineState(
            step=self.step,
            current_path=self.current_path,
            register=copy.deepcopy(self.register),
            halted=self.halted,
        )

    def frozen(self) -> "FrozenMachineState":
        return FrozenMachineState(
            step=self.step,
            current_path=self.current_path,
            register=copy.deepcopy(self.register),
            halted=self.halted,
        )


@dataclass(slots=True)
class FrozenMachineState:
    """Immutable snapshot passed across the black-box boundary."""

    step: int
    current_path: str
    register: dict[str, Any]
    halted: bool = False


@dataclass(slots=True)
class Transition:
    """One AI-Turing-machine step: (q, s, d) -> (q', s', d')."""

    next_register: dict[str, Any]
    next_path: str
    write_mode: WriteMode = "keep"
    write_content: str | None = None
    halt: bool = False
    notes: str = ""

    def normalized(self) -> "Transition":
        if self.write_mode == "keep":
            return Transition(
                next_register=self.next_register,
                next_path=self.next_path,
                write_mode="keep",
                write_content=None,
                halt=self.halt,
                notes=self.notes,
            )
        return self


@dataclass(slots=True)
class AgentView:
    """Masked context visible to a worker."""

    agent_id: str
    state: FrozenMachineState
    current_content: str


@dataclass(slots=True)
class AgentProposal:
    agent_id: str
    transition: Transition
    raw_response: str = ""


@dataclass(slots=True)
class SignalBundle:
    hard_pass: bool
    score: float = 0.0
    hard_fail_reasons: list[str] = field(default_factory=list)
    score_components: dict[str, float] = field(default_factory=dict)
    public_feedback: list[str] = field(default_factory=list)
    private_feedback: list[str] = field(default_factory=list)
    error_fingerprints: list[str] = field(default_factory=list)

    @property
    def utility(self) -> float:
        if not self.hard_pass:
            return float("-inf")
        return self.score


@dataclass(slots=True)
class EvaluatedProposal:
    proposal: AgentProposal
    signals: SignalBundle
    scheduler_score: float = 0.0

    @property
    def agent_id(self) -> str:
        return self.proposal.agent_id

    @property
    def transition(self) -> Transition:
        return self.proposal.transition


@dataclass(slots=True)
class RunResult:
    state: MachineState
    final_files: dict[str, str]
    steps_executed: int
    selected_agents: list[str]
    success: bool
    diagnostics: dict[str, Any] = field(default_factory=dict)
