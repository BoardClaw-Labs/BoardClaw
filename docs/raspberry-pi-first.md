# Raspberry Pi First Plan

Raspberry Pi is the first complete BoardClaw target. This profile is called
RasClaw, but it must be implemented inside BoardClaw abstractions.

## Recommended MVP Hardware

Minimum useful:

- Raspberry Pi 5 8GB
- official power supply
- active cooling
- microSD or NVMe storage
- Raspberry Pi OS 64-bit

Best development target:

- Raspberry Pi 5 16GB or Compute Module 5
- NVMe storage
- active cooling
- camera module
- known-good GPIO/I2C/SPI test fixtures
- optional AI HAT+ 2 for local GenAI acceleration

## MVP Software Stack

```text
boardclawd
  Rust daemon, local HTTP API, CLI channel

provider
  Ollama on localhost first

model
  suitable 1B-4B local instruct model

memory
  SQLite sessions, device graph, automations, event log

hardware
  gpio.read, gpio.write, i2c.scan, i2c.read, uart.read, camera.capture

safety
  read-only default, write approvals, allowlist, dry-run, audit log
```

## Why Ollama First

Ollama is the fastest path to a working local model:

- easy install
- local HTTP API
- OpenAI-compatible endpoint
- model management
- enough community knowledge

BoardClaw should not depend on Ollama forever. The provider interface should
allow llama.cpp, Hailo-Ollama, and other local servers.

The Raspberry Pi profile should not require one exact model. Gemma 4 E2B/E4B
are good candidates, but the default should be chosen by measured latency,
memory, tool-call behavior, license, and availability on the target provider.

## First Tool Set

### Read-only tools

- `system.info`
- `board.profile`
- `gpio.read`
- `i2c.scan`
- `i2c.read`
- `uart.read`
- `camera.capture`
- `mqtt.subscribe`
- `home_assistant.get_state`

### Write tools

- `gpio.write`
- `pwm.set`
- `i2c.write`
- `uart.write`
- `mqtt.publish`
- `home_assistant.call_service`
- `file.write_allowed`
- `shell.safe_exec`

Write tools should start disabled or approval-required.

## First Demos

### IoT Demo

"Check the room sensors and tell me if the greenhouse needs water."

Flow:

```text
read MQTT sensors
read recent automation history
model summarizes
if watering is recommended, propose relay action
approval required before relay write
```

### Smart Automation Demo

"If the garage camera sees the door open after 22:00, notify me and turn on the
entry light."

Flow:

```text
camera.capture
local vision or lightweight VLM
create automation rule
publish to Home Assistant or MQTT
log event
```

### Robotics Demo

"Move the rover forward 30 cm if the path is clear."

Flow:

```text
camera/sensor check
model proposes motion
BoardClaw validates speed/distance/safety zone
ROS 2 or microcontroller executes motion
watchdog stops on timeout
```

No direct motor PWM from the LLM.

### Embedded Engineering Demo

"Scan the I2C bus, identify connected sensors, and create a wiring report."

Flow:

```text
i2c.scan
match known addresses
read safe identity registers
write markdown report
```

## Raspberry Pi Profile Milestones

### Milestone R1: Local Chat

- daemon boots
- CLI channel works
- Ollama provider works
- model can answer basic local prompts

### Milestone R2: Board Detection

- detect Raspberry Pi model
- detect OS/kernel
- detect GPIO availability
- detect camera stack
- report thermal/power warnings

### Milestone R3: Read-only Hardware

- GPIO read
- I2C scan/read
- UART read
- camera capture
- event log

### Milestone R4: Safe Writes

- GPIO write behind approval
- PWM behind approval
- I2C/UART writes behind approval
- dry-run mode
- tool timeout and rollback notes

### Milestone R5: Automation

- MQTT publish/subscribe
- Home Assistant state/tool calls
- scheduled tasks
- device graph in SQLite

### Milestone R6: Optional AI HAT+ 2

- Hailo runtime detection
- Hailo-Ollama provider adapter
- model availability check
- performance telemetry

### Milestone R7: Uniclaw Sidecar

- proposals sent to Uniclaw
- pending approvals
- tool execution receipts
- mobile approval prototype
