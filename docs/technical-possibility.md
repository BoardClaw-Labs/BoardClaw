# Technical Possibility

BoardClaw is technically possible.

The successful version is not "one model runtime that runs perfectly on every
board." The successful version is a common agent, routing, tool, memory, and
safety core with board-specific profiles for inference, hardware access, and
operating limits.

## Feasibility Verdict

| Area | Verdict | Reason |
|---|---|---|
| Three-board first version | High | Raspberry Pi 5, Orange Pi 5 Plus, and Jetson Orin Nano each prove a distinct domain |
| Common agent core | High | Channels, sessions, routing, providers, and tools are normal software architecture |
| Local small-model operation | High | Small quantized models can handle command parsing, summaries, and simple tool use |
| Safe typed hardware tools | High with care | Linux exposes GPIO/I2C/SPI/UART, but permissions and pin maps must be profile-specific |
| Smart-home integration | High | MQTT and Home Assistant APIs are mature and testable |
| Robotics orchestration | Medium to High | Good as a planner/explainer with ROS 2 boundary; unsafe as a hard real-time controller |
| Local large-model operation on every board | Low | RAM, accelerator support, and thermals differ too much |
| Multi-board NPU acceleration | Medium to Low | Vendor stacks are fragmented and model conversion is often the hard part |
| Later mobile approval and receipts | Medium to High | Clean if BoardClaw emits proposal/tool metadata from the start |
| Rust core implementation | High | Rust fits daemon, hardware, policy, and safety boundaries |
| One-language-only product | Medium | Core can be Rust, but UI and some vendor SDK bridges may need TypeScript/Python |

## Is The Three-Board Focus Right?

Yes.

It makes BoardClaw stronger because:

- Raspberry Pi 5 validates normal IoT hardware control.
- Orange Pi 5 Plus validates smart-home gateway behavior and RK3588 reuse.
- Jetson Orin Nano validates robotics and vision boundaries.
- The three boards force the provider layer to be real from the beginning.
- The project avoids pretending one board or one model path is universal.

It would make BoardClaw weaker only if the implementation becomes three
separate apps. The solution is to build shared runtime interfaces first and make
each board a profile.

## What Is Straightforward

These parts are conventional and should be built early:

- daemon service
- CLI and local HTTP API
- local Ollama provider
- OpenAI-compatible provider interface
- SQLite sessions and device graph
- tool registry with schemas
- read-only hardware tools
- MQTT/Home Assistant integration
- board profile files
- event logs
- simulation backend for CI

## What Is Possible But Needs Care

These are feasible, but need careful engineering and testing:

- GPIO/I2C/SPI/UART writes
- camera integration across boards
- ROS 2 bridge
- per-board pin maps
- safe shell/file tools
- local model routing
- Raspberry Pi accelerator route
- RK3588 NPU route
- Jetson accelerated route
- later mobile/PWA approval
- durable receipt verification

## What Is Hard

These should not be promised in the first release:

- one universal accelerator abstraction
- high-quality local reasoning on low-RAM boards
- automatic safe robotics autonomy
- arbitrary model conversion to every NPU
- guaranteed real-time behavior on Linux SBCs
- unattended destructive repair actions
- perfect offline vision-language performance on every board

## Product Strategy

The right path:

```text
0. Define core contracts: Rust core, profile schema, provider interface,
   benchmark format, and safety policy vocabulary.
1. Complete Raspberry Pi 5 for IoT.
2. Complete Orange Pi 5 Plus for smart-home gateway workflows.
3. Complete Jetson Orin Nano for robotics and vision workflows.
4. Harden the shared safety layer across all three profiles.
5. Build the cross-board demo.
6. Add more board profiles only after the three-board triangle is stable.
7. Use benchmarks to decide acceleration and model defaults.
8. Add approval, receipt, and mobile verification after the tool metadata is stable.
```

## Board Roles

Not every board should do the same job.

| Role | Best boards |
|---|---|
| IoT reference | Raspberry Pi 5 / CM5 |
| Smart-home reference | Orange Pi 5 Plus |
| Robotics and VLM reference | Jetson Orin Nano |
| RK3588 expansion | Radxa ROCK 5B+, Banana Pi BPI-M7 |
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

Only `local-small` should be mandatory for the first version.

## Minimum Viable Technical Slice

The smallest useful BoardClaw implementation is:

```text
Rust daemon
  + simulation channel/provider/tool stack
  + policy gate
  + SQLite memory
  + Raspberry Pi 5 profile
  + local Ollama provider
  + read-only GPIO/I2C/UART/camera tools
  + approval-gated GPIO write
  + MQTT publish/subscribe
```

This slice proves the architecture without depending on NPU support, mobile
approval, or ROS 2. Orange Pi and Jetson then prove the architecture is truly
multi-board.

## Decision

Proceed.

Build BoardClaw as a shared Rust core with three deep reference profiles before
expanding the board matrix.
