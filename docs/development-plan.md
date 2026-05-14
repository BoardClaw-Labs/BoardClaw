# Development Plan

BoardClaw development is CI-first and benchmark-gated.

Every implementation step must define its expected result before the feature is
treated as complete. After implementation, the step must run tests and collect
benchmark data, then compare the actual data with the expected thresholds stored
under `benchmarks/expected/`.

## Big Picture

```text
BoardClaw
  Rust core
    boardclawd
    boardclaw-hwd
    provider router
    tool registry
    policy engine
    memory/event log
       |
       +--> RasClaw profile first
       |
       +--> RK3588 family
       |
       +--> Jetson robotics profile
       |
       +--> x86/low-cost/industrial profiles
       |
       +--> optional Uniclaw/SecuClaw proof and approval layer
```

The first success path is intentionally narrow:

```text
Raspberry Pi 5
  + Rust daemon
  + Ollama local model
  + SQLite memory
  + CLI/local HTTP channel
  + read-only hardware tools
  + approval-gated GPIO write
  + benchmarked and CI-gated
```

## Development Rule

No phase is complete until:

- CI passes.
- Unit tests pass.
- Integration or simulation tests pass.
- Expected benchmark thresholds exist.
- Actual benchmark results are collected.
- Actual results pass the expected thresholds.
- Risk notes are updated if the implementation changes the safety profile.

## Phase 00: CI and Project Contracts

Goal: make the repository testable before adding real features.

Deliverables:

- Rust workspace
- initial `boardclaw-core` crate
- GitHub Actions CI
- benchmark specification format
- expected benchmark files
- phase-00 actual baseline result

Benchmark file:

- `benchmarks/expected/phase-00-ci.json`

Expected result:

- CI workflow exists.
- Rust workspace exists.
- core crate exists.
- required docs exist.
- benchmark scripts exist.
- benchmark expected files validate.

## Phase 01: RasClaw MVP

Goal: Raspberry Pi local chat and read-only board inspection.

Deliverables:

- `boardclawd` service
- CLI channel
- local HTTP API
- Ollama provider
- model route metadata
- SQLite memory
- Raspberry Pi detection
- read-only hardware tools

Benchmark file:

- `benchmarks/expected/phase-01-rasclaw-mvp.json`

Expected result:

- daemon starts on Pi and Linux CI simulation
- local provider health check succeeds
- board detection succeeds on Pi and returns safe unknown profile elsewhere
- read-only tools succeed in simulation
- memory writes and reads are correct

## Phase 02: Safe Control

Goal: write tools exist, but dangerous actions cannot execute accidentally.

Deliverables:

- `boardclaw-hwd` hardware helper
- tool risk levels
- policy gate
- approval-required write tools
- dry-run mode
- timeout handling

Benchmark file:

- `benchmarks/expected/phase-02-safe-control.json`

Expected result:

- unsafe writes are denied without approval
- approved simulated GPIO write succeeds
- policy evaluation is fast enough for interactive use
- all write tools emit audit events

## Phase 03: Automation

Goal: BoardClaw can coordinate IoT and smart automation workflows.

Deliverables:

- MQTT channel
- Home Assistant adapter
- scheduled tasks
- device graph
- automation event history
- camera capture interface

Benchmark file:

- `benchmarks/expected/phase-03-automation.json`

Expected result:

- MQTT loopback publish/subscribe passes
- automation rule evaluation is deterministic
- device graph survives restart
- camera capture has clear timeout and error behavior

## Phase 04: Raspberry Pi AI HAT+ 2

Goal: optional accelerated Raspberry Pi provider route.

Deliverables:

- accelerator detection
- Hailo-Ollama provider adapter
- model availability check
- route fallback to CPU Ollama
- performance telemetry

Benchmark file:

- `benchmarks/expected/phase-04-raspberry-pi-ai-hat.json`

Expected result:

- route selection is correct with and without accelerator
- provider failure falls back safely
- performance telemetry is recorded

## Phase 05: RK3588 Family

Goal: add the first non-Raspberry Pi family without rewriting the core.

Targets:

- Orange Pi 5 Plus
- Radxa ROCK 5B+
- Banana Pi BPI-M7

Benchmark file:

- `benchmarks/expected/phase-05-rk3588-family.json`

Expected result:

- one RK3588 board reaches Tier 2
- a second RK3588 board reuses the family profile with minimal code
- CPU/Ollama route works before NPU work is required

## Phase 06: Jetson Robotics Profile

Goal: prove BoardClaw can support robotics/VLM without violating real-time
safety boundaries.

Benchmark file:

- `benchmarks/expected/phase-06-jetson-robotics.json`

Expected result:

- Jetson provider route works
- camera/VLM route is visible
- ROS 2 bridge can publish bounded commands
- robot motion tools require safety metadata and approval

## Phase 07: Broad Board Profiles

Goal: add x86, embedded-control, low-cost, and industrial profiles.

Targets:

- ODROID-H3+
- BeagleBone AI-64
- Libre Computer Le Potato
- ASUS Tinker Board 2S

Benchmark file:

- `benchmarks/expected/phase-07-broad-board-profiles.json`

Expected result:

- profiles can clearly report whether the board is an LLM host, gateway, or
  satellite node
- low-end boards do not pretend to run large models

## Phase 08: Uniclaw Sidecar

Goal: add proof without rewriting the BoardClaw tool layer.

Benchmark file:

- `benchmarks/expected/phase-08-uniclaw-sidecar.json`

Expected result:

- BoardClaw can submit proposal events
- pending approvals block execution
- tool execution receipts are recorded
- receipt IDs are attached to BoardClaw events

## Phase 09: SecuClaw Mobile Verification

Goal: mobile verification for high-risk board actions.

Benchmark file:

- `benchmarks/expected/phase-09-secuclaw-mobile.json`

Expected result:

- mobile approval request displays the correct action summary
- approval expires
- high-risk action executes only after valid approval
- receipt chain proves proposal, approval, and execution

