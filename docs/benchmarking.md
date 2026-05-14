# Benchmarking

BoardClaw stores expected benchmark thresholds before implementation and actual
benchmark results after implementation.

This protects the project from "it works on my board" drift.

## Directory Layout

```text
benchmarks/
  expected/
    phase-00-ci.json
    phase-01-rasclaw-mvp.json
    ...
  results/
    phase-00-ci-baseline.json
    phase-01-rasclaw-mvp-rpi5-YYYYMMDD.json
```

Expected files are committed. Actual baseline files that describe meaningful
project milestones should be committed. Temporary CI or local results use
`*.latest.json` or `*.local.json` and are ignored.

## Benchmark File Contract

Each expected file contains:

- `id`: stable benchmark set id
- `phase`: development phase
- `status`: `active` or `planned`
- `summary`: what the benchmark protects
- `metrics`: expected thresholds

Metric operators:

- `eq`: actual value must equal expected value
- `gte`: actual value must be greater than or equal to expected value
- `lte`: actual value must be less than or equal to expected value

## Required Workflow

1. Add or update the expected benchmark file.
2. Implement the feature.
3. Run tests.
4. Collect actual benchmark results.
5. Compare actual results with expected thresholds.
6. Save meaningful baseline results in `benchmarks/results/`.
7. Update docs if the result changes the risk or support status.

## Commands

Validate expected benchmark files:

```bash
python3 scripts/validate_benchmark_specs.py
```

Collect phase-00 metrics:

```bash
python3 scripts/collect_phase00_metrics.py --out benchmarks/results/phase-00-ci.local.json
```

Compare actual data to expected thresholds:

```bash
python3 scripts/compare_benchmark.py \
  --expected benchmarks/expected/phase-00-ci.json \
  --actual benchmarks/results/phase-00-ci.local.json
```

## CI Rules

CI must run:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace --all-targets`
- benchmark spec validation
- phase-00 benchmark comparison

Later hardware phases should add dedicated workflows or self-hosted runners.
Hardware-dependent benchmarks must have a simulation path for pull requests and
a real-board path for release qualification.

## Benchmark Philosophy

Benchmarks must measure risk, not vanity.

Good metrics:

- whether unsafe actions are denied
- daemon idle memory
- model route latency
- tool-call success rate
- hallucinated-tool rate
- hardware timeout behavior
- restart persistence
- receipt verification

Weak metrics:

- raw tokens per second with no task context
- one-off demo latency
- unsupported board performance claims
- screenshots without machine-readable results

