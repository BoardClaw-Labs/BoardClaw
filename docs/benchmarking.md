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

## Result Levels

BoardClaw uses three result levels so maintainers know what was actually proven.

| Level | Environment | Required for |
|---|---|---|
| CI | GitHub Actions, no hardware | every pull request |
| Simulation | fake provider and fake hardware backend | every phase before real hardware |
| Real Environment | physical board or approved robotics simulator | profile support claims and v1 release |

Do not mark a board feature as supported from CI-only evidence. CI proves code
shape. Simulation proves routing and policy. Real environment results prove
hardware behavior.

## Real Environment Result Metadata

Every real-board result file should include:

- phase id
- board id
- board revision if known
- OS image and kernel
- BoardClaw commit SHA
- provider name and version
- model name, quantization, and context length
- enabled feature mode
- connected hardware fixture
- cooling and power notes
- metrics
- failure notes
- maintainer name or handle

Example result filename:

```text
benchmarks/results/phase-01-raspberry-pi-iot-rpi5-20260601.json
```

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

Real environment pass condition:

- detects `raspberry_pi_5`
- reads a known GPIO/I2C/UART fixture
- publishes and receives MQTT loopback
- captures camera frame or returns clear camera-unavailable error
- denies unsafe write without approval

### Orange Pi 5 Plus

Measure:

- CPU/Ollama route
- Home Assistant service simulation
- MQTT automation loop
- storage/event persistence
- RK3588 family reuse
- NPU path only when conversion is proven

Real environment pass condition:

- detects `orange_pi_5_plus`
- runs CPU local model provider route
- queries Home Assistant test instance
- records device graph and survives restart
- keeps RK acceleration experimental unless benchmarked

### Jetson Orin Nano

Measure:

- NVIDIA provider health
- camera/VLM route visibility
- ROS 2 bounded publish
- motion denial without safety metadata
- no direct model motor loop
- thermal and memory pressure

Real environment pass condition:

- detects `jetson_orin_nano`
- records Jetson provider capability
- captures or simulates camera/perception result
- publishes bounded ROS 2 command in simulation or safe test robot
- denies movement without safety metadata

## Cross-Board Demo Result

The final v1 demo result should prove:

- Raspberry Pi 5 publishes IoT state.
- Orange Pi 5 Plus receives the state and evaluates Smart Home context.
- Jetson Orin Nano provides perception or robot inspection proposal.
- one trace id links the full flow.
- one dashboard/event log explains decisions, denials, approvals, and results.
