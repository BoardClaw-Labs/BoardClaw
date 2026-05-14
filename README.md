# BoardClaw

BoardClaw is a local-first AI agent runtime for single-board computers, edge
gateways, robots, IoT labs, and embedded engineering workbenches.

The final goal is not only "RasClaw on Raspberry Pi". RasClaw is the first
board profile inside BoardClaw. The internal design must support many boards
from day one while shipping the Raspberry Pi path first.

```text
BoardClaw
  = board profile + local model provider + message channels
  + hardware tools + memory/routing + safety/proof layer
```

## Mission

BoardClaw turns a Linux-capable board into a private local automation agent
that can understand operator intent, inspect device state, call typed tools,
and safely coordinate IoT, smart automation, robotics, and embedded workflows.

The model proposes actions. BoardClaw validates and executes those actions
through narrow tools. High-risk actions can later be anchored through Uniclaw
receipts and mobile verification.

## First Target

The first completed profile is:

```text
RasClaw
  = Raspberry Pi 5 / Compute Module 5 profile
  + suitable local small model through Ollama, llama.cpp, or Hailo-Ollama
  + GPIO/I2C/SPI/UART/camera/MQTT/Home Assistant tools
  + SQLite memory and local web/mobile control
```

Raspberry Pi is the best first target because it has the strongest community,
stable documentation, accessible GPIO and camera support, and a clear local AI
path through CPU inference, Ollama, and the Raspberry Pi AI HAT+ 2.

## Board Roadmap

BoardClaw should grow by board profiles, not by forks.

| Phase | Profile | Role |
|---|---|---|
| 1 | Raspberry Pi 5 / CM5 | First complete product path, RasClaw |
| 2 | Orange Pi 5 Plus | RK3588 gateway and NPU experimentation |
| 3 | NVIDIA Jetson Orin Nano | Robotics, VLM, CUDA/TensorRT acceleration |
| 4 | ODROID-H3+ | x86 edge gateway, NAS, lab server, CPU inference |
| 5 | Radxa ROCK 5B+ | RK3588 board with strong RAM/storage options |
| 6 | BeagleBone AI-64 | Embedded control, vision, industrial I/O |
| 7 | Libre Computer Le Potato | Low-cost IoT gateway, light automation |
| 8 | ASUS Tinker Board 2S | Industrial IoT and FOTA-oriented profile |
| 9 | Banana Pi BPI-M7 | Compact RK3588 automation/robotics node |

ODROID-H3+ appears twice in the planning list; this README keeps one profile.

## Core Architecture

```text
channels
  web UI, mobile, CLI, Telegram, MQTT, ROS 2, Home Assistant
        |
        v
boardclawd
  session memory, routing, policy, model selection, audit events
        |
        +--> provider adapter
        |      ollama, llama.cpp, Hailo-Ollama, RKLLM/RKNN, TensorRT-LLM,
        |      OpenVINO, remote fallback
        |
        +--> hardware tool daemon
        |      GPIO, PWM, I2C, SPI, UART, CAN, camera, USB, sensors,
        |      relays, motors, shell-safe, files-safe
        |
        +--> safety/proof sidecar
               Uniclaw receipts, approvals, budgets, mobile verification
```

The most important design rule:

> BoardClaw exposes one agent/control API, but each board has its own provider
> and hardware profile.

Do not pretend Jetson, Raspberry Pi, RK3588 boards, BeagleBone, and x86 ODROID
share one perfect inference stack. They do not. They can share the BoardClaw
agent core.

## Local Model Strategy

BoardClaw is model-agnostic. Gemma-class models are a good first family, but
the project should use whichever local model is best for the board, task,
runtime, license, and memory budget.

Use Ollama for the first Raspberry Pi MVP because it is simple, local,
scriptable, and exposes OpenAI-compatible endpoints. Keep the provider layer
generic so the runtime can later use:

- `ollama` for the easiest local model path
- `llama.cpp` for direct GGUF control and lower-level tuning
- `hailo-ollama` for Raspberry Pi AI HAT+ 2
- `tensorrt-llm` or NVIDIA containers for Jetson
- `rkllm` / `rknn` for RK3588-family boards
- `openvino` for x86/Intel-class gateways
- cloud or LAN fallback for tasks too large for the board

Default policy: local model first, remote fallback only when explicitly enabled.

Recommended model classes:

- 1B-4B local instruct models for Raspberry Pi, low-power RK3588 boards, and
  IoT control.
- 4B-9B local models for Jetson, RK3588 boards with enough RAM, and ODROID/x86.
- Vision-language models for camera-heavy Jetson and Raspberry Pi AI HAT+ 2
  profiles when the provider supports them.
- Larger LAN/cloud models only as optional fallback for complex reasoning.

## Implementation Language

