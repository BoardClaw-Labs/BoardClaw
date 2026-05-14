# Risks and Mitigations

BoardClaw controls physical systems. Its largest risks are safety, security,
and overpromising local model capability.

## Risk Summary

| Risk | Severity | Why it matters | Mitigation |
|---|---:|---|---|
| Unsafe physical control | Critical | Relays, motors, heaters, pumps, batteries can cause damage | Typed tools, approvals, interlocks, watchdogs, safe defaults |
| LLM hallucination | High | The model may invent device state or commands | Tools return structured facts; model cannot bypass validation |
| Arbitrary shell/root access | Critical | Full host compromise | No raw shell by default; narrow `shell.safe_exec`; least privilege |
| Vendor NPU fragmentation | High | Every board accelerator stack differs | Provider plugin architecture; CPU/Ollama baseline first |
| Thermal/power instability | High | SBCs throttle, crash, corrupt storage | profile warnings, telemetry, active cooling checks |
| GPIO/pin damage | High | Wrong voltage or pin mode can damage hardware | pin maps, confirmations, board-specific policies |
| Robotics real-time misuse | Critical | LLM latency is not deterministic | real-time loops outside BoardClaw; ROS 2/MCU boundary |
| Privacy leak | High | Local sensors/cameras can expose private data | local-first, redaction, explicit remote fallback, audit logs |
| Supply chain/model risk | Medium | model/provider packages may change | pinned versions, checksums, profile test matrix |
| Board support drift | Medium | kernels/images change often | board compatibility tiers and validation scripts |

## Physical Safety

Default posture:

- read-only tools enabled first
- write tools disabled until configured
- high-risk tools require approval
- dry-run available where meaningful
- timeouts on every tool
- emergency stop path outside BoardClaw

High-risk action examples:

- turn on a relay
- change PWM duty cycle
- send motor velocity
- unlock a door
- turn on a heater
- start a pump
- write firmware
- write to unknown I2C/SPI address

## Security

BoardClaw should assume the model is not trusted.

Controls:

- least-privilege daemon
- hardware helper with narrow permissions
- allowlisted file paths
- deny raw shell by default
- no secrets in model context
- local-only admin API by default
- token/passkey auth for remote access
- audit log for every tool call
- optional Uniclaw receipts for high-risk actions

## Local Model Limits

Small local models are useful but imperfect.

Expected strengths:

- command parsing
- summaries
- simple troubleshooting
- tool selection
- local automations
- structured reporting

Expected weaknesses:

- complex planning
- broad factual knowledge
- long multi-step debugging
- high-assurance safety reasoning
- very large context tasks

Mitigation:

- specialize prompts and tools
- use retrieval over local docs
- make uncertainty visible
- route complex tasks to larger local/LAN/cloud models only when enabled

## NPU and Accelerator Risk

The hardest BoardClaw engineering problem is not chat. It is making local
inference portable across incompatible acceleration stacks.

Reality:

- Raspberry Pi AI HAT+ 2 uses Hailo tooling.
- Jetson uses NVIDIA CUDA/TensorRT/JetPack.
- RK3588 boards use Rockchip tooling.
- ODROID-H3+ is x86 CPU-first.
- BeagleBone AI-64 has TI accelerator/DSP tooling.

Mitigation:

- provider interface owns model backend differences
- board profile declares supported providers
- CPU provider is always the baseline when possible
- acceleration is optional per profile
- model benchmarks are stored per board and provider

## Robotics Risk

Do not give the LLM a direct actuator loop.

Safe robotics pattern:

```text
user intent
  -> model plan
  -> BoardClaw validator
  -> bounded ROS 2 / MCU command
  -> controller enforces safety
  -> sensor feedback
  -> BoardClaw summarizes outcome
```

Required safeguards:

- maximum speed/distance
- timeout
- obstacle state
- emergency stop status
- watchdog heartbeat
- manual override

## Project Risk

BoardClaw can fail by trying to support every board before one board is great.

Mitigation:

1. Complete Raspberry Pi first.
2. Keep internals board-generic.
3. Add one board family at a time.
4. Reuse family profiles: `rk3588`, `jetson`, `x86`, `beaglebone`.
5. Mark unsupported features honestly.

