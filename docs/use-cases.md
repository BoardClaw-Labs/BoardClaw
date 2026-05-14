# Use Cases

BoardClaw focuses on IoT, smart automation, robotics, and embedded engineering.

## IoT

### Local Sensor Gateway

BoardClaw reads sensors through GPIO/I2C/SPI/UART and publishes normalized
events to MQTT.

Example:

```text
"What changed in the greenhouse overnight?"
```

BoardClaw can read temperature, humidity, soil moisture, relay state, and camera
snapshots, then summarize anomalies.

### Offline Monitoring Node

BoardClaw keeps working when internet access is down:

- local model answers basic questions
- local automations continue
- events queue for later sync
- remote fallback is skipped

### Device Inventory

BoardClaw can maintain a local device graph:

- sensor name
- bus/address
- physical location
- safe read commands
- unsafe write commands
- last-seen timestamp

## Smart Automation

### Home Assistant Copilot

BoardClaw can act as a local explanation and control layer for Home Assistant:

- summarize state
- inspect automation failures
- propose scene changes
- call services only after policy checks

High-risk service calls should require approval.

### MQTT Automation Brain

BoardClaw can subscribe to MQTT topics, infer useful state, and publish actions:

```text
sensors/garage/door -> open
sensors/garage/person -> false
time -> after 22:00
action -> lights/entry/set on
```

### Scheduled Maintenance

BoardClaw can run scheduled checks:

- battery health
- disk health
- temperature trends
- camera health
- offline devices
- relay state mismatch

## Robotics

### Robot Companion Computer

BoardClaw should be a companion brain, not the motor controller.

Good tasks:

- interpret operator commands
- generate mission plans
- explain robot state
- inspect camera/sensor data
- send bounded commands to ROS 2
- ask for confirmation before risky movement

Bad tasks:

- direct high-frequency motor loop
- bypassing emergency stop
- commanding movement without sensor validation

### ROS 2 Bridge

Initial ROS 2 bridge tools:

- `ros2.topic_list`
- `ros2.echo_once`
- `ros2.publish_bounded`
- `ros2.service_call_safe`
- `ros2.nav_goal_propose`

Motion commands should include:

- maximum speed
- maximum distance
- timeout
- frame of reference
- safety state
- stop condition

### Vision-Language Robotics

Jetson Orin Nano should become the strongest VLM robotics profile:

- camera capture
- object detection
- scene summary
- VLM question answering
- bounded ROS 2 action proposals

## Embedded Engineering

### Hardware Bring-Up Assistant

BoardClaw can help bring up a board:

- detect OS/kernel
- list I2C/SPI/UART devices
- identify common sensor addresses
- read safe device registers
- generate wiring notes

### Firmware and Bus Work

Potential tools:

- `serial.monitor`
- `serial.send_line`
- `openocd.status`
- `flash.propose`
- `i2c.scan`
- `spi.transfer`
- `logic_capture.import`

Firmware flashing should always require explicit approval.

### Lab Automation

BoardClaw can coordinate lab equipment through USB, serial, SCPI, Modbus, or
network APIs:

- power supply on/off
- relay matrix control
- measurement capture
- test logs
- report generation

Use Uniclaw receipts for tests where provenance matters.

