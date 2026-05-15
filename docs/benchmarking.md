# Benchmarking

BoardClaw stores expected benchmark thresholds before implementation and actual
benchmark results after implementation.

This protects the project from "it works on my board" drift and keeps open
source contributors aligned on measurable exit criteria.

## Directory Layout

```text
benchmarks/
  expected/
    phase-00-ci.json
    phase-01-raspberry-pi-iot.json
    phase-02-orange-pi-smart-home.json
    phase-03-jetson-robotics.json
    phase-04-shared-safety-layer.json
    phase-05-cross-board-demo.json
    phase-06-more-board-profiles.json
    phase-07-provider-acceleration.json
    phase-08-approval-receipts.json
    phase-09-release-hardening.json
  results/
    phase-00-ci-baseline.json
    phase-01-raspberry-pi-iot-rpi5-YYYYMMDD.json
```

Expected files are committed. Actual baseline files that describe meaningful
milestones should be committed. Temporary CI or local results use
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
7. Update docs if the result changes risk, support status, or defaults.

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

Hardware-dependent benchmarks must have:

- simulation path for pull requests
- real-board path for release qualification
- board/provider/model metadata in the result file
- clear failure reason when hardware is absent

## Benchmark Philosophy

Benchmarks must measure risk, not vanity.

Good metrics:

- unsafe actions denied
- daemon idle memory
- model route latency
- provider health checks
- tool-call success rate
- hallucinated-tool denial rate
- hardware timeout behavior
- restart persistence
- thermal throttling
- approval and receipt verification

Weak metrics:

- raw tokens per second with no task context
- one-off demo latency
- unsupported board performance claims
- screenshots without machine-readable results

## Reference Board Benchmarks

### Raspberry Pi 5

Measure:

- local provider health
- GPIO/I2C/UART read-only success
- MQTT loopback
- camera timeout behavior
- approved simulated GPIO write
- thermal and power warnings

### Orange Pi 5 Plus

Measure:

- CPU/Ollama route
- Home Assistant service simulation
- MQTT automation loop
- storage/event persistence
- RK3588 family reuse
- NPU path only when conversion is proven

### Jetson Orin Nano

Measure:

- NVIDIA provider health
- camera/VLM route visibility
- ROS 2 bounded publish
- motion denial without safety metadata
- no direct model motor loop
- thermal and memory pressure
