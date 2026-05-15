# First Version

The first completed BoardClaw version should ship three reference profiles:

- Raspberry Pi 5 for IoT
- Orange Pi 5 Plus for Smart Home
- NVIDIA Jetson Orin Nano for Robotics

This is the right first version because it proves three very different board
classes without exploding the project scope.

## First-Version Product Promise

A technical user should be able to install BoardClaw on a supported reference
board and:

- run a local model route
- talk through CLI or local web/API
- inspect board profile and connected devices
- use read-only hardware tools safely
- enable selected write tools behind policy and approval
- use MQTT, Home Assistant, or ROS 2 depending on the reference board
- see model route, tool decisions, denials, approvals, and results in the event
  history
- recover from provider, tool, or hardware failure without corrupting state

## First-Version Channels

| Channel | Used by | Purpose |
|---|---|---|
| Local Web UI | Human | dashboard, approvals, board status |
| CLI | Developer/admin | setup, diagnostics, testing |
| MQTT | IoT | sensor events, device messages |
| Home Assistant | Smart Home | state query, service call proposal |
| ROS 2 bridge | Robotics | bounded robot command proposal |
| Local HTTP API | Integrations | stable internal/external API |

Telegram, Matrix, and mobile/PWA approval can come later. They should not be
required for the first completed version.

## Shared First-Version Stack

```text
boardclawd
  Rust daemon
  normalized event API
  session and device memory
  board profile registry
  model provider router
  tool registry
  policy engine
  approval state
  event log

boardclaw-hwd
  least-privilege hardware/control helper
  GPIO/I2C/SPI/UART/camera adapters
  simulation backend for CI

providers
  Ollama first
  llama.cpp second
  board-specific acceleration only after benchmarks

storage
  SQLite for sessions, devices, automations, approvals, and events
  optional JSONL export for debugging
```

## Raspberry Pi 5: IoT Reference

Primary purpose:

- sensor gateway
- GPIO and bus inspection
- MQTT automation
- camera snapshot
- approval-gated relay/PWM demo

Minimum development hardware:

- Raspberry Pi 5 8GB
- official power supply
- active cooling
- Raspberry Pi OS 64-bit
- microSD or NVMe
- simple GPIO/I2C fixture

Best development hardware:

- Raspberry Pi 5 16GB or Compute Module 5
- NVMe storage
- camera module
- known-good relay/sensor test board
- optional supported accelerator

First model route:

- Ollama or llama.cpp
- 1B-4B instruct model
- selected by measured latency, memory, license, and tool-call quality

Required tools:

- `system.info`
- `board.profile`
- `gpio.read`
- `i2c.scan`
- `i2c.read`
- `uart.read`
- `camera.capture`
- `mqtt.publish`
- `mqtt.subscribe`
- `gpio.write` behind approval
- `pwm.set` behind approval

Expected demo:

```text
"Check the greenhouse sensors and tell me if watering is needed."
  -> read MQTT/state history
  -> read safe sensor tools
  -> summarize
  -> if watering is recommended, create a relay proposal
  -> require approval before relay write
```

## Orange Pi 5 Plus: Smart-Home Reference

Primary purpose:

- local smart-home gateway
- Home Assistant control surface
- MQTT event loop
- automation history
- RK3588 family baseline

Recommended hardware:

- Orange Pi 5 Plus with enough RAM for local services
- stable power supply
- active cooling
- NVMe or reliable eMMC/microSD
- Home Assistant or MQTT test environment

First model route:

- Ollama CPU route first
- 2B-7B quantized model depending on RAM and benchmark result
- RKLLM/RKNN only after conversion and benchmark success

Required tools:

- `mqtt.publish`
- `mqtt.subscribe`
- `home_assistant.get_state`
- `home_assistant.call_service` behind policy
- `automation.rule_create`
- `automation.rule_dry_run`
- `device_graph.read`
- `device_graph.update`

Expected demo:

```text
"If the garage door is open after 22:00 and no person is detected, turn on
the entry light and notify me."
  -> subscribe/read sensor state
  -> create deterministic automation rule
  -> dry-run the rule
  -> call Home Assistant service only when policy allows it
  -> log every event and action
```

## Jetson Orin Nano: Robotics Reference

Primary purpose:

- robot companion computer
- camera and vision-language workflows
- ROS 2 bridge
- bounded motion proposals
- local acceleration route

Recommended hardware:

- Jetson Orin Nano developer kit
- JetPack-supported OS
- active cooling
- camera
- ROS 2 test environment
- simulated robot or safe low-power test platform

First model route:

- NVIDIA-friendly provider path for accelerated local inference
- Ollama or llama.cpp fallback for text tasks
- camera pipeline or local VLM when benchmarked

Required tools:

- `camera.capture`
- `vision.describe`
- `ros2.topic_list`
- `ros2.echo_once`
- `ros2.publish_bounded`
- `ros2.service_call_safe`
- `robot.safety_state`
- `robot.motion_propose`

Expected demo:

```text
"Move the rover forward 30 cm if the path is clear."
  -> read safety state and camera/sensor data
  -> model proposes bounded motion
  -> policy checks speed, distance, timeout, stop condition, and safety state
  -> ROS 2 or microcontroller executes
  -> watchdog stops on timeout
```

The model must never own the direct motor loop.

## First-Version Exit Criteria

BoardClaw first version is complete when:

- all three profiles boot in simulation
- each reference board has real hardware validation for its core use case
- local model routing is visible and benchmarked
- write tools are denied without policy approval
- event logs contain tool inputs, outputs, denials, approvals, and route data
- CI can run without hardware through simulation
- release docs clearly say which features are supported, experimental, or not
  supported

## Final V1 Definition

```text
BoardClaw v1 =
  3 board profiles
  + local model router
  + web/CLI/MQTT/Home Assistant/ROS 2/local HTTP channels
  + typed tool registry
  + read-first safety policy
  + approval-gated writes
  + audit log
  + benchmark results per board
```