The recommended core language is **Rust**.

Rust is the best fit for the final BoardClaw goal because BoardClaw is a
hardware-adjacent, safety-sensitive daemon that needs typed tool schemas,
least-privilege process boundaries, cross-platform Linux support, low memory
overhead, strong concurrency, and future Uniclaw integration. Uniclaw is already
Rust, so a Rust core can embed or sidecar the proof layer cleanly.

Recommended split:

- **Rust**: `boardclawd`, hardware daemon, provider adapters, tool registry,
  policy, memory, routing, Uniclaw adapter.
- **TypeScript**: web dashboard and optional mobile-facing UI.
- **Python**: optional board/vendor adapters only when a hardware SDK is
  Python-first. Keep Python behind a process boundary.
- **Go**: acceptable for quick prototypes and PicoClaw-style experiments, but
  not the preferred final core for BoardClaw's safety/proof-heavy design.

## What BoardClaw Is For

- IoT gateways that speak MQTT, Modbus, Zigbee bridges, BLE bridges, and local
  HTTP APIs.
- Smart automation where the agent can explain, schedule, verify, and trigger
  actions across sensors, relays, cameras, and Home Assistant.
- Robotics orchestration where the LLM plans and explains, while real-time motor
  control stays in ROS 2, a microcontroller, or a dedicated safety controller.
- Embedded engineering where the agent helps inspect buses, flash firmware,
  read logs, capture camera frames, and document hardware state.
- Offline-first private automation where network loss should degrade capability,
  not stop the system.

## What BoardClaw Is Not

- Not a hard real-time motor controller.
- Not a universal NPU abstraction that magically makes every model fast.
- Not an unattended root shell for an LLM.
- Not a cloud-only assistant.
- Not a replacement for limit switches, watchdogs, fuses, emergency stops, or
  real electrical safety.

## Safety Model

BoardClaw must be useful because it can control real things. It must be careful
for the same reason.

High-risk operations should be behind typed tools, policy, budgets, and optional
approval. Examples:

- GPIO writes that energize relays
- PWM outputs for motors/servos
- I2C/SPI/UART writes
- firmware flashing
- shell execution
- file writes outside an allowed workspace
- network calls to private control APIs
- robot movement, unlocking doors, pumps, heaters, charging circuits

For robotics, the LLM must not sit in the hard real-time loop. Use this split:

```text
LLM: understand intent, plan, ask questions, explain
BoardClaw: validate and translate intent into typed actions
Controller: run motors, PID, interlocks, emergency stop, watchdog
```

## Uniclaw / SecuClaw Path

Uniclaw is not required for the first BoardClaw MVP, but BoardClaw should be
designed so Uniclaw can plug in cleanly when its proof layer is ready.

Future SecuClaw profile:

```text
SecuClaw = BoardClaw + Uniclaw + mobile verification
```

Flow:

```text
model proposes action
  -> BoardClaw creates a typed proposal
  -> Uniclaw checks constitution and budget
  -> risky action becomes pending
  -> phone receives approval request
  -> user approves with passkey/biometric
  -> BoardClaw executes the tool
  -> Uniclaw records execution receipt
```

This gives BoardClaw a future path for auditability, shared trust, and safe
remote approvals without blocking the Raspberry Pi-first build.

## Documents

- [Vision and Goals](docs/vision.md)
- [Development Plan](docs/development-plan.md)
- [Benchmarking](docs/benchmarking.md)
- [Architecture](docs/architecture.md)
- [Technical Possibility](docs/technical-possibility.md)
- [Model and Language Strategy](docs/model-and-language.md)
- [Board Profiles](docs/board-profiles.md)
- [Raspberry Pi First Plan](docs/raspberry-pi-first.md)
- [Use Cases](docs/use-cases.md)
- [Risks and Mitigations](docs/risks.md)
- [Uniclaw and SecuClaw Integration](docs/uniclaw-secuclaw.md)
- [Roadmap](docs/roadmap.md)
- [References](docs/references.md)

## Success Criteria

BoardClaw is successful when:

- Raspberry Pi can run a local model and safely control real GPIO/I2C/SPI/UART
  devices through typed tools.
- A user can automate an IoT task without giving the model arbitrary shell or
  root access.
- The same agent core can boot on at least three board families with different
  provider backends.
- Robotics demos use real safety boundaries instead of prompt-only safety.
- Uniclaw can be added as a sidecar for approvals and receipts without rewriting
  the BoardClaw tool system.

## Status

Planning and architecture stage.

The repository now has a CI-first Rust workspace, benchmark expectation files,
and an initial `boardclaw-core` crate for project contracts. The first feature
milestone is Raspberry Pi support with local Ollama provider, SQLite memory,
web/CLI channel, and a minimal hardware tool daemon.
