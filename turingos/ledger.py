from __future__ import annotations

import hashlib
import json
from dataclasses import dataclass
from pathlib import Path
from typing import Any


@dataclass(slots=True)
class AppendOnlyLedger:
    path: Path

    def __post_init__(self) -> None:
        self.path.parent.mkdir(parents=True, exist_ok=True)
        if not self.path.exists():
            self.path.write_text("", encoding="utf-8")

    def _line_hash(self, payload: dict[str, Any], previous_hash: str) -> str:
        canonical = json.dumps(payload, ensure_ascii=False, sort_keys=True, separators=(",", ":"))
        material = f"{previous_hash}:{canonical}".encode("utf-8")
        return hashlib.sha256(material).hexdigest()

    def append(self, payload: dict[str, Any]) -> dict[str, Any]:
        records = self.records()
        previous_hash = records[-1]["hash"] if records else "GENESIS"
        record = {
            "previous_hash": previous_hash,
            "payload": payload,
        }
        record["hash"] = self._line_hash(payload, previous_hash)
        with self.path.open("a", encoding="utf-8") as fh:
            fh.write(json.dumps(record, ensure_ascii=False, sort_keys=True) + "\n")
        return record

    def records(self) -> list[dict[str, Any]]:
        content = self.path.read_text(encoding="utf-8")
        records: list[dict[str, Any]] = []
        for line in content.splitlines():
            if not line.strip():
                continue
            records.append(json.loads(line))
        return records

    def verify_integrity(self) -> bool:
        previous_hash = "GENESIS"
        for record in self.records():
            payload = record.get("payload")
            expected = self._line_hash(payload, previous_hash)
            if record.get("previous_hash") != previous_hash:
                return False
            if record.get("hash") != expected:
                return False
            previous_hash = record["hash"]
        return True
