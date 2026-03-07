from __future__ import annotations

import json
import random
from dataclasses import dataclass, field
from pathlib import Path
from typing import Iterable

from .broadcast import BroadcastHub
from .fs import WorkspaceFS
from .ledger import AppendOnlyLedger
from .masking import MaskingPolicy
from .models import EvaluatedProposal, MachineState, RunResult
from .scheduler import PriceBoard, ProposalScheduler
from .tasks.base import Task


@dataclass(slots=True)
class TuringOSConfig:
    max_steps: int = 128
    explore_epsilon: float = 0.05
    random_seed: int = 0
    ledger_filename: str = "ledger.jsonl"
    abort_on_no_valid_proposal: bool = True


@dataclass(slots=True)
class TuringOSKernel:
    workspace_root: Path
    task: Task
    agents: list
    config: TuringOSConfig = field(default_factory=TuringOSConfig)
    fs: WorkspaceFS | None = None
    state: MachineState | None = None
    board: PriceBoard = field(default_factory=PriceBoard)
    broadcasts: BroadcastHub = field(default_factory=BroadcastHub)
    masking: MaskingPolicy = field(default_factory=MaskingPolicy)
    scheduler: ProposalScheduler | None = None
    ledger: AppendOnlyLedger | None = None
    selected_agents: list[str] = field(default_factory=list)

    def __post_init__(self) -> None:
        if self.fs is None:
            self.fs = WorkspaceFS(Path(self.workspace_root))
        if self.scheduler is None:
            self.scheduler = ProposalScheduler(epsilon=self.config.explore_epsilon, rng=random.Random(self.config.random_seed))
        if self.ledger is None:
            self.ledger = AppendOnlyLedger(Path(self.workspace_root) / self.config.ledger_filename)
        self.task.setup(self.fs)
        if self.state is None:
            self.state = self.task.initial_state()

    def _agent_view(self, agent_id: str):
        assert self.state is not None
        assert self.fs is not None
        current_content = self.fs.read_text(self.state.current_path)
        public_messages, private_messages = self.broadcasts.visible_to(agent_id)
        return self.masking.build_view(
            agent_id=agent_id,
            state=self.state,
            current_content=current_content,
            public_messages=public_messages,
            private_messages=private_messages,
            price_stats=self.board.stats.get(agent_id),
        )

    def _evaluate_proposals(self) -> list[EvaluatedProposal]:
        assert self.state is not None
        assert self.fs is not None
        current_content = self.fs.read_text(self.state.current_path)
        evaluated: list[EvaluatedProposal] = []
        for agent in self.agents:
            view = self._agent_view(agent.agent_id)
            proposal = agent.propose(view)
            signals = self.task.verify_proposal(self.state, current_content, proposal.transition, self.fs)
            evaluated.append(EvaluatedProposal(proposal=proposal, signals=signals))
        return evaluated

    def _record_feedback(self, evaluated: list[EvaluatedProposal], chosen: EvaluatedProposal) -> None:
        for candidate in evaluated:
            for msg in candidate.signals.private_feedback:
                self.broadcasts.record_private(candidate.agent_id, msg)
            public_msg = candidate.signals.public_feedback[0] if candidate.signals.public_feedback else ""
            for fingerprint in candidate.signals.error_fingerprints:
                self.broadcasts.record_public_error(fingerprint, public_msg)

            accepted = candidate is chosen and candidate.signals.hard_pass
            reward = candidate.signals.score if accepted else (candidate.signals.score * 0.25 if candidate.signals.hard_pass else candidate.signals.score)
            self.board.update(candidate.agent_id, accepted=accepted, reward=reward)

        ranking = self.board.ranking()[:3]
        if ranking:
            summary = "price 排名：" + " > ".join(agent_id for agent_id, _ in ranking)
            self.broadcasts.publish_price_summary(summary)

    def _apply(self, chosen: EvaluatedProposal) -> None:
        assert self.state is not None
        assert self.fs is not None
        assert self.ledger is not None

        current_path = self.state.current_path
        current_content = self.fs.read_text(current_path)
        transition = chosen.transition.normalized()

        if transition.write_mode == "overwrite":
            self.fs.write_text(current_path, transition.write_content or "")
            self.fs.generate_ls_files()

        next_state = MachineState(
            step=self.state.step + 1,
            current_path=transition.next_path,
            register=transition.next_register,
            halted=transition.halt,
        )
        self.selected_agents.append(chosen.agent_id)

        self.ledger.append(
            {
                "step": self.state.step,
                "selected_agent": chosen.agent_id,
                "current_path": current_path,
                "current_content": current_content,
                "transition": {
                    "next_path": transition.next_path,
                    "write_mode": transition.write_mode,
                    "write_content": transition.write_content,
                    "halt": transition.halt,
                    "next_register": transition.next_register,
                },
                "signals": {
                    "hard_pass": chosen.signals.hard_pass,
                    "score": chosen.signals.score,
                    "score_components": chosen.signals.score_components,
                },
                "price_ranking": self.board.ranking()[:5],
            }
        )
        self.state = next_state

    def step_once(self) -> None:
        assert self.state is not None
        evaluated = self._evaluate_proposals()
        chosen = self.scheduler.select(evaluated, self.board)
        if self.config.abort_on_no_valid_proposal and not chosen.signals.hard_pass:
            reasons = "; ".join(chosen.signals.hard_fail_reasons)
            raise RuntimeError(f"no valid proposal at step {self.state.step}: {reasons}")
        self._record_feedback(evaluated, chosen)
        self._apply(chosen)

    def run(self) -> RunResult:
        assert self.state is not None
        assert self.fs is not None
        for _ in range(self.config.max_steps):
            if self.state.halted:
                break
            self.step_once()
            if self.task.is_success(self.state, self.fs):
                break

        success = self.task.is_success(self.state, self.fs)
        return RunResult(
            state=self.state,
            final_files=self.fs.snapshot(include_hidden=False),
            steps_executed=self.state.step,
            selected_agents=list(self.selected_agents),
            success=success,
            diagnostics={
                "task": self.task.diagnostics(self.fs),
                "ledger_ok": self.ledger.verify_integrity() if self.ledger else False,
                "price_ranking": self.board.ranking(),
                "public_broadcasts": self.broadcasts.maybe_publish_typical_errors(),
            },
        )

    def dump_run(self, path: Path) -> None:
        result = self.run()
        payload = {
            "success": result.success,
            "steps": result.steps_executed,
            "state": {
                "step": result.state.step,
                "current_path": result.state.current_path,
                "register": result.state.register,
                "halted": result.state.halted,
            },
            "selected_agents": result.selected_agents,
            "diagnostics": result.diagnostics,
        }
        path.write_text(json.dumps(payload, ensure_ascii=False, indent=2), encoding="utf-8")
