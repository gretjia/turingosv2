from __future__ import annotations

from pathlib import Path

from turingos.ledger import AppendOnlyLedger


def test_append_only_ledger_detects_tamper(tmp_path: Path) -> None:
    ledger = AppendOnlyLedger(tmp_path / "ledger.jsonl")
    ledger.append({"step": 1, "value": "a"})
    ledger.append({"step": 2, "value": "b"})
    assert ledger.verify_integrity() is True

    content = (tmp_path / "ledger.jsonl").read_text(encoding="utf-8")
    (tmp_path / "ledger.jsonl").write_text(content.replace('"b"', '"evil"'), encoding="utf-8")
    assert ledger.verify_integrity() is False
