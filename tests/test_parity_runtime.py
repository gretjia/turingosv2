from __future__ import annotations


def test_parity_runtime_completes_with_deterministic_agent(parity_kernel_factory) -> None:
    kernel = parity_kernel_factory(noisy_agents=0, epsilon=0.0, seed=11)
    result = kernel.run()

    assert result.success is True
    assert result.final_files["result.md"] == "odd"
    assert result.diagnostics["ledger_ok"] is True
    assert result.selected_agents == ["alpha"] * result.steps_executed


def test_parity_runtime_survives_noisy_agents_via_white_box_control(parity_kernel_factory) -> None:
    kernel = parity_kernel_factory(noisy_agents=4, error_rate=0.55, epsilon=0.0, seed=17)
    result = kernel.run()

    assert result.success is True
    assert result.final_files["result.md"] == "odd"
    assert result.diagnostics["ledger_ok"] is True
    # The deterministic agent should dominate pricing after noisy failures accumulate.
    assert result.diagnostics["price_ranking"][0][0] == "alpha"
    assert any(msg.startswith("典型错误") for msg in result.diagnostics["public_broadcasts"])
