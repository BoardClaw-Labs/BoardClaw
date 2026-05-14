# Contributing

BoardClaw is correctness-first. Contributions should make the project safer,
clearer, or more capable without weakening the board/profile boundaries.

## Before Opening a Pull Request

Run:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
python3 scripts/validate_benchmark_specs.py
python3 scripts/collect_phase00_metrics.py --out benchmarks/results/phase-00-ci.local.json
python3 scripts/compare_benchmark.py \
  --expected benchmarks/expected/phase-00-ci.json \
  --actual benchmarks/results/phase-00-ci.local.json
```

## Benchmark Rule

Every feature phase needs expected benchmark data before it is considered done.
If a change touches safety, hardware control, model routing, provider behavior,
or memory, add or update benchmark expectations in `benchmarks/expected/`.

## Safety Rule

The model is not trusted. Hardware writes, shell execution, firmware flashing,
robot motion, and private-network control APIs must be typed, gated, audited,
and testable.

## Language Rule

The trusted core is Rust. TypeScript is for UI. Python is allowed for vendor SDK
bridges when needed, behind a process boundary.
