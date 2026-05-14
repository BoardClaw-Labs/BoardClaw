#!/usr/bin/env python3
"""Compare benchmark result data against expected BoardClaw thresholds."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def compare_value(actual: Any, operator: str, expected: Any) -> bool:
    if operator == "eq":
        return actual == expected
    if operator == "gte":
        return actual >= expected
    if operator == "lte":
        return actual <= expected
    raise ValueError(f"unknown operator: {operator}")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--expected", required=True, type=Path)
    parser.add_argument("--actual", required=True, type=Path)
    args = parser.parse_args()

    expected = load_json(args.expected)
    actual = load_json(args.actual)
    actual_metrics = actual.get("metrics", {})
    if not isinstance(actual_metrics, dict):
        print(f"{args.actual}: metrics must be an object")
        return 1

    failures: list[str] = []
    for metric in expected["metrics"]:
        name = metric["name"]
        if name not in actual_metrics:
            failures.append(f"missing actual metric {name}")
            continue
        actual_value = actual_metrics[name]
        operator = metric["operator"]
        expected_value = metric["expected"]
        if not compare_value(actual_value, operator, expected_value):
            failures.append(
                f"{name}: actual {actual_value!r} does not satisfy "
                f"{operator} {expected_value!r}"
            )

    if failures:
        for failure in failures:
            print(failure)
        return 1

    print(f"{actual.get('id', args.actual.name)} satisfies {args.expected.name}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
