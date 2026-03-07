from __future__ import annotations

from typing import Protocol

from ..fs import WorkspaceFS
from ..models import MachineState, SignalBundle, Transition


class Task(Protocol):
    name: str

    def setup(self, fs: WorkspaceFS) -> None:
        ...

    def initial_state(self) -> MachineState:
        ...

    def expected_transition(self, state: MachineState, current_content: str, fs: WorkspaceFS) -> Transition:
        ...

    def verify_proposal(self, state: MachineState, current_content: str, proposal: Transition, fs: WorkspaceFS) -> SignalBundle:
        ...

    def is_success(self, state: MachineState, fs: WorkspaceFS) -> bool:
        ...

    def diagnostics(self, fs: WorkspaceFS) -> dict:
        ...
