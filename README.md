# BoardClaw

BoardClaw is a local-first AI control runtime for edge boards, IoT gateways,
smart automation hubs, robots, and embedded engineering benches.

```text
BoardClaw
  = board profiles
  + local model routing
  + message channels
  + typed hardware/control tools
  + memory and event history
  + safety policy, approval, and audit receipts
```

The goal is not a chat demo on one board. The goal is a board-aware control
platform that can run private local models, understand device context, route
actions through narrow tools, and keep risky physical operations visible,
approved, and recoverable.

## First Version

The first complete BoardClaw version focuses on three reference boards:

| Reference board | Primary role | Why it is first | First local model route |
|---|---|---|---|
| Raspberry Pi 5 | IoT reference | strongest maker/IoT ecosystem, 40-pin GPIO, MIPI camera/display, PCIe, long production life | Ollama or llama.cpp with a 1B-4B instruct model |
| Orange Pi 5 Plus | Smart Home reference | RK3588, dual 2.5G Ethernet, high RAM options, gateway-style I/O, AIoT fit | Ollama CPU first, RKLLM/RKNN later only after benchmarks prove it |
| NVIDIA Jetson Orin Nano | Robotics reference | AI-powered robots, cameras, vision systems, CUDA/TensorRT ecosystem | NVIDIA provider path with Ollama/llama.cpp fallback |

This is the right narrowing. It makes BoardClaw stronger because each first
board proves a different product surface:

- Raspberry Pi 5 proves sensors, GPIO, MQTT, and normal IoT control.
- Orange Pi 5 Plus proves a more powerful smart-home gateway with RK3588 family
  reuse for later Radxa and Banana Pi profiles.
- Jetson Orin Nano proves robotics, vision, ROS 2 boundaries, and accelerated
  local inference.

Trying to finish every listed board at once would make BoardClaw weaker. The
three-board plan keeps the architecture broad while making validation deep.

## Roadmap

| Phase | Goal |
|---|---|
| 0 | Core contracts: Rust core, profile schema, provider interface, benchmark format, safety policy vocabulary |
| 1 | Raspberry Pi 5 IoT profile |
| 2 | Orange Pi 5 Plus Smart Home profile |
| 3 | Jetson Orin Nano Robotics profile |
| 4 | Shared safety layer |
| 5 | Cross-board demo |
| 6 | More board profiles |
| 7 | Provider acceleration and model telemetry |
| 8 | Approval and receipts |
| 9 | Release hardening |

## Final Version

The final BoardClaw is a multi-board local AI control system with:

- board profiles for Raspberry Pi, RK3588 boards, Jetson, x86 gateways,
  BeagleBone-class embedded boards, and low-cost automation satellites
- swappable model providers for Ollama, llama.cpp, vendor acceleration stacks,
  OpenVINO, and explicitly enabled LAN/cloud fallback
- channels for CLI, local web, HTTP API, MQTT, Home Assistant, Telegram/Matrix,
  ROS 2, and mobile approval
- typed tools for GPIO, PWM, I2C, SPI, UART, CAN, camera, MQTT, Home Assistant,
  ROS 2, safe files, and safe shell commands
- local memory for sessions, device inventory, automation history, safety
  incidents, and model/tool traces
- policy gates, dry-run, timeouts, least-privilege hardware helpers, audit
  events, signed receipt hooks, and mobile verification for high-risk actions

## Completed Runtime Flow

```text
1. Channel receives a message or event.
   Local web UI, CLI, MQTT, Home Assistant, ROS 2, or local HTTP API.

2. BoardClaw normalizes the event.
   It records principal, channel, board id, device context, and request type.

3. Session and memory are loaded.
   SQLite provides device graph, known automations, user preferences, and recent
   safety history.

4. Board router selects the active profile.
   The profile declares available buses, cameras, accelerators, providers,
   pin maps, thermal limits, and default tool visibility.

5. Model router selects the provider.
   Local-small first, local-accelerated when proven, LAN/cloud fallback only
   when explicitly enabled.

6. Tool router exposes only allowed tools.
   Read-only tools are visible first. Write tools require profile support,
   policy checks, and sometimes approval.

7. Model proposes an answer or typed tool call.
   The model does not get raw shell, raw GPIO authority, or hidden secrets.

8. Policy engine validates the proposal.
   It checks risk, board profile, channel, budget, target device, dry-run mode,
   approval state, and timeout limits.

9. Hardware/control layer executes the tool.
   A narrow helper process touches GPIO, buses, cameras, ROS 2, or local APIs.

10. Audit event is written.
    Every tool call records input hash, output hash, risk, approval, result,
    model route, board id, and redaction report. Later versions can attach
    stronger receipt verification to the same event data.

11. Channel receives the final response.
    The user sees what happened, what was denied, what needs approval, and what
    to do next.
```

## Big Picture

