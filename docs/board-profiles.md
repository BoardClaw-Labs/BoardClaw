# Board Profiles

BoardClaw supports boards by adding profiles. A profile describes what is safe,
available, and recommended for that board.

The goal is one agent core with different board profiles and provider routes.

## Profile Tiers

| Tier | Meaning |
|---|---|
| Tier 0 | Documented only, no runtime support |
| Tier 1 | Boots, local provider works, read-only tools work |
| Tier 2 | GPIO/I2C/SPI/UART tools validated |
| Tier 3 | Camera, MQTT/Home Assistant, and storage profile validated |
| Tier 4 | Accelerator/provider profile validated |
| Tier 5 | Safety profile, Uniclaw receipts, and mobile approval validated |

The Raspberry Pi profile should reach Tier 3 before adding many boards. Jetson
and RK3588 boards can then validate the provider abstraction.

## Board Matrix

| Board | Family | BoardClaw role | Local model path | Risk |
|---|---|---|---|---|
| Raspberry Pi 5 / CM5 | Raspberry Pi | First complete profile, RasClaw | Ollama CPU, llama.cpp, Hailo-Ollama with AI HAT+ 2 | Thermal, power, small-model quality |
| Orange Pi 5 Plus | RK3588 | High-performance ARM automation gateway | Ollama CPU first, RKLLM/RKNN later | Vendor NPU tooling and kernel variance |
| NVIDIA Jetson Orin Nano | Jetson | Robotics/VLM profile | NVIDIA containers, TensorRT-LLM, Ollama as fallback | JetPack complexity, memory pressure |
| ODROID-H3+ | x86 | Edge gateway, NAS, lab server | Ollama/llama.cpp CPU, OpenVINO experiments | Discontinued board, no dedicated NPU |
| Radxa ROCK 5B+ | RK3588 | RK3588 flagship profile | Ollama CPU first, RKLLM/RKNN later | NPU conversion and OS drift |
| BeagleBone AI-64 | TI TDA4VM | Embedded control and vision | Tiny local model or remote/LAN model | 4GB RAM, DSP/NPU toolchain complexity |
| Libre Computer Le Potato | Amlogic S905X | Low-cost IoT gateway | Tiny local model only, remote/LAN fallback | Limited RAM/CPU, no modern NPU |
| ASUS Tinker Board 2S | RK3399 | Industrial IoT / FOTA profile | Tiny local model, remote/LAN fallback | Older SoC, limited model capacity |
| Banana Pi BPI-M7 | RK3588 | Compact RK3588 node | Ollama CPU first, RKLLM/RKNN later | Board support maturity |

## Raspberry Pi 5 / CM5

Why first:

- strongest community
- stable GPIO/camera ecosystem
- Raspberry Pi OS support
- official AI HAT+ 2 path
- many real IoT/robotics examples

Recommended baseline:

- Raspberry Pi 5 8GB or 16GB
- NVMe storage if possible
- active cooling
- official power supply
- local Ollama provider first
- AI HAT+ 2 optional for GenAI acceleration

Use cases:

- Home Assistant local copilot
- MQTT sensor gateway
- camera and smart automation node
- ROS 2 companion computer
- embedded workbench assistant

## Orange Pi 5 Plus

Orange Pi 5 Plus uses Rockchip RK3588 with a 6 TOPS NPU and up to 32GB RAM
options. It is attractive for edge gateway work because of CPU performance,
storage, display, and dual Ethernet options.

BoardClaw should support it through an `rk3588` family profile:

- CPU/Ollama first
- RKNN/RKLLM later
- profile-specific GPIO/pin maps
- thermal and power warnings
- board image compatibility notes

Do not block Orange Pi support on NPU acceleration. The first useful profile can
be CPU inference plus hardware tools.

## NVIDIA Jetson Orin Nano

Jetson Orin Nano is the strongest robotics and VLM target in the list.

Use it for:

- camera-heavy robots
- object detection plus language control
- ROS 2 integration
- local VLM experiments
- TensorRT-accelerated inference

The Jetson profile should not try to look like Raspberry Pi. It needs its own
provider profile, container strategy, camera strategy, and ROS 2 defaults.

## ODROID-H3+

ODROID-H3+ is an x86 edge gateway board. It is especially useful for:

- local automation server
- MQTT broker plus BoardClaw
- NAS/logging node
- Home Assistant host
- CPU inference with larger RAM than many SBCs

Because it has no modern dedicated NPU, treat it as:

- reliable x86 local server
- good llama.cpp/Ollama CPU host
- possible OpenVINO test target

It is listed as discontinued by Hardkernel, so BoardClaw should support it as a
requested profile but avoid making it the long-term x86 flagship.

## Radxa ROCK 5B+

Radxa ROCK 5B+ is another RK3588 board, with up to 32GB LPDDR5 and NVMe/eMMC
options. It should share most provider work with Orange Pi 5 Plus and Banana Pi
BPI-M7 under the `rk3588` family.

The right abstraction:

```text
rk3588-family provider profile
  + board-specific pins, cameras, boot quirks, package notes
```

## BeagleBone AI-64

BeagleBone AI-64 is not the best LLM host. It is a strong embedded control and
vision board.

Use it for:

- industrial I/O
- deterministic-ish embedded experiments
- camera/vision accelerators
- cape ecosystem
- robotics sensor/control companion

The model route should be conservative:

- tiny local model for intent classification or command parsing
- LAN model hub for heavier reasoning
- strict hardware tool permissions

## Libre Computer Le Potato

Le Potato should be a lightweight IoT profile:

- MQTT gateway
- small automation node
- sensor reader
- Home Assistant satellite

It should not be marketed as a serious local LLM board. Use tiny models or a LAN
fallback. Keep tools light and reliable.

## ASUS Tinker Board 2S

Tinker Board 2S is useful for industrial IoT-style deployments because ASUS
positions it with management/FOTA tooling.

Treat it as:

- IoT device profile
- local gateway with small model or remote fallback
- stable deployment target once OS support is proven

It is not a primary GenAI acceleration target.

## Banana Pi BPI-M7

Banana Pi BPI-M7 is another RK3588-family board with compact dimensions,
dual 2.5GbE, eMMC, camera/display, M.2, and GPIO.

It should share the RK3588 strategy:

- Ollama/llama.cpp first
- RKLLM/RKNN second
- board-specific IO validation

## Profile File Shape

Example:

```yaml
id: radxa_rock_5b_plus
family: rk3588
arch: arm64
status: tier_0
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
  can: adapter_required
  camera: board_specific
safety:
  default_mode: read_only
  writes_require_approval: true
  require_pinmap_confirmation: true
notes:
  - Validate OS image and kernel interfaces before enabling write tools.
```

