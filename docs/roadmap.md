# Roadmap

BoardClaw should be built in narrow, testable phases.

## Phase 0: Planning and Contracts

Status: current.

Deliverables:

- README
- docs
- CI workflow
- Rust workspace
- initial `boardclaw-core` crate
- board profile schema
- provider interface sketch
- model selection policy
- implementation language decision
- tool manifest schema
- safety policy model
- Raspberry Pi MVP definition
- benchmark expectation files
- benchmark comparison scripts

Exit criteria:

- a contributor can explain BoardClaw without private context
- the first implementation tasks are clear
- CI can test formatting, linting, unit tests, and benchmark expectations

## Phase 1: RasClaw MVP

Target: Raspberry Pi 5.

Deliverables:

- `boardclawd` daemon
- CLI channel
- local HTTP API
- Ollama provider
- suitable 1B-4B local model default
- SQLite memory
- board detection
- read-only hardware tools
- event log

Exit criteria:

- user can chat locally
- agent can inspect board state
- agent can scan/read safe hardware interfaces
- no root shell required for basic operation

## Phase 2: Safe Control

Deliverables:

- write tools behind approval
- allowlists
- dry-run mode
- hardware helper daemon
- tool timeouts
- risk levels
- local web UI for approvals

Exit criteria:

- GPIO relay demo works with approval
- I2C/UART writes are gated
- failed tools are logged clearly
- user can disable all write tools

## Phase 3: Automation

Deliverables:

- MQTT channel
- Home Assistant integration
- schedules
- device graph
- automation history
- camera capture

Exit criteria:

- local smart automation demo works
- user can create and inspect automation rules
- all automations have audit events

## Phase 4: Raspberry Pi AI HAT+ 2

Deliverables:

- accelerator detection
- Hailo-Ollama provider adapter
- model availability check
- performance telemetry
- fallback to regular Ollama

Exit criteria:

- BoardClaw can select CPU Ollama or Hailo-Ollama by profile
- model route is visible to the user

## Phase 5: RK3588 Family

Targets:

- Orange Pi 5 Plus
- Radxa ROCK 5B+
- Banana Pi BPI-M7

Deliverables:

- `rk3588` family profile
- per-board pin maps
- CPU/Ollama provider support
- RKLLM/RKNN experimental provider
- thermal/power notes

Exit criteria:

- one RK3588 board reaches Tier 2
- second RK3588 board reuses the family profile with minimal code

## Phase 6: Jetson Robotics Profile

Target: NVIDIA Jetson Orin Nano.

Deliverables:

- JetPack-aware detection
- NVIDIA provider route
- camera/VLM pipeline
- ROS 2 bridge
- bounded robot command tools

Exit criteria:

- robot companion demo works
- motion commands require safety metadata
- no direct motor loop is controlled by the LLM

## Phase 7: x86 and Low-Cost Profiles

Targets:

- ODROID-H3+
- Libre Computer Le Potato
- ASUS Tinker Board 2S
- BeagleBone AI-64

Deliverables:

- x86 gateway profile
- low-cost IoT satellite profile
- BeagleBone embedded/vision profile
- remote/LAN fallback profiles

Exit criteria:

- BoardClaw can clearly say which boards are local-LLM hosts and which are
  satellite/control nodes

## Phase 8: Uniclaw Optional Sidecar

Deliverables:

- proposal API adapter
- tool execution receipt adapter
- pending approval flow
- receipt references in BoardClaw event log

Exit criteria:

- high-risk GPIO write can require Uniclaw approval
- tool execution receipt verifies independently

## Phase 9: SecuClaw

Deliverables:

- mobile verification app or mobile web approval
- passkey/biometric approval
- receipt verification on mobile
- remote approval policies

Exit criteria:

- user can approve a risky board action from phone
- BoardClaw executes only after valid approval
- receipt chain proves proposal, approval, and execution

## Phase 10: Hardening

Deliverables:

- install scripts
- service files
- backup/restore
- profile test matrix
- security review
- threat model
- model/provider benchmark suite
- documentation for supported hardware fixtures

Exit criteria:

- BoardClaw can be installed by a normal technical user
- board support status is honest and repeatable
