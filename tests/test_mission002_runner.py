from __future__ import annotations

import json
from pathlib import Path

from harnesses.mission002_py.mission002 import (
    MODE_DUAL,
    MODE_SINGLE_27B,
    EndpointSpec,
    Mission002Config,
    deterministic_agent_factory,
    run_benchmark,
)
from harnesses.mission002_py.parity_cases import DEFAULT_GENERATOR_VERSION


def test_mission002_runner_preserves_case_stream_and_emits_artifacts(tmp_path: Path) -> None:
    config = Mission002Config(
        modes=[MODE_SINGLE_27B, MODE_DUAL],
        start_case=1,
        max_cases=2,
        global_seed=20260308,
        generator_version=DEFAULT_GENERATOR_VERSION,
        max_steps=64,
        output_root=tmp_path,
        run_name="mission002_test",
        clean=True,
        qwen27=EndpointSpec(base_url="http://127.0.0.1:18080", model="qwen27"),
        qwen9=EndpointSpec(base_url="http://127.0.0.1:18081", model="qwen9"),
    )

    summary = run_benchmark(config, agent_factory=deterministic_agent_factory)

    assert [item["status"] for item in summary["mode_summaries"]] == ["PASS", "PASS"]
    assert (tmp_path / "mission002_test" / "overall_summary.json").exists()

    single_case = json.loads(
        (tmp_path / "mission002_test" / MODE_SINGLE_27B / "case_000001" / "case_result.json").read_text(encoding="utf-8")
    )
    dual_case = json.loads(
        (tmp_path / "mission002_test" / MODE_DUAL / "case_000001" / "case_result.json").read_text(encoding="utf-8")
    )

    assert single_case["case"]["tree_digest"] == dual_case["case"]["tree_digest"]
    assert single_case["result"]["success"] is True
    assert dual_case["result"]["success"] is True
    assert "ledger.jsonl" in single_case["workspace_snapshot"]
