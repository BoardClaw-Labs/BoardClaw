#!/usr/bin/env python3
"""Collect deterministic phase-00 CI bootstrap metrics."""

from __future__ import annotations

import argparse
import json
from pathlib import Path


REQUIRED_DOCS = [
    "docs/architecture.md",
    "docs/benchmarking.md",
    "docs/board-profiles.md",
    "docs/development-plan.md",
    "docs/first-version.md",
    "docs/model-and-language.md",
    "docs/receipt-and-mobile-approval.md",
    "docs/references.md",
    "docs/risks.md",
    "docs/roadmap.md",
    "docs/technical-possibility.md",
    "docs/use-cases.md",
    "docs/vision.md",
]


def bool_metric(value: bool) -> int:
    return 1 if value else 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()

    metrics = {
        "ci.workflow_present": bool_metric(Path(".github/workflows/ci.yml").is_file()),
        "workspace.cargo_toml_present": bool_metric(Path("Cargo.toml").is_file()),
        "workspace.rust_toolchain_present": bool_metric(
            Path("rust-toolchain.toml").is_file()
        ),
        "workspace.core_crate_present": bool_metric(
            Path("crates/boardclaw-core/src/lib.rs").is_file()
        ),
        "docs.required_files_present": bool_metric(
            all(Path(doc).is_file() for doc in REQUIRED_DOCS)
        ),
        "bench.expected_files_count": len(list(Path("benchmarks/expected").glob("*.json"))),
        "bench.compare_script_present": bool_metric(
            Path("scripts/compare_benchmark.py").is_file()
        ),
        "bench.validate_script_present": bool_metric(
            Path("scripts/validate_benchmark_specs.py").is_file()
        ),
    }

    payload = {
        "id": "phase-00-ci-baseline",
        "source": "scripts/collect_phase00_metrics.py",
        "metrics": metrics,
    }
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    print(f"wrote {args.out}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
