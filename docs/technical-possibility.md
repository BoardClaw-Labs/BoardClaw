# Technical Possibility

BoardClaw is technically possible.

The successful version is not "one model runtime that runs perfectly on every
board." The successful version is a common agent and safety core with
board-specific profiles for inference, hardware access, and operating limits.

## Feasibility Verdict

| Area | Verdict | Reason |
|---|---|---|
| Raspberry Pi first product | High | Strong OS, GPIO, camera, community, Ollama, and Hailo-Ollama paths |
| Common agent core | High | Channels, sessions, routing, providers, and tools are normal software architecture |
| Local small-model operation | High | Small quantized models can handle command parsing, summaries, and simple tool use |
| Local large-model operation on every board | Low | RAM, accelerator support, and thermal limits differ too much |
| Multi-board hardware tools | Medium | Linux exposes GPIO/I2C/SPI/UART, but pin maps and permissions differ |
| Multi-board NPU acceleration | Medium to Low | Vendor stacks are fragmented and model conversion is often the hard part |
| Robotics support | Medium | Good as an orchestrator; unsafe as a hard real-time controller |
| SecuClaw with Uniclaw and mobile approval | Medium to High | Integration is clean if BoardClaw emits proposal/tool metadata from the start |
| Rust core implementation | High | Rust fits the daemon, hardware, policy, and Uniclaw integration requirements |
| One-language-only product | Medium | The core can be Rust, but UI and some vendor SDK adapters may need TypeScript/Python |

## What Is Straightforward

These parts are conventional and should be built early:

- daemon service
- CLI and local web API
- local Ollama provider
- OpenAI-compatible provider interface
- SQLite sessions and device graph
- tool registry with JSON schemas
- read-only hardware tools
- MQTT/Home Assistant integration
- board profile files
- event logs

## What Is Possible But Needs Care

These are feasible, but they need careful engineering and testing:

- GPIO/I2C/SPI/UART writes
- camera integration across boards
- ROS 2 bridge
- per-board pin maps
- safe shell/file tools
- local model routing
- Hailo-Ollama provider
- RK3588 NPU provider
- Jetson TensorRT provider
- mobile approvals
- Uniclaw receipt integration

## What Is Hard

These should not be promised in the first release:

- one universal accelerator abstraction
- high-quality local reasoning on low-RAM boards
- automatic safe robotics autonomy
- arbitrary model conversion to every NPU
- guaranteed real-time behavior on Linux SBCs
- unattended destructive repair actions

## Product Strategy

The right path:

```text
1. Build RasClaw as the first BoardClaw profile.
2. Implement the core in Rust.
3. Keep provider, tool, memory, policy, and board-profile boundaries generic.
4. Select models by board/task/provider instead of hardcoding Gemma.
5. Add RK3588 family after Raspberry Pi is stable.
6. Add Jetson for robotics/VLM.
7. Add lower-power boards as satellites or control nodes, not LLM-heavy hosts.
8. Add Uniclaw as an optional proof sidecar once the tool event model is stable.
```

## Board Roles

Not every board should do the same job.

| Role | Best boards |
|---|---|
| First complete product | Raspberry Pi 5 / CM5 |
| Robotics and VLM | Jetson Orin Nano |
| ARM gateway with NPU experiments | Orange Pi 5 Plus, Radxa ROCK 5B+, Banana Pi BPI-M7 |
| Local server and storage gateway | ODROID-H3+ |
| Embedded control and vision | BeagleBone AI-64 |
| Low-cost IoT satellite | Libre Computer Le Potato |
| Industrial IoT profile | ASUS Tinker Board 2S |

## Local Model Reality

BoardClaw can use Gemma, Qwen, Phi, Llama, DeepSeek, SmolLM, or other suitable
local models. The model is a replaceable runtime asset, not the identity of the
project.

Small local models are enough for:

- intent classification
- choosing tools
- summarizing sensor state
- generating simple automation rules
- explaining device status
- writing short reports

They are not always enough for:

- broad engineering reasoning
- complex coding
- advanced robotics planning
- long document analysis
- high-stakes safety decisions

BoardClaw should make the model route visible:

```text
local-small -> local-accelerated -> LAN model -> cloud fallback
```

Only `local-small` should be mandatory.

Recommended first model policy:

- Raspberry Pi MVP: start with a 1B-4B instruct model through Ollama.
- Raspberry Pi AI HAT+ 2: add Hailo-supported models through Hailo-Ollama.
- Jetson: prefer models that work well with NVIDIA's current acceleration path.
- RK3588: start CPU/Ollama, then add RKLLM/RKNN models only after conversion is
  proven.
- Low-end boards: use tiny local command models or a LAN model hub.

## Minimum Viable Technical Slice

The smallest useful BoardClaw is:

```text
Raspberry Pi 5
  + boardclawd
  + Ollama local model
  + CLI/local web channel
  + SQLite memory
  + board profile detection
  + read-only GPIO/I2C/UART/camera tools
  + approval-gated GPIO write
  + MQTT publish/subscribe
```

This slice proves the architecture without depending on NPU support, mobile
approval, ROS 2, or Uniclaw.

## Decision

Proceed.

Build the first version as RasClaw on Raspberry Pi, but name and structure every
internal subsystem as BoardClaw. That avoids the most common failure: building a
Pi-specific prototype that later needs to be rewritten to support real boards.
