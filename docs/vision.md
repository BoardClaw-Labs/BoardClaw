# Vision and Goals

BoardClaw is a local-first AI control plane for Linux-capable boards.

It is designed for people who work close to hardware: IoT builders, automation
engineers, robotics developers, embedded engineers, makers, labs, and small
teams that need private local control instead of a cloud-only assistant.

## Product Definition

```text
BoardClaw
  = board-aware agent runtime
  + local model provider
  + message channels
  + hardware control tools
  + memory and routing
  + safety and proof integration
```

RasClaw is the first complete BoardClaw profile:

```text
RasClaw = BoardClaw profile for Raspberry Pi
```

The final product is BoardClaw, not a Raspberry Pi-only fork.

## Primary Goal

BoardClaw should let a user talk to an edge device and safely ask it to:

- inspect sensors and buses
- summarize device state
- control relays, lights, pumps, motors, and lab fixtures through safe tools
- capture and interpret camera frames
- schedule automation
- coordinate with Home Assistant, MQTT, ROS 2, and local APIs
- remember useful device context
- explain what it did
- prove or approve sensitive actions when Uniclaw is connected

## Design Principles

### Local First

The system should work without cloud inference for the common path. Cloud or LAN
fallbacks are allowed, but they must be explicit and visible.

### Board Profiles, Not Forks

Every supported board gets a profile:

- how to detect the board
- which OS/kernel assumptions are safe
- which GPIO/I2C/SPI/UART/CAN devices are available
- which provider backends are supported
- which camera and accelerator stacks are stable
- which warnings and thermal/power limits apply

The agent core stays the same.

### Typed Control

The model never receives raw authority over the machine. It asks for typed
actions. BoardClaw validates those actions, checks policy, and then invokes
narrow tools.

### Real Safety

Robots, relays, heaters, pumps, and batteries are physical systems. Prompting is
not safety. BoardClaw must support confirmations, policy gates, watchdogs,
hardware interlocks, read-only defaults, dry-run mode, and emergency stops.

### Useful Without Being Magical

BoardClaw should not promise that every board can run every model. It should
make capability visible and pick the best route for each board.

### Rust Core, Swappable Models

BoardClaw should use Rust for the trusted runtime core and treat models as
swappable provider assets. The goal is not to worship one model family. The goal
is to pick the smallest reliable local model for the board and task.

## Non-Goals

- Hard real-time control loops inside the LLM.
- One universal NPU runtime for all vendors.
- Direct arbitrary shell access for the model.
- Cloud dependency for basic local automation.
- Replacing hardware-level safety components.
- Supporting every SBC at once before the first board is excellent.

## Completion Bar

BoardClaw is not complete when it can chat. It is complete when it can safely
operate in hardware-heavy environments:

- local model works
- board profile detects real capabilities
- tools can control real buses and devices
- risky tools require policy or approval
- logs and state are useful for debugging
- user can recover from failure
- a second board family can be added without rewriting the core
