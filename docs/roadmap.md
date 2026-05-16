# Roadmap

BoardClaw should be built as three deep reference profiles on one shared core.
Each phase has a matching expected benchmark file and a documented real
environment result before support is claimed.

Supported v1 deployment styles:

- one board with one enabled feature mode
- one board with multiple enabled feature modes, if hardware allows
- hub and node deployment across specialized boards

The detailed gates, real-board test expectations, and saved-result requirements
live in [Development Plan](development-plan.md).

## Phase 0: Core Contracts

Deliverables:

- README and docs
- CI workflow
- Rust workspace
- initial `boardclaw-core` crate
- profile schema vocabulary
- provider interface vocabulary
- channel vocabulary
- tool risk model
- benchmark expectation files
- benchmark comparison scripts

Exit criteria:

- contributors can explain BoardClaw without private context
- CI can test formatting, linting, unit tests, and benchmark expectations
- phase-00 benchmark comparison passes

Benchmark:

- `benchmarks/expected/phase-00-ci.json`

## Phase 1: Raspberry Pi 5 IoT Profile

Deliverables:

- Raspberry Pi 5 profile
- GPIO/I2C/SPI/UART read-only tools
- MQTT tools
- sensor workflows
- camera capture
- local Ollama route
- llama.cpp route later
- local web, CLI, and HTTP API channels

Exit criteria:

- user can chat locally
- agent can inspect board state
- agent can scan/read safe hardware interfaces
- MQTT loopback works
- unsafe GPIO/PWM writes are denied unless explicitly approved

Benchmark:

- `benchmarks/expected/phase-01-raspberry-pi-iot.json`

## Phase 2: Orange Pi 5 Plus Smart Home Profile

Deliverables:

- Orange Pi 5 Plus profile
- RK3588 family abstraction
- Home Assistant adapter
- MQTT broker workflows
- local dashboard support
- device graph
- storage/event-history profile
- Ollama or llama.cpp CPU route first
- RKNN/RKLLM later only after CPU path works

Exit criteria:

- smart-home gateway demo works in simulation
- Home Assistant service calls are policy checked
- device graph survives restart
- RK3588 family data is reusable by later boards

Benchmark:

- `benchmarks/expected/phase-02-orange-pi-smart-home.json`

## Phase 3: Jetson Orin Nano Robotics Profile

Deliverables:

- Jetson Orin Nano profile
- ROS 2 bridge
- camera/perception tools
- bounded movement command proposals
- NVIDIA provider route
- local VLM experiments
- direct motor-loop control forbidden

Exit criteria:

- robot companion demo works in simulation
- motion commands require safety metadata
- no direct motor loop is controlled by the LLM

Benchmark:

- `benchmarks/expected/phase-03-jetson-robotics.json`

## Phase 4: Shared Safety Layer

Deliverables:

- tool permissions
- approvals
- audit events
- dry-run mode
- read-only default
- dangerous-write gates
- least-privilege hardware helper interface

Exit criteria:

- one policy system gates all three reference profiles
- risky actions can become pending
- denied actions explain why
- every write attempt records an audit event

Benchmark:

- `benchmarks/expected/phase-04-shared-safety-layer.json`

## Phase 5: Cross-Board Demo

Deliverables:

- shared cross-board event flow
- Raspberry Pi sensor/MQTT publisher
- Orange Pi smart-home automation coordinator
- Jetson camera/perception and robot inspection proposal
- one event/audit model
- one result-reporting flow

Exit criteria:

- BoardClaw is demonstrably one multi-board control system
- three boards cooperate without separate product forks
- all actions and denials are visible in one audit trail

Benchmark:

- `benchmarks/expected/phase-05-cross-board-demo.json`

## Phase 6: More Board Profiles

Targets:

- Radxa ROCK 5B+
- Banana Pi BPI-M7
- ODROID-H3+
- BeagleBone AI-64
- Libre Computer Le Potato
- ASUS Tinker Board 2S

Deliverables:

- profile tiers
- host/gateway/satellite classification
- board-specific safety notes
- RK3588 reuse measurement
- safe unknown profile

Exit criteria:

- BoardClaw can clearly say which boards are local model hosts and which are
  lightweight control nodes
- unsupported features are marked honestly

Benchmark:

- `benchmarks/expected/phase-06-more-board-profiles.json`

## Phase 7: Provider Acceleration And Model Telemetry

Deliverables:

- provider capability reports
- route telemetry
- board/provider/model benchmark collector
- Raspberry Pi accelerator route when available
- RK route experimental validation
- Jetson accelerated route
- fallback behavior

Exit criteria:

- model route is visible to the user
- CPU fallback works
- acceleration is never default without benchmark proof

Benchmark:

- `benchmarks/expected/phase-07-provider-acceleration.json`

## Phase 8: Approval And Receipts

Deliverables:

- proposal object
- pending approval flow
- local approval API
- optional mobile/PWA approval surface
- single-use approval token
- receipt metadata in event log
- proposal/approval/execution chain verification

Exit criteria:

- risky action can become pending
- valid approval executes once
- expired approval is denied
- event log can verify proposal, approval, and execution metadata

Benchmark:

- `benchmarks/expected/phase-08-approval-receipts.json`

## Phase 9: Release Hardening

Deliverables:

- install scripts
- service files
- configuration examples
- backup/restore docs
- hardware fixture docs
- threat model
- support matrix
- release checklist

Exit criteria:

- clean install works on reference boards
- user can disable all write tools
- support status is machine-readable
- release process blocks unsupported claims

Benchmark:

- `benchmarks/expected/phase-09-release-hardening.json`
