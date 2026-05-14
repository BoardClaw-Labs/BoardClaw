#!/usr/bin/env python3
"""Validate BoardClaw benchmark expectation files."""

from __future__ import annotations

import json
from pathlib import Path


EXPECTED_DIR = Path("benchmarks/expected")
REQUIRED_TOP_LEVEL = {"id", "phase", "status", "summary", "metrics"}
REQUIRED_METRIC_KEYS = {"name", "operator", "expected", "unit", "reason"}
VALID_OPERATORS = {"eq", "gte", "lte"}


def validate_file(path: Path) -> list[str]:
    errors: list[str] = []
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        return [f"{path}: invalid JSON: {exc}"]

    missing = REQUIRED_TOP_LEVEL - set(data)
    if missing:
        errors.append(f"{path}: missing top-level keys: {sorted(missing)}")

    metrics = data.get("metrics")
    if not isinstance(metrics, list) or not metrics:
        errors.append(f"{path}: metrics must be a non-empty list")
        return errors

    seen_names: set[str] = set()
    for index, metric in enumerate(metrics):
        if not isinstance(metric, dict):
            errors.append(f"{path}: metric {index} must be an object")
            continue
        missing_metric = REQUIRED_METRIC_KEYS - set(metric)
        if missing_metric:
            errors.append(
                f"{path}: metric {index} missing keys: {sorted(missing_metric)}"
            )
        name = metric.get("name")
        if not isinstance(name, str) or not name:
            errors.append(f"{path}: metric {index} name must be a non-empty string")
        elif name in seen_names:
            errors.append(f"{path}: duplicate metric name: {name}")
        else:
            seen_names.add(name)
        operator = metric.get("operator")
        if operator not in VALID_OPERATORS:
            errors.append(
                f"{path}: metric {name or index} has invalid operator {operator!r}"
            )
        expected = metric.get("expected")
        if not isinstance(expected, (int, float, str, bool)):
            errors.append(f"{path}: metric {name or index} expected must be scalar")

    return errors


def main() -> int:
    paths = sorted(EXPECTED_DIR.glob("*.json"))
    if not paths:
        print(f"no benchmark expectation files found in {EXPECTED_DIR}")
        return 1

    errors: list[str] = []
    for path in paths:
        errors.extend(validate_file(path))

    if errors:
        for error in errors:
            print(error)
        return 1

    print(f"validated {len(paths)} benchmark expectation files")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
