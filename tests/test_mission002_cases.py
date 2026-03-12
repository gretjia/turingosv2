from __future__ import annotations

from harnesses.mission002_py.parity_cases import DEFAULT_GENERATOR_VERSION, generate_parity_case


def _count_numeric_files(tree: dict) -> int:
    count = 0
    for value in tree.values():
        if isinstance(value, dict):
            count += _count_numeric_files(value)
        else:
            count += 1
    return count


def test_mission002_case_generator_is_deterministic() -> None:
    case_a = generate_parity_case(global_seed=17, case_index=3, generator_version=DEFAULT_GENERATOR_VERSION)
    case_b = generate_parity_case(global_seed=17, case_index=3, generator_version=DEFAULT_GENERATOR_VERSION)

    assert case_a.case_seed == case_b.case_seed
    assert case_a.tree == case_b.tree
    assert case_a.tree_digest() == case_b.tree_digest()


def test_mission002_case_generator_changes_across_case_index() -> None:
    case_a = generate_parity_case(global_seed=17, case_index=3, generator_version=DEFAULT_GENERATOR_VERSION)
    case_b = generate_parity_case(global_seed=17, case_index=4, generator_version=DEFAULT_GENERATOR_VERSION)

    assert case_a.case_seed != case_b.case_seed
    assert case_a.tree_digest() != case_b.tree_digest()
    assert _count_numeric_files(case_a.tree) >= 5
    assert _count_numeric_files(case_b.tree) >= 5
