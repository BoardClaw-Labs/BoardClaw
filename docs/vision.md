# Vision and Goals

BoardClaw is a local-first AI control plane for Linux-capable edge boards.

It is designed for people who work close to hardware: IoT builders, smart-home
operators, robotics developers, embedded engineers, labs, makers, and small
teams that need private local automation instead of a cloud-only assistant.

## Product Definition

```text
BoardClaw
  = board-aware agent runtime
  + local model provider routing
  + message channels
  + typed hardware/control tools
  + memory and device graph
  + safety policy, approval, audit, and receipt metadata
```

The final product is BoardClaw: one control runtime with many board profiles.
The first version is deliberately focused on three reference boards so the core
can become excellent before the support matrix grows.

## First Version Goal

The first version proves three practical domains:

- Raspberry Pi 5 for IoT sensors, buses, GPIO, camera, and MQTT.
- Orange Pi 5 Plus for smart-home gateway work, Home Assistant, automation
  history, storage, and RK3588 family reuse.
- NVIDIA Jetson Orin Nano for robotics, camera-heavy workflows, local vision,
  and ROS 2 safety boundaries.

This is the smallest set that proves BoardClaw is not tied to one board, one
model runtime, or one use case.

## Final Version Goal

The final version should let a user talk to an edge board and safely ask it to:

- inspect sensors, buses, cameras, OS state, and connected services
- summarize device and automation state
- control relays, lights, pumps, motors, lab fixtures, and robots through typed
  tools rather than raw model authority
- coordinate with MQTT, Home Assistant, ROS 2, local HTTP APIs, and operator
  channels
- remember useful device context and explain what changed
- require approval for high-risk actions
- record enough audit and receipt metadata to prove what was proposed,
  approved, executed, denied, or redacted
- degrade gracefully when a model, accelerator, network, or board feature is
  unavailable

## Design Principles

### Local First

Common work should run locally. LAN or cloud fallback can exist, but it must be
explicit, visible, policy-controlled, and benchmarked.

### Board Profiles, Not Forks

Every supported board gets a profile:

- board detection and OS assumptions
- GPIO/I2C/SPI/UART/CAN availability
- camera and accelerator support
- provider routes and model classes
- pin maps, thermal limits, power warnings, and safe defaults
- feature tier and benchmark status

The agent core stays the same.

### Typed Control

The model never receives raw authority over the machine. It can propose typed
actions. BoardClaw validates those actions, checks policy, and then invokes a
narrow tool.

### Real Safety

Robots, relays, heaters, pumps, batteries, locks, and lab equipment are
physical systems. Prompting is not safety. BoardClaw must support read-only
defaults, confirmations, policy gates, dry-run, timeouts, watchdogs, hardware
interlocks, event logs, and emergency stop paths outside the model.

### Honest Capability

BoardClaw should not pretend every board can run every model. It should show the
selected route, explain when a board is a host versus a satellite, and keep
accelerator claims tied to measured benchmarks.

### Rust Core, Swappable Models

The trusted runtime core should be Rust. Models are runtime assets selected by
board, task, license, memory, local availability, and measured behavior.

## Non-Goals

- hard real-time control loops inside an LLM
- one universal NPU runtime for all vendors
- direct arbitrary shell access for the model
- hidden cloud dependency for local automation
- replacing hardware-level safety components
- supporting every board at once before the reference boards are real

## Completion Bar

BoardClaw is not complete when it can chat. It is complete when it can safely
operate in hardware-heavy environments:

- local model route works and is visible
- board profile detects real capabilities
- read-only tools inspect real state
- write tools are policy-gated and auditable
- risky actions can wait for approval
- logs and receipts are useful for debugging and trust
- failures are recoverable
- a new board family can be added without rewriting the core
