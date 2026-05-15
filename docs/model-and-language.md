# Model and Language Strategy

BoardClaw should be model-agnostic and Rust-first.

The model can change. The board can change. The trusted control architecture
should not change.

## Model Decision

Do not define BoardClaw as one board plus one model family.

Define it as:

```text
BoardClaw = board-aware local agent runtime with swappable model providers
```

Gemma-class models can be strong edge candidates, but BoardClaw should also be
able to use suitable Qwen, Phi, Llama, DeepSeek, SmolLM, or future small models
when they are better for the board, license, task, and provider.

## Model Selection Policy

The router should choose models by capability, not brand.

Selection inputs:

- board profile
- role: IoT, Smart Home, Robotics, gateway, satellite, embedded
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
high-risk action           -> model proposal + policy + approval, never model-only
```

## First Model Matrix

| Board/profile | First model path | Notes |
|---|---|---|
| Raspberry Pi 5 / CM5 | Ollama or llama.cpp with 1B-4B instruct model | choose by latency, memory, license, and tool-call quality |
| Raspberry Pi with supported accelerator | supported accelerated route | only models compiled and benchmarked for the accelerator |
| Orange Pi 5 Plus | Ollama CPU first, RKLLM/RKNN later | smart-home gateway should not depend on NPU conversion |
| Jetson Orin Nano | NVIDIA-friendly provider path, fallback text route | strongest robotics/VLM target |
| ODROID-H3+ | Ollama or llama.cpp CPU | good gateway/server profile, no dedicated NPU |
| BeagleBone AI-64 | tiny local model or LAN model hub | embedded/vision/control board, not heavy LLM host |
| Le Potato / Tinker Board 2S | tiny local model or LAN model hub | low-cost automation satellite profile |

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

- Raspberry Pi 5 IoT reference
- basic smart-home automation
- structured reports
- simple embedded debugging
- local-first assistant behavior

### Medium Local Models

Approximate range: 7B-9B parameters.

Use for:

- Jetson Orin Nano
- Orange Pi 5 Plus and other RK3588 boards with enough RAM
- ODROID/x86 gateways
- better coding and reasoning

### Large Local Or LAN Models

Approximate range: 14B+.

Use for:

- engineering reasoning
- code generation
- complex diagnosis
- planning with long context

These should be optional fallback, not the base requirement.

## Provider Priority

Initial provider priority:

1. Ollama for easiest local model bring-up.
2. llama.cpp for direct GGUF control and tighter embedded deployments.
3. Board-specific Raspberry Pi acceleration route when benchmarked.
4. NVIDIA/TensorRT-oriented route for Jetson.
5. RKLLM/RKNN path for RK3588 boards after CPU support works.
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
- hallucinated-tool denial rate
- thermal throttling notes
- power notes

No model should become a default only because it is fashionable.

## Programming Language Decision

Use **Rust** for the BoardClaw core.

Rust is the most suitable language for successful BoardClaw because the final
product is hardware-adjacent, safety-sensitive, local-first, and built around
typed control boundaries.

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
- installable service binaries

## Why Rust

Rust gives BoardClaw:

- memory safety without a garbage collector
- low resource overhead for SBCs
- strong types for dangerous tool boundaries
- good async/network support
- good cross-compilation story for Linux targets
- strong process-boundary and systems-programming fit
- better long-term safety posture than C/C++
- tighter runtime footprint than Node/Python for the daemon

## Language Split

| Area | Language | Why |
|---|---|---|
| Core daemon | Rust | safety, performance, typed policies |
| Hardware helper | Rust | least-privilege device control and stable binaries |
| Provider adapters | Rust first | shared routing and telemetry |
| Web dashboard | TypeScript | best browser UI ecosystem |
| Mobile approval UI | TypeScript/PWA first | works across devices before native apps |
| Vendor SDK bridge | Python only when needed | many board/AI SDKs expose Python-first tools |
| Experiments | Python or Go allowed | prototypes are useful, final core stays Rust |

## Why Not Go As The Final Core

Go is excellent for lightweight gateways and would be fast for prototypes. It
has simple cross-compilation, good networking, and easy concurrency.

For BoardClaw's final goal, Rust is a better core choice because BoardClaw needs
strong type boundaries around dangerous hardware actions, low-level systems
control, predictable binaries, and a security posture suitable for robotics and
embedded control.

Go can still be useful for:

- experiments
- companion CLIs
- simple bridge services
- comparison prototypes

## Why Not Python As The Final Core

Python has the best hardware-learning ecosystem and many AI SDKs expose Python
examples first. It is valuable for adapters and prototypes.

It should not be the trusted core because packaging, long-running service
stability, dependency supply chain, resource usage, and type boundaries are all
harder to control on small boards.

## Final Recommendation

Build BoardClaw as:

```text
Rust core
  + TypeScript web/mobile UI
  + optional Python vendor adapters
  + swappable local model providers
```

This gives the project the best chance to become a serious IoT, smart-home,
robotics, and embedded engineering platform rather than a single-board demo.
