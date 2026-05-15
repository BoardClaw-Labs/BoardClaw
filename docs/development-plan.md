# Development Plan

BoardClaw development is CI-first and benchmark-gated.

Every step must define the expected result before the feature is considered
complete. After implementation, the step must run tests, collect benchmark data,
compare actual data with the expected thresholds under `benchmarks/expected/`,
and save meaningful baseline results under `benchmarks/results/`.

## Big Picture

```text
BoardClaw Core
  + Raspberry Pi 5 profile      -> IoT / sensors / GPIO / MQTT
  + Orange Pi 5 Plus profile    -> Smart Home / gateway / local services
  + Jetson Orin Nano profile    -> Robotics / vision / ROS 2 / VLM
```

These are three reference profiles, not three products. They must share:

- one BoardClaw core
- one profile schema
- one provider interface
- one tool schema system
- one policy system
- one event/audit model
- one benchmark workflow

## Official Phase Plan

```text
Phase 00: Core Contracts
Phase 01: Raspberry Pi 5 IoT Profile
Phase 02: Orange Pi 5 Plus Smart Home Profile
Phase 03: Jetson Orin Nano Robotics Profile
Phase 04: Shared Safety Layer
Phase 05: Cross-Board Demo
Phase 06: More Board Profiles
Phase 07: Provider Acceleration And Model Telemetry
Phase 08: Approval And Receipts
Phase 09: Release Hardening
```

The safety rule still applies from day one: no model gets direct hardware
authority. Phase 04 is where shared safety becomes a complete reusable layer
across the three profiles.

## Development Rule

No phase is complete until:

- CI passes.
- Unit tests pass.
- Integration or simulation tests pass.
- Expected benchmark thresholds exist.
- Actual benchmark results are collected.
- Actual results pass the expected thresholds.
- The baseline result is saved when it represents a milestone.
- Risk notes are updated when the implementation changes the safety profile.

## Phase 00: Core Contracts

Goal: create the shared BoardClaw foundation before board work begins.

Deliverables:

- Rust workspace
- initial `boardclaw-core` crate
- board profile schema vocabulary
- provider interface vocabulary
- tool risk vocabulary
- channel vocabulary
- benchmark specification format
- GitHub Actions CI
- required docs
- phase-00 baseline result

Benchmark file:

- `benchmarks/expected/phase-00-ci.json`

Expected result:

- CI workflow exists.
- Rust workspace exists.
- core crate exists.
- required docs exist.
- benchmark scripts exist.
- expected benchmark files validate.

## Phase 01: Raspberry Pi 5 IoT Profile

Goal: prove the IoT reference profile with local model routing and safe board
inspection.

Deliverables:

- Raspberry Pi 5 profile
- board detection
- Ollama first, llama.cpp later
- GPIO/I2C/SPI/UART read-only tools
- MQTT publish/subscribe
- camera capture
- SQLite memory
- local web, CLI, and local HTTP API channels
- approval-gated GPIO/PWM write in simulation first

Benchmark file:

- `benchmarks/expected/phase-01-raspberry-pi-iot.json`

Expected result:

- local provider health check succeeds
- board detection succeeds on target hardware
- read-only tools succeed in simulation and on a test fixture
- MQTT loopback meets latency threshold
- unsafe write is denied without approval

## Phase 02: Orange Pi 5 Plus Smart Home Profile

Goal: prove the smart-home gateway reference and RK3588 family abstraction.

Deliverables:

- Orange Pi 5 Plus profile
- RK3588 family profile base
- Ollama or llama.cpp CPU route first
- Home Assistant state and service adapters
- MQTT broker workflows
- local dashboard support
- device graph
- storage/event-history profile
- RKNN/RKLLM marked experimental until CPU path works and benchmarks pass

Benchmark file:

- `benchmarks/expected/phase-02-orange-pi-smart-home.json`

Expected result:

- Orange Pi profile reaches Tier 2
- Home Assistant service simulation passes
- MQTT loopback meets latency threshold
- device graph survives restart
- CPU/Ollama route works before any NPU dependency

## Phase 03: Jetson Orin Nano Robotics Profile

Goal: prove the robotics, vision, ROS 2, and VLM reference profile without
turning the LLM into a motor controller.

Deliverables:

- Jetson Orin Nano profile
- Ollama/llama.cpp text control route first
- NVIDIA provider route for robotics/perception
- camera/perception tools
- ROS 2 bridge
- bounded movement command proposal
- safety-state tool inputs
- direct motor-loop control forbidden

Benchmark file:

- `benchmarks/expected/phase-03-jetson-robotics.json`

Expected result:

- Jetson provider route is visible and health-checked
- ROS 2 bounded publish works in simulation
- motion without safety metadata is denied
- direct model motor-loop control is impossible

## Phase 04: Shared Safety Layer

Goal: turn the safety behavior from profile-local checks into one reusable
BoardClaw safety layer.

Deliverables:

- tool permissions
- approval-required write tools
- audit events
- dry-run mode
- read-only default
- dangerous-write gates
- least-privilege hardware helper interface
- timeout handling
- denial, pending, approval, and execution events

Benchmark file:

- `benchmarks/expected/phase-04-shared-safety-layer.json`

Expected result:

- unsafe writes are denied without approval
- approved simulated write succeeds once
- raw shell is unavailable by default
- every write produces an audit event
- same policy behavior works across all three reference profiles

## Phase 05: Cross-Board Demo

Goal: show BoardClaw as a real multi-board control system.

Demo shape:

```text
Raspberry Pi 5:
  reads IoT sensors and publishes MQTT state

Orange Pi 5 Plus:
  receives MQTT, checks Home Assistant, decides smart-home action

Jetson Orin Nano:
  uses camera/perception, proposes safe robot inspection movement
```

Benchmark file:

- `benchmarks/expected/phase-05-cross-board-demo.json`

Expected result:

- all three profiles participate through one event/audit model
- incompatible tools are hidden per board
- cross-board automation state is deterministic
- channel responses explain actions and denials clearly

## Phase 06: More Board Profiles

Goal: expand only after the three-board triangle is stable.

Targets:

- Radxa ROCK 5B+
- Banana Pi BPI-M7
- ODROID-H3+
- BeagleBone AI-64
- Libre Computer Le Potato
- ASUS Tinker Board 2S

Benchmark file:

- `benchmarks/expected/phase-06-more-board-profiles.json`

Expected result:

- each board has an honest role classification
- low-end boards do not default to large local models
- unknown boards remain read-only
- RK3588 family reuse is measured

## Phase 07: Provider Acceleration And Model Telemetry

Goal: add acceleration only where it is measurable and safe to route.

Benchmark file:

- `benchmarks/expected/phase-07-provider-acceleration.json`

Expected result:

- CPU fallback works when acceleration is missing
- route selection is recorded
- memory and latency metrics are saved per board/provider/model
- acceleration never becomes default without benchmark proof

## Phase 08: Approval And Receipts

Goal: high-risk actions can become pending, approved, executed once, and
verified through durable metadata.

Benchmark file:

- `benchmarks/expected/phase-08-approval-receipts.json`

Expected result:

- pending approval blocks execution
- expired approval is denied
- valid approval executes only once
- proposal, approval, and execution records can be verified as a chain

## Phase 09: Release Hardening

Goal: make BoardClaw installable, understandable, and honest for open-source
users.

Benchmark file:

- `benchmarks/expected/phase-09-release-hardening.json`

Expected result:

- clean install works on each reference board
- user can disable all write tools
- profile support status is machine-readable
- release checklist blocks unsupported claims
