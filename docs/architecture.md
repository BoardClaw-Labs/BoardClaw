# Architecture

BoardClaw has one agent runtime and many board profiles.

```text
channels
  web, mobile, CLI, Telegram, MQTT, Home Assistant, ROS 2
        |
        v
boardclawd
  identity, sessions, memory, routing, policy, orchestration
        |
        +--> provider layer
        |      local or remote model backends
        |
        +--> tool layer
        |      hardware, automation, files, shell-safe, network-safe
        |
        +--> board profile layer
        |      detection, pin maps, accelerators, cameras, thermal limits
        |
        +--> proof layer
               optional Uniclaw sidecar for approvals and receipts
```

## Runtime Components

### `boardclawd`

The long-running daemon owns:

- session state
- message routing
- model selection
- tool registration
- policy enforcement
- memory writes
- event/audit stream
- local web API

It should be implemented in Rust and run as a normal service with minimal
privileges. Hardware access can be delegated to a separate Rust helper daemon or
Linux groups instead of running the entire agent as root.

### Channels

Channels convert user or system events into normalized messages.

Initial channels:

- CLI for development
- local web UI
- local HTTP API
- MQTT for IoT events

Later channels:

- Telegram or Matrix
- Home Assistant integration
- ROS 2 bridge
- mobile app push/approval channel

### Provider Layer

BoardClaw talks to model backends through a provider interface:

```text
Provider.Chat(messages, tools, model, options) -> response + tool_calls
```

Provider adapters should include:

- Ollama
- llama.cpp server
- Hailo-Ollama
- RKLLM/RKNN bridge
- NVIDIA/TensorRT-LLM bridge
- OpenVINO bridge
- OpenAI-compatible remote fallback

BoardClaw should not hardcode one model family. The provider layer should expose
capabilities such as text, vision, audio, function/tool calling, context window,
memory requirement, accelerator support, and license metadata. The router then
chooses a suitable model for the board and task.

The provider layer must support local-first routing:

```text
simple/control task -> local small model
vision/robotics task -> board-specific VLM or vision pipeline
large reasoning task -> optional LAN/cloud fallback
```

### Tool Layer

Tools are the only way the model controls the board.

Every tool needs:

- name
- schema
- capability declaration
- read/write classification
- risk level
- board compatibility
- timeout
- dry-run behavior when possible
- audit event

Core tools:

- `gpio.read`
- `gpio.write`
- `pwm.set`
- `i2c.scan`
- `i2c.read`
- `i2c.write`
- `spi.transfer`
- `uart.read`
- `uart.write`
- `can.send`
- `camera.capture`
- `mqtt.publish`
- `home_assistant.call_service`
- `ros2.publish`
- `shell.safe_exec`
- `file.read_allowed`
- `file.write_allowed`

Dangerous tools must require policy approval by default.

### Hardware Tool Daemon

For safety and portability, BoardClaw should split model orchestration from
hardware control:

```text
boardclawd      normal service, no broad root
boardclaw-hwd   narrow hardware daemon, group or capability scoped
```

This keeps the LLM-facing runtime away from raw device authority.

The hardware daemon should also be Rust. Python SDKs can be wrapped by small
adapter processes when a board vendor requires Python, but the trusted interface
back to BoardClaw should stay narrow and typed.

### Board Profiles

A board profile is data plus small adapters:

```yaml
id: raspberry_pi_5
family: raspberry_pi
arch: arm64
preferred_os: raspberry_pi_os
providers:
  - ollama
  - hailo_ollama
buses:
  gpio: true
  i2c: true
  spi: true
  uart: true
  can: adapter_required
cameras:
  - libcamera
accelerators:
  - cpu
  - hailo_10h_optional
limits:
  require_active_cooling: true
  conservative_pwm_default: true
```

Profiles should be declarative first. Code should only appear where a board has
real API differences.

### Memory

Initial memory should be boring:

- SQLite for sessions, device graph, facts, and automations
- JSONL or append-only event stream for debugging
- summaries for long conversations
- optional vector index later

Memory should separate:

- user preferences
- board facts
- device inventory
- automation history
- safety incidents
- model/tool traces

### Routing

Routing answers three questions:

1. Which agent/profile handles this?
2. Which model/provider is appropriate?
3. Which tools are visible to the model?

Routing features:

- local-first by default
- model fallback only when explicitly enabled
- read-only mode for unknown boards
- high-risk tools hidden until the user enables them
- robot motion tools hidden unless safety profile is configured

### Proof Layer

The proof layer is optional in the first MVP but must fit the design.

BoardClaw should be able to send this to Uniclaw:

- action proposal
- requested capability
- estimated resource charge
- user/channel identity
- tool input hash
- tool output hash
- secret-use names
- redaction report

BoardClaw should never need to rewrite tools to add Uniclaw. The tool system
should already produce the right events.

## Process Boundaries

Recommended process layout:

```text
boardclawd
  orchestrates agent turns and providers

boardclaw-hwd
  controls hardware buses and exposes narrow local RPC

uniclaw-host
  optional receipt/approval sidecar

provider process
  ollama, llama.cpp, hailo-ollama, tensorrt server, rkllm bridge
```

This keeps failure domains clear.

## Real-Time Boundary

BoardClaw may plan robot actions, but it should not run the control loop.

```text
LLM / agent:       seconds, language, planning, uncertainty
BoardClaw tools:   tens to hundreds of milliseconds, validated commands
Controller:        microseconds to milliseconds, closed-loop control
Hardware:          physical limits and interlocks
```

For robotics, BoardClaw should speak to ROS 2, a microcontroller, or a dedicated
controller that owns real-time safety.
