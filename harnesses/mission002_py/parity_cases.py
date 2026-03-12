from __future__ import annotations

import hashlib
import json
import random
from dataclasses import dataclass
from typing import Any


DEFAULT_GENERATOR_VERSION = "mission002.parity.v1"


@dataclass(slots=True)
class ParityCase:
    case_index: int
    global_seed: int
    generator_version: str
    case_seed: int
    tree: dict[str, Any]

    def tree_digest(self) -> str:
        payload = json.dumps(self.tree, ensure_ascii=True, sort_keys=True)
        return hashlib.sha256(payload.encode("utf-8")).hexdigest()

    def manifest(self) -> dict[str, Any]:
        return {
            "case_index": self.case_index,
            "global_seed": self.global_seed,
            "generator_version": self.generator_version,
            "case_seed": self.case_seed,
            "tree_digest": self.tree_digest(),
            "tree": self.tree,
        }


def generate_parity_case(
    *,
    global_seed: int,
    case_index: int,
    generator_version: str = DEFAULT_GENERATOR_VERSION,
) -> ParityCase:
    case_seed = _derive_case_seed(global_seed=global_seed, case_index=case_index, generator_version=generator_version)
    rng = random.Random(case_seed)
    tree = _build_tree(rng)
    return ParityCase(
        case_index=case_index,
        global_seed=global_seed,
        generator_version=generator_version,
        case_seed=case_seed,
        tree=tree,
    )


def _derive_case_seed(*, global_seed: int, case_index: int, generator_version: str) -> int:
    digest = hashlib.sha256(f"{generator_version}:{global_seed}:{case_index}".encode("utf-8")).digest()
    return int.from_bytes(digest[:8], "big")


def _build_tree(rng: random.Random) -> dict[str, Any]:
    tree: dict[str, Any] = {}
    dir_paths: list[tuple[str, ...]] = [()]
    max_depth = 3
    dir_count = 3 + rng.randrange(4)
    for i in range(dir_count):
        eligible = [path for path in dir_paths if len(path) < max_depth]
        parent = rng.choice(eligible)
        child = parent + (f"dir_{i:02d}_{rng.randint(10, 99)}",)
        dir_paths.append(child)
        _ensure_dir(tree, child)

    file_count = 5 + rng.randrange(6)
    for i in range(file_count):
        parent = rng.choice(dir_paths)
        container = _ensure_dir(tree, parent)
        name = _unique_file_name(container, f"n_{i:02d}_{rng.randint(100, 999)}.md")
        container[name] = str(rng.randint(0, 999_999))

    return tree


def _ensure_dir(tree: dict[str, Any], path: tuple[str, ...]) -> dict[str, Any]:
    node = tree
    for part in path:
        child = node.setdefault(part, {})
        if not isinstance(child, dict):
            raise ValueError(f"path collision at {part}")
        node = child
    return node


def _unique_file_name(container: dict[str, Any], base_name: str) -> str:
    if base_name not in container:
        return base_name
    stem = base_name[:-3]
    suffix = 1
    while True:
        candidate = f"{stem}_{suffix:02d}.md"
        if candidate not in container:
            return candidate
        suffix += 1