```text
channels
  CLI | web | HTTP API | MQTT | Home Assistant | ROS 2
        |
        v
boardclawd
  session memory
  event normalization
  board router
  model router
  tool visibility
  policy and approval state
  audit/receipt metadata
        |
        +--> providers
        |      Ollama | llama.cpp | Hailo route | RK route | NVIDIA route
        |      OpenVINO | explicit LAN/cloud fallback
        |
        +--> hardware/control helper
        |      GPIO | PWM | I2C | SPI | UART | CAN | camera | files | shell-safe
        |      MQTT | Home Assistant | ROS 2
        |
        +--> local storage
               SQLite | append-only events | benchmark results | device graph
```

The most important rule: BoardClaw exposes one agent/control API, but each board
has its own capability profile and provider route.

## Model Strategy

BoardClaw is model-agnostic. Gemma-class models are good candidates, but the
default should always be chosen by measured quality, latency, memory use,
license, local availability, and tool-calling behavior.

Recommended defaults:

- Raspberry Pi 5: 1B-4B instruct model through Ollama or llama.cpp.
- Orange Pi 5 Plus: 2B-7B quantized model through Ollama first; RK acceleration
  becomes experimental only after conversion and benchmark success.
- Jetson Orin Nano: small/medium local text model plus vision pipeline or local
  VLM through NVIDIA-friendly runtimes.
- Low-cost boards later: tiny local command model or LAN model hub, with honest
  labeling as satellites rather than heavy local LLM hosts.

Local-first is the default. Remote fallback is a feature flag, not a hidden
dependency.

## Channel Strategy

BoardClaw should support channels by normalizing every input into one internal
event shape.

First version channels:

| Channel | Used by | Purpose |
|---|---|---|
| Local Web UI | Human | dashboard, approvals, board status |
| CLI | Developer/admin | setup, diagnostics, testing |
| MQTT | IoT | sensor events, device messages |
| Home Assistant | Smart Home | state query, service call proposal |
| ROS 2 bridge | Robotics | bounded robot command proposal |
| Local HTTP API | Integrations | stable internal/external API |

Later channels:

- Telegram or Matrix for operator messages
- mobile/PWA approval for high-risk actions

Channels do not execute tools directly. They create requests. BoardClaw routes,
checks, executes, logs, and replies.

## Implementation Language

The trusted core should be written in **Rust**.

Rust is the best fit because BoardClaw is hardware-adjacent, long-running,
resource-sensitive, and safety-critical. The core needs typed tool schemas,
least-privilege process boundaries, predictable memory use, strong async
networking, and cross-platform Linux support.

Recommended split:

- Rust: daemon, hardware helper, provider interface, policy, tool registry,
  routing, memory, event log, local HTTP API
- TypeScript: web dashboard and mobile/PWA approval surface
- Python: narrow vendor SDK bridges only when a hardware or AI SDK requires it

## What BoardClaw Is For

- IoT gateways that speak MQTT, Modbus, Zigbee bridges, BLE bridges, and local
  HTTP APIs
- smart automation that can inspect state, explain actions, call Home Assistant
  services, and require approval for risky changes
- robotics orchestration where BoardClaw plans and explains while ROS 2,
  microcontrollers, watchdogs, and safety controllers own real-time motion
- embedded engineering workflows such as bus scans, serial monitoring, camera
  captures, wiring reports, firmware proposals, and lab automation
- offline-first private automation where network loss reduces capability but
  does not stop local control

## What BoardClaw Is Not

- not a hard real-time motor controller
- not a universal NPU abstraction that makes every model fast
- not an unattended root shell for an LLM
- not a cloud-only assistant
- not a replacement for limit switches, watchdogs, fuses, emergency stops, or
  physical safety design

## PicoClaw Lessons

PicoClaw already proved useful patterns that BoardClaw should reuse at a larger
hardware-control scale:

- keep channels, providers, tools, sessions, and runtime events as first-class
  modules
- use explicit routing instead of scattering provider decisions through code
- keep tool allowlists and sensitive-data filtering near the execution boundary
- make event logs useful enough to debug real devices
- test routing, session, provider, and tool behavior before adding integrations

BoardClaw applies those lessons to Linux boards, local models, physical safety,
and multi-board profiles.

## Documents

- [Vision and Goals](docs/vision.md)
- [Development Plan](docs/development-plan.md)
- [Benchmarking](docs/benchmarking.md)
- [Architecture](docs/architecture.md)
- [Technical Possibility](docs/technical-possibility.md)
- [Model and Language Strategy](docs/model-and-language.md)
- [Board Profiles](docs/board-profiles.md)
- [First Version](docs/first-version.md)
- [Use Cases](docs/use-cases.md)
- [Risks and Mitigations](docs/risks.md)
- [Approval and Receipts](docs/receipt-and-mobile-approval.md)
- [Roadmap](docs/roadmap.md)
- [References](docs/references.md)

## Status

Planning and bootstrap implementation.

The repository has a CI-first Rust workspace, benchmark expectation files, and
an initial `boardclaw-core` crate for project contracts. The next work is Phase
01: Raspberry Pi 5 IoT profile, while keeping the shared core, policy, provider,
tool, and event contracts generic for all three reference profiles.
