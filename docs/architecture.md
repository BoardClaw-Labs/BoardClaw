# Architecture

BoardClaw has one agent runtime and many board profiles.

```text
channels
  CLI | web | HTTP API | MQTT | Home Assistant | ROS 2
        |
        v
event normalizer
  principal, channel, board id, request kind, device context, trace id
        |
        v
boardclawd
  sessions, memory, routing, policy, approval state, event log
        |
        +--> model providers
        |      Ollama | llama.cpp | Hailo route | RK route | NVIDIA route
        |      OpenVINO | explicit LAN/cloud fallback
        |
        +--> tool registry
        |      typed schemas, risk levels, compatibility, dry-run, timeouts
        |
        +--> board profile layer
        |      detection, pin maps, accelerators, cameras, OS notes, limits
        |
        +--> hardware/control helper
        |      least-privilege GPIO, buses, camera, shell-safe, files, ROS 2
        |
        +--> audit and receipt metadata
               proposal, approval, denial, execution, redaction, hashes
```

## Runtime Components

### `boardclawd`

The long-running daemon owns:

- channel ingestion
- session state
- device graph and memory
- board detection
- model selection
- tool visibility
- policy enforcement
- approval state
- event/audit stream
- local HTTP API

It should run as a normal service with minimal privileges. Hardware access
should be delegated to a narrow helper process or Linux groups instead of
running the entire agent as root.

### Channels

Channels convert user or system input into normalized BoardClaw events.

First channels:

- CLI for development and tests
- local HTTP API
- local web dashboard
- MQTT for IoT events

Reference-board channels:

- Raspberry Pi 5: CLI, HTTP, MQTT, camera/tool events
- Orange Pi 5 Plus: MQTT, Home Assistant, local web, storage/event history
- Jetson Orin Nano: ROS 2, camera/VLM, CLI, local web

Later channels:

- Telegram or Matrix operator chat
- mobile approval/PWA
- additional automation platforms

Channels never bypass policy. They create requests. BoardClaw decides what can
run.

### Provider Layer

BoardClaw talks to model backends through a provider interface:

```text
Provider.chat(messages, tools, model, options) -> response + tool_calls
Provider.health() -> capabilities + latency + model availability
Provider.telemetry() -> route, memory, timing, errors
```

Provider adapters should include:

- Ollama
- llama.cpp server
- Hailo-oriented route for supported Raspberry Pi accelerators
- RKLLM/RKNN bridge for RK3588 boards after CPU support is stable
- NVIDIA/TensorRT-oriented route for Jetson
- OpenVINO for x86/Intel experiments
- OpenAI-compatible LAN/cloud fallback when explicitly enabled

The provider layer must expose capability data:

- text, vision, audio, tool/function calling
- context window
- memory requirement
- accelerator requirement
- license metadata
- measured benchmark status
- offline availability

The router then chooses the smallest reliable model for the board and task.

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
- approval requirement
- simulation behavior for CI

Core tools:

- `system.info`
- `board.profile`
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
- `ros2.publish_bounded`
- `shell.safe_exec`
- `file.read_allowed`
- `file.write_allowed`

Dangerous tools must require policy approval by default.

### Hardware/Control Helper

BoardClaw should split model orchestration from device authority:

```text
boardclawd        normal user service, policy, routing, memory
boardclaw-hwd     narrow hardware helper, scoped permissions
vendor-adapter    optional process for Python-first SDKs
```

The trusted interface back to BoardClaw stays typed even when a vendor SDK
requires a Python bridge.

### Board Profiles

A profile is mostly declarative:

```yaml
id: orange_pi_5_plus
family: rk3588
reference_role: smart_home
arch: aarch64
providers:
  preferred:
    - ollama
  experimental:
    - rkllm
    - rknn
hardware:
  gpio: true
  i2c: true
  spi: true
  uart: true
  camera: board_specific
safety:
  default_mode: read_only
  writes_require_approval: true
  require_pinmap_confirmation: true
limits:
  thermal_monitoring: required
  npu_claim_requires_benchmark: true
```

Profiles should be data first. Code should appear only where a board has real
API differences.

### Memory

Initial memory should be boring and reliable:

- SQLite for sessions, device graph, facts, automations, approvals, and events
- append-only JSONL event stream for debugging
- short summaries for long sessions
- optional vector index later

Memory must separate:

- user preferences
- board facts
- device inventory
- automation history
- approval state
- safety incidents
- model/tool traces

### Routing

Routing answers three questions:

1. Which board profile owns this request?
2. Which model/provider is appropriate?
3. Which tools are visible to the model?

Routing rules:

- local-first by default
- model fallback only when explicitly enabled
- unknown boards start read-only
- high-risk tools hidden until policy enables them
- robotics motion tools hidden unless a safety profile is configured
- provider acceleration must have benchmark proof before becoming default

### Approval And Receipt Layer

The first version can use local approval state and event logs. The final version
should be able to attach stronger receipts without changing the tool system.

Metadata required for every tool proposal:

- action id
- tool name
- target board
- target device
- input hash
- requested capability
- risk level
- channel
- principal
- model route
- policy decision
- approval status
- output hash or error code
- redaction report

This makes later mobile approval and independent receipt verification a clean
extension instead of a rewrite.

## First-Version Flow

```text
User/MQTT/ROS event
  -> normalized BoardClaw event
  -> load session and board profile
  -> select local provider
  -> expose allowed tools
  -> model proposes answer/tool
  -> policy validates
  -> approval required or tool executes
  -> event/receipt metadata written
  -> response sent to channel
```

## Safety Boundary

For robotics and physical control:

```text
LLM: understand intent, summarize state, propose bounded action
BoardClaw: validate, route, log, request approval
Controller: enforce timing, interlocks, watchdogs, emergency stop
```

The LLM must not own hard real-time loops.
