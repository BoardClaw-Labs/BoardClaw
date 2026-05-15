# Risks and Mitigations

BoardClaw controls physical systems. Its largest risks are safety, security,
overpromising local model capability, and spreading the project across too many
boards before the core is proven.

## Risk Summary

| Risk | Severity | Why it matters | Mitigation | Implementation phase |
|---|---:|---|---|---|
| Unsafe physical control | Critical | Relays, motors, heaters, pumps, chargers, and robots can cause damage | typed tools, approval, interlocks, watchdogs, safe defaults | Phase 04 |
| Arbitrary shell/root access | Critical | Full host compromise | no raw shell by default, narrow safe shell, least privilege | Phase 04 |
| LLM hallucination | High | Model may invent state, tools, or unsafe commands | structured tools, validation, denial for unknown tools | Phase 00-04 |
| Robotics real-time misuse | Critical | LLM latency is not deterministic | ROS 2/MCU boundary, bounded commands, emergency stop outside BoardClaw | Phase 03-04 |
| Vendor NPU fragmentation | High | Raspberry Pi, RK3588, Jetson, x86, and TI stacks differ | provider interface, CPU baseline first, acceleration after benchmark | Phase 07 |
| Thermal/power instability | High | SBCs throttle, crash, corrupt storage, or damage hardware | profile warnings, telemetry, active cooling checks | Phase 01-03 |
| GPIO/pin damage | High | Wrong voltage, pin mode, or wiring can destroy devices | pin maps, fixture tests, confirmations, read-only default | Phase 01 and Phase 04 |
| Privacy leak | High | Local sensors/cameras can expose private data | local-first, redaction, explicit fallback, audit events | Phase 00-04 |
| Weak provenance | High | Users cannot prove what was approved or executed | proposal, approval, execution records and receipt hooks | Phase 08 |
| Supply chain/model risk | Medium | Model/provider packages change and may be unsafe | pinned versions, checksums, profile matrix, visible model route | Phase 07-09 |
| Board support drift | Medium | kernels/images/SDKs change often | support tiers, validation scripts, honest release matrix | Phase 06-09 |
| Scope explosion | High | Too many boards too early makes every profile shallow | three reference boards first, family reuse later | Phase 01-06 |

## Physical Safety

Default posture:

- read-only tools enabled first
- write tools disabled until configured
- high-risk tools require approval
- dry-run available where meaningful
- timeouts on every tool
- emergency stop path outside BoardClaw
- hardware interlocks preferred over software-only safety

High-risk action examples:

- turn on a relay
- change PWM duty cycle
- send motor velocity
- unlock a door
- turn on a heater
- start a pump
- charge or discharge a battery
- write firmware
- write to unknown I2C/SPI/UART targets

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
- event log for every tool call
- approval and receipt metadata for high-risk actions

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
- keep deterministic rules outside the model
- route complex tasks to larger local/LAN/cloud models only when enabled
- record model route and benchmark result for every default

## NPU And Accelerator Risk

The hardest BoardClaw engineering problem is not chat. It is making local
inference portable across incompatible acceleration stacks.

Reality:

- Raspberry Pi acceleration routes use their own model formats and tooling.
- Jetson uses NVIDIA CUDA/TensorRT/JetPack.
- RK3588 boards use Rockchip tooling.
- ODROID-H3+ is x86 CPU-first.
- BeagleBone AI-64 has TI accelerator/DSP tooling.

Mitigation:

- provider interface owns backend differences
- board profile declares supported providers
- CPU provider is the first baseline when possible
- acceleration is optional per profile
- model benchmarks are stored per board and provider
- acceleration never becomes default without benchmark proof

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
- dry-run or simulator before real robot

## Project Risk

BoardClaw can fail by trying to support every board before three boards are
excellent.

Mitigation:

1. Define core contracts and safety vocabulary first.
2. Complete Raspberry Pi 5 as the IoT reference.
3. Complete Orange Pi 5 Plus as the Smart Home reference.
4. Complete Jetson Orin Nano as the Robotics reference.
5. Harden one shared safety layer across all three profiles.
6. Reuse family profiles for RK3588 and later boards.
7. Mark unsupported and experimental features honestly.
8. Use benchmark files as release gates, not after-the-fact reports.
