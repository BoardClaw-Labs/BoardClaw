# Board Profiles

BoardClaw supports boards by adding profiles. A profile describes what is safe,
available, recommended, and benchmarked for that board.

The goal is one agent core with different board profiles and provider routes.

## Profile Tiers

| Tier | Meaning |
|---|---|
| Tier 0 | Documented only, no runtime support |
| Tier 1 | Boots, detection works, local or LAN provider route is visible |
| Tier 2 | GPIO/I2C/SPI/UART read-only tools validated |
| Tier 3 | Write tools, MQTT/Home Assistant, camera, and persistence validated |
| Tier 4 | Accelerator/provider profile validated with benchmark data |
| Tier 5 | Approval, receipts, mobile verification, and release hardening validated |

The first version should push three reference boards deep before adding many
shallow profiles.

## First-Version Board Matrix

| Board | Role | BoardClaw focus | Local model path | Main risk | Mitigation |
|---|---|---|---|---|---|
| Raspberry Pi 5 | IoT reference | sensors, 40-pin GPIO, MIPI camera/display, PCIe storage, MQTT, local web/API | Ollama or llama.cpp with 1B-4B instruct model | thermal, power, GPIO mistakes | active cooling, official power, pin maps, approval-gated writes |
| Orange Pi 5 Plus | Smart Home reference | Home Assistant, MQTT, dual 2.5G gateway I/O, storage/event history, RK3588 family | Ollama CPU first, RKLLM/RKNN experimental later | OS/kernel variance, NPU conversion | CPU baseline first, profile tiers, NPU benchmarks before default |
| NVIDIA Jetson Orin Nano | Robotics reference | camera/VLM, ROS 2, bounded robot commands, AI-powered robotics/vision | NVIDIA provider route, Ollama/llama.cpp fallback | real-time safety, JetPack complexity | ROS 2/MCU boundary, container notes, safety metadata gates |

## Final Board Matrix

| Board | Family | Final role | Model route | Support posture |
|---|---|---|---|---|
| Raspberry Pi 5 / CM5 | Raspberry Pi | IoT and maker automation host | Ollama, llama.cpp, optional Hailo-supported route | first-class reference |
| Orange Pi 5 Plus | RK3588 | smart-home/automation gateway | Ollama CPU first, RKLLM/RKNN experimental | first-class reference |
| NVIDIA Jetson Orin Nano | Jetson | robotics and local vision host | NVIDIA provider path, local VLM/text route | first-class reference |
| Radxa ROCK 5B+ | RK3588 | high-performance RK3588 node | reuse Orange Pi family route with board-specific pins | follow after Orange Pi |
| Banana Pi BPI-M7 | RK3588 | compact RK3588 automation node | reuse RK3588 family route | follow after Orange Pi |
| ODROID-H3+ | x86 | edge server, NAS, automation hub | Ollama/llama.cpp CPU, OpenVINO experiments | useful but not accelerator-first |
| BeagleBone AI-64 | TI TDA4VM | embedded control and vision | tiny local model or LAN model hub | strict hardware permissions |
| Libre Computer Le Potato | Amlogic S905X | low-cost IoT satellite | tiny local command model or LAN model | not marketed as heavy LLM host |
| ASUS Tinker Board 2S | RK3399 | industrial IoT-style node | tiny local model or LAN model | validate OS and FOTA story first |

## Raspberry Pi 5

Why it is the IoT reference:

- strongest community and documentation
- stable GPIO/camera ecosystem
- standard 40-pin header, MIPI camera/display, and PCIe expansion
- Raspberry Pi OS support
- production commitment until at least January 2036
- accessible test fixtures
- optional AI HAT-class acceleration
- many real IoT and automation examples

Recommended baseline:

- Raspberry Pi 5 8GB or 16GB
- official power supply
- active cooling
- NVMe storage when possible
- Raspberry Pi OS 64-bit
- known-good GPIO/I2C/SPI test fixtures

First use cases:

- MQTT sensor gateway
- camera and local automation node
- Home Assistant companion
- embedded workbench assistant
- approval-gated relay demo

## Orange Pi 5 Plus

Why it is the Smart Home reference:

- RK3588 CPU performance is strong for a gateway board
- the board class includes 6 TOPS NPU capability, but model conversion is the
  hard part
- high RAM options are useful for local services
- dual 2.5G Ethernet and storage options fit gateway deployments
- storage and networking are suitable for Home Assistant/MQTT/event history
- successful profile work can be reused by Radxa ROCK 5B+ and Banana Pi BPI-M7

Recommended baseline:

- CPU/Ollama route first
- RKLLM/RKNN marked experimental until conversion and benchmarks pass
- Home Assistant adapter
- MQTT automation loop
- storage/event persistence
- thermal/power telemetry

Do not block Orange Pi support on NPU acceleration. The first useful profile is
CPU inference plus reliable automation tools.

## NVIDIA Jetson Orin Nano

Why it is the Robotics reference:

- strongest camera and vision target in the first set
- NVIDIA acceleration ecosystem
- good fit for local VLM experiments
- natural ROS 2 companion board
- NVIDIA positions the developer kit for AI-powered robots, smart drones, and
  intelligent cameras

Recommended baseline:

- JetPack-aware detection
- NVIDIA provider route
- camera capture and vision metadata
- ROS 2 bounded publish/service tools
- watchdog and emergency-stop status inputs
- motion commands denied without safety metadata

The Jetson profile should not pretend to be a Raspberry Pi profile. It needs its
own provider strategy, container notes, camera path, and robotics safety model.

## RK3588 Family Reuse

Orange Pi 5 Plus, Radxa ROCK 5B+, and Banana Pi BPI-M7 should share a family
base:

```text
rk3588-family
  provider baseline: Ollama CPU
  experimental provider: RKLLM/RKNN
  shared risk: OS drift, NPU conversion, thermal behavior
  board-specific data: pins, cameras, storage, image notes
```

BoardClaw should measure how much code the second RK3588 board reuses. If reuse
is low, the profile abstraction is wrong.

## Low-Power And Special Boards

Some boards are excellent control nodes but weak local LLM hosts.

BoardClaw should label them honestly:

- host: can run the default local model route
- gateway: can run services and a small local model, may use LAN model hub
- satellite: good at control/sensors, not a heavy reasoning host

This avoids false promises and helps users choose the right hardware.

## Profile File Shape

Example:

```yaml
id: jetson_orin_nano
family: jetson
reference_role: robotics
arch: aarch64
status: tier_0
providers:
  preferred:
    - nvidia
  fallback:
    - ollama
    - llama_cpp
hardware:
  gpio: true
  i2c: true
  spi: true
  uart: true
  camera: nvidia_stack
  ros2: supported
safety:
  default_mode: read_only
  writes_require_approval: true
  robot_motion_requires_safety_metadata: true
  llm_direct_motor_loop: forbidden
benchmarks:
  provider_route_required: true
  motion_denial_required: true
```
