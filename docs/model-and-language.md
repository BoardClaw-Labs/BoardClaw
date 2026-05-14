# Model and Language Strategy

BoardClaw should be model-agnostic and Rust-first.

The model can change. The board can change. The trusted control architecture
should not change.

## Model Decision

Do not define BoardClaw as "Raspberry Pi + Gemma only."

Define it as:

```text
BoardClaw = board-aware local agent runtime with swappable model providers
```

Gemma 4 E2B/E4B is a strong first family for edge work, but BoardClaw should
also be able to use suitable Qwen, Phi, Llama, DeepSeek, SmolLM, or future small
models when they are better for the board and task.

## Model Selection Policy

The router should choose models by capability, not brand.

Selection inputs:

- board profile
- available RAM/VRAM/NPU memory
- provider backend
- text/vision/audio requirement
- tool-calling quality
- context length
- latency target
- privacy requirement
- license and redistribution terms
- offline availability
- measured benchmark on that board

Recommended routing:

```text
simple device query        -> smallest local instruct model
tool selection/control     -> local model proven with function/tool calls
camera/robot scene task    -> local VLM if available, otherwise vision pipeline + text model
complex engineering task   -> larger local/LAN model if enabled
high-stakes action         -> model proposal + policy + approval, never model-only
```

## First Model Matrix

| Board/profile | First model path | Notes |
|---|---|---|
| Raspberry Pi 5 / CM5 | Ollama with a 1B-4B instruct model | Start simple; Gemma 4 E2B/E4B are good candidates, not mandatory |
| Raspberry Pi + AI HAT+ 2 | Hailo-Ollama supported model | Use only models compiled/supported by the Hailo stack |
| Orange Pi 5 Plus / Radxa ROCK 5B+ / Banana Pi BPI-M7 | Ollama CPU first, RKLLM/RKNN later | Do not block support on NPU conversion |
| Jetson Orin Nano | NVIDIA-accelerated provider path | Strongest robotics/VLM target |
| ODROID-H3+ | Ollama or llama.cpp CPU | Good gateway/server profile, no dedicated NPU |
| BeagleBone AI-64 | tiny local model or LAN model hub | Treat as embedded/vision/control board, not LLM-heavy host |
| Le Potato / Tinker Board 2S | tiny local model or LAN model hub | Low-cost automation satellite profile |

## Model Classes

### Tiny Local Models

Approximate range: under 2B parameters.

Use for:

- command parsing
- intent classification
- simple sensor summaries
- choosing among safe tools
- offline fallback

### Small Local Models

Approximate range: 2B-4B parameters.

Use for:

- Raspberry Pi MVP
- IoT automation
- structured reports
- simple embedded debugging
- local-first assistant behavior

### Medium Local Models

Approximate range: 7B-9B parameters.

Use for:

- Jetson
- RK3588 boards with enough RAM
- ODROID/x86 gateways
- better coding and reasoning

### Large Local or LAN Models

Approximate range: 14B+.

Use for:

- engineering reasoning
- code generation
- complex diagnosis
- planning with long context

These should be optional fallback, not the base requirement.

## Provider Priority

Initial provider priority:

1. Ollama for Raspberry Pi MVP.
2. llama.cpp for direct GGUF control and tighter embedded deployments.
3. Hailo-Ollama for Raspberry Pi AI HAT+ 2.
4. NVIDIA/TensorRT path for Jetson.
5. RKLLM/RKNN path for RK3588 boards.
6. OpenVINO path for x86/Intel experiments.
7. OpenAI-compatible LAN/cloud fallback when explicitly enabled.

## Benchmark Requirement

Every supported model/profile pair should record:

- model name and version
- quantization
- provider
- board
- memory use
- cold start time
- time to first token
- decode tokens per second
- tool-call success rate
- hallucinated-tool rate
- thermal throttling notes
- power notes

No model should become a default only because it is fashionable.

## Programming Language Decision

Use **Rust** for the BoardClaw core.

Rust is the most suitable language for successful BoardClaw because the final
product is hardware-adjacent, safety-sensitive, local-first, and eventually
proof-integrated through Uniclaw.

Rust should own:

- `boardclawd`
- `boardclaw-hwd`
- provider interface
- tool registry
- hardware tool schemas
- policy and approvals
- memory and event log
- routing
- local HTTP API
- Uniclaw adapter
- installable service binaries

## Why Rust

Rust gives BoardClaw:

- memory safety without a garbage collector
- low resource overhead for SBCs
- strong types for dangerous tool boundaries
- good async/network support
- good cross-compilation story for Linux targets
- strong process-boundary and systems-programming fit
- direct compatibility with Uniclaw's Rust ecosystem
- better long-term safety posture than C/C++
- tighter runtime footprint than Node/Python for the daemon

## Language Split

| Area | Language | Why |
|---|---|---|
| Core daemon | Rust | safety, performance, typed policies, Uniclaw fit |
| Hardware daemon | Rust | least-privilege device control and stable binaries |
| Provider adapters | Rust first | shared routing and telemetry |
| Web dashboard | TypeScript | best browser UI ecosystem |
| Mobile approval UI | TypeScript or native later | ship web/PWA first, native later if needed |
| Vendor SDK bridge | Python only when needed | many board/AI SDKs expose Python-first tools |
| Experiments | Python or Go allowed | prototypes are fine, final core stays Rust |

## Why Not Go as the Final Core

Go is excellent for a PicoClaw-style lightweight gateway and would be faster for
some prototypes. It has simple cross-compilation, good networking, and easy
concurrency.

For BoardClaw's final goal, Rust is a better core choice because BoardClaw needs
stronger type boundaries around dangerous hardware actions, closer alignment
with Uniclaw, lower-level systems control, and a security posture suitable for
robotics and embedded control.

Go can still be useful for:

- experiments
- companion CLIs
- simple bridge services
- comparing against PicoClaw patterns

## Why Not Python as the Final Core

Python has the best hardware-learning ecosystem and many AI SDKs expose Python
examples first. It is valuable for adapters and prototypes.

It should not be the trusted core because packaging, long-running service
stability, dependency supply chain, resource usage, and type boundaries are all
harder to control on small boards.

## Final Recommendation

Build BoardClaw as:

```text
Rust core
  + TypeScript UI
  + optional Python vendor adapters
  + swappable local model providers
```

This gives the project the best chance to become a serious IoT, automation,
robotics, and embedded engineering platform rather than a Raspberry Pi demo.

