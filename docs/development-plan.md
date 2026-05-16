# Development Plan

BoardClaw development is CI-first, benchmark-gated, and profile-based.

The most important rule:

```text
BoardClaw is one project.
Boards are profiles.
Features are optional.
Use cases are reference deployments.
```

Raspberry Pi 5, Orange Pi 5 Plus, and Jetson Orin Nano are not separate
products. They are the first three reference profiles that prove one shared
BoardClaw core can serve IoT, Smart Home, and Robotics.

## Product Shape

```text
BoardClaw Core
  profiles:
    raspberry_pi_5       -> IoT / sensors / GPIO / MQTT
    orange_pi_5_plus     -> Smart Home / gateway / local services
    jetson_orin_nano     -> Robotics / vision / ROS 2 / VLM

  shared systems:
    channel registry
    model router
    provider interface
    tool registry
    policy engine
    memory and device graph
    event/audit log
    benchmark runner
```

## Deployment Modes

BoardClaw must support both simple and advanced users.

### Single-Feature Mode

User enables only one feature area.

```text
Smart Home only:
  board: Orange Pi 5 Plus
  features: smart_home, mqtt, home_assistant, dashboard
  disabled: robotics, GPIO write, ROS 2
```

This is valid BoardClaw. The user should not need Jetson or Raspberry Pi if
they only want Smart Home.

### Single-Device Multi-Feature Mode

One strong device runs multiple feature areas.

```text
One Jetson Orin Nano:
  features: smart_home, robotics, camera, ROS 2, local model
```

This is valid when the board has enough CPU, memory, storage, network, and
physical access to devices.

### Hub And Node Mode

One board acts as the hub. Other boards act as specialized nodes.

```text
Orange Pi 5 Plus:
  BoardClaw hub, dashboard, memory, model router, Home Assistant, MQTT

Raspberry Pi 5:
  IoT node, sensors, GPIO, camera, MQTT publisher

Jetson Orin Nano:
  robotics node, camera/perception, ROS 2 bridge
```

This is the strongest final v1 demo because it proves BoardClaw can coordinate
multiple physical contexts without becoming three separate projects.

## Feature Selection

Every install should be explicit about enabled capabilities.

Example configuration shape:

```yaml
board:
  id: orange_pi_5_plus
  role: smart_home_gateway

features:
  iot: false
  smart_home: true
  robotics: false
  approval: true

channels:
  local_web: true
  cli: true
  http_api: true
  mqtt: true
  home_assistant: true
  ros2: false

tools:
  default_mode: read_only
  writes_require_approval: true
  raw_shell: false

model:
  provider: ollama
  fallback_remote: false
```

Maintainers should treat feature flags as safety controls, not only convenience.
Disabled features must hide their tools from the model.

## Required Development Gates

No phase is complete until all applicable gates pass.

### CI Gate

Runs on GitHub for every pull request.

Required:

- formatting
- linting
- unit tests
- benchmark spec validation
- phase-00 benchmark comparison
- no forbidden old project names

### Simulation Gate

Runs without physical hardware.

Required:

- fake board profile
- fake model provider
- fake hardware tool backend
- deterministic events
- policy decisions
- benchmark result file

### Real Environment Gate

Runs on the real target board.

Required:

- board id and OS recorded
- provider/model recorded
- enabled features recorded
- hardware fixture recorded
- pass/fail metrics saved
- known limitations written in docs

### Maintainer Review Gate

Required for physical-control changes:

- safety review
- tool permission review
- benchmark review
- docs update
- release/support matrix update if capability changed

## Official Phase Plan

```text
Phase 00: Core Contracts
Phase 01: Raspberry Pi 5 IoT Profile
Phase 02: Orange Pi 5 Plus Smart Home Profile
Phase 03: Jetson Orin Nano Robotics Profile
Phase 04: Shared Safety Layer
Phase 05: Cross-Board Demo
Phase 06: More Board Profiles
Phase 07: Provider Acceleration And Model Telemetry
Phase 08: Approval And Receipts
Phase 09: Release Hardening
```

## Phase 00: Core Contracts

Goal: define the shared BoardClaw foundation before hardware work begins.

Implementation steps:

1. Create Rust workspace.
2. Create `boardclaw-core`.
3. Define board families, profile roles, provider kinds, model classes, channel
   kinds, and tool risk levels.
4. Define benchmark expectation format.
5. Add GitHub Actions CI.
6. Add product docs and risk docs.
7. Save phase-00 baseline result.

Expected code result:

- repository builds
- `boardclaw-core` tests pass
- CI runs on PR and push
- benchmark scripts validate expected files

Expected real environment result:

- no real hardware required
- contributors can clone the repo and run all phase-00 checks on a normal Linux
  development machine

Saved artifacts:

- `benchmarks/expected/phase-00-ci.json`
- `benchmarks/results/phase-00-ci-baseline.json`

Done when:

- CI is green
- docs explain the product without private context
- first-version profiles and channels are represented in core contracts

## Phase 01: Raspberry Pi 5 IoT Profile

Goal: prove BoardClaw can run on a real edge board, inspect hardware safely, and
use a local model for IoT tasks.

Enabled feature mode:

```text
BoardClaw IoT mode
  board: Raspberry Pi 5
  features: iot, mqtt, camera, local_web, cli, http_api
  model: Ollama first, llama.cpp later
```

Implementation steps:

1. Add Raspberry Pi board detection.
2. Add local provider health check for Ollama.
3. Add SQLite session, device graph, and event history.
4. Add CLI and local HTTP API.
5. Add MQTT publish/subscribe.
6. Add read-only tools: `system.info`, `board.profile`, `gpio.read`,
   `i2c.scan`, `i2c.read`, `uart.read`, `camera.capture`.
7. Add simulated write proposal for `gpio.write` and `pwm.set`.
8. Deny real write execution unless policy approval is present.
9. Add hardware fixture docs for GPIO/I2C/UART/camera testing.

CI/simulation test result:

- simulated Raspberry Pi profile loads
- local provider can be mocked
- MQTT loopback passes
- read-only tools return structured data
- unsafe GPIO write is denied without approval
- event log records request, model route, tool proposal, policy decision, and
  result

Real environment test result:

- Raspberry Pi 5 is detected as `raspberry_pi_5`
- local Ollama health check succeeds
- I2C scan works on a known safe fixture
- GPIO read works on a known input pin
- camera capture succeeds or returns a clear timeout/error
- MQTT publish/subscribe loopback meets benchmark threshold
- approved write is tested only on a safe LED/relay fixture with low-risk power

Saved artifacts:

- `benchmarks/expected/phase-01-raspberry-pi-iot.json`
- `benchmarks/results/phase-01-raspberry-pi-iot-rpi5-YYYYMMDD.json`
- hardware notes in `docs/board-profiles.md`

Done when:

- Raspberry Pi can run BoardClaw locally
- user can ask a local model about sensor/device state
- BoardClaw can propose, deny, approve, and log a simple GPIO action safely

## Phase 02: Orange Pi 5 Plus Smart Home Profile

Goal: prove BoardClaw can act as a local Smart Home gateway without becoming
Raspberry Pi-specific.

Enabled feature mode:

```text
BoardClaw Smart Home mode
  board: Orange Pi 5 Plus
  features: smart_home, mqtt, home_assistant, dashboard, device_graph
  model: Ollama or llama.cpp CPU route first
```

Implementation steps:

1. Add Orange Pi 5 Plus profile.
2. Add reusable RK3588 family profile base.
3. Add Home Assistant state query adapter.
4. Add Home Assistant service-call proposal tool.
5. Add MQTT broker workflow tests.
6. Add device graph and automation history.
7. Add local dashboard views for devices, events, and pending actions.
8. Keep RKNN/RKLLM disabled by default until benchmarks prove the route.

CI/simulation test result:

- simulated Orange Pi profile loads
- Home Assistant adapter can query fake state
- service-call proposal is policy checked
- MQTT event creates deterministic automation input
- device graph survives restart
- unsupported RK acceleration is not selected as default

Real environment test result:

- Orange Pi 5 Plus is detected as `orange_pi_5_plus`
- CPU local model route works
- MQTT broker loopback passes
- Home Assistant test instance can be queried
- service-call proposal is logged and approval-gated
- device graph persists after daemon restart
- thermal/power notes are recorded

Saved artifacts:

- `benchmarks/expected/phase-02-orange-pi-smart-home.json`
- `benchmarks/results/phase-02-orange-pi-smart-home-opi5plus-YYYYMMDD.json`
- RK3588 profile notes in `docs/board-profiles.md`

Done when:

- user can run BoardClaw as a Smart Home-only system
- BoardClaw can explain home state and propose safe service calls
- the RK3588 profile base is reusable by later Radxa/Banana Pi boards

## Phase 03: Jetson Orin Nano Robotics Profile

Goal: prove BoardClaw can support robotics and vision without letting an LLM
control motors directly.

Enabled feature mode:

```text
BoardClaw Robotics mode
  board: Jetson Orin Nano
  features: robotics, camera, ros2, perception, local_web
  model: text route first, NVIDIA/perception route when benchmarked
```

Implementation steps:

1. Add Jetson Orin Nano profile.
2. Add Jetson provider capability reporting.
3. Add camera capture/perception tool shape.
4. Add ROS 2 bridge in simulation first.
5. Add bounded movement proposal schema.
6. Add `robot.safety_state` input.
7. Require speed, distance, frame, timeout, stop condition, and safety state for
   movement proposals.
8. Deny direct motor-loop or raw actuator tools.

CI/simulation test result:

- simulated Jetson profile loads
- camera/perception tool can return fake scene data
- ROS 2 bounded publish works in simulation
- movement without safety metadata is denied
- direct model motor-loop control is impossible

Real environment test result:

- Jetson Orin Nano is detected as `jetson_orin_nano`
- camera capture succeeds or reports a clear driver/runtime error
- ROS 2 test topic publishes bounded command in simulation or safe test robot
- unsafe movement request is denied
- safe movement proposal is logged, bounded, and optionally executed only in a
  controlled low-speed test environment

Saved artifacts:

- `benchmarks/expected/phase-03-jetson-robotics.json`
- `benchmarks/results/phase-03-jetson-robotics-jetson-YYYYMMDD.json`
- robotics safety notes in `docs/risks.md`

Done when:

- user can run BoardClaw as a Robotics-only system
- BoardClaw can explain robot/camera state
- BoardClaw can propose bounded ROS 2 actions while controllers own real-time
  safety

## Phase 04: Shared Safety Layer

Goal: turn safety from local checks into one reusable policy system for all
profiles and feature modes.

Implementation steps:

1. Define policy input and policy decision types.
2. Add read-only default mode.
3. Add write approval requirement.
4. Add dry-run mode.
5. Add tool timeout rules.
6. Add allowlists for files, shell commands, devices, pins, topics, services,
   and ROS 2 actions.
7. Add audit events for request, proposal, denial, approval, execution, timeout,
   and error.
8. Add least-privilege `boardclaw-hwd` interface.

CI/simulation test result:

- same policy engine gates GPIO, Home Assistant service calls, and ROS 2 motion
- raw shell is unavailable by default
- unknown tools are denied
- unsafe writes are denied without approval
- approved simulated write executes once
- every write attempt emits an audit event

Real environment test result:

- Raspberry Pi GPIO write, Orange Pi Home Assistant service call, and Jetson ROS
  2 movement proposal all use the same policy decision model
- denied actions show human-readable reason
- approval-gated test action executes only after approval
- all results appear in event history

Saved artifacts:

- `benchmarks/expected/phase-04-shared-safety-layer.json`
- `benchmarks/results/phase-04-shared-safety-layer-YYYYMMDD.json`
- safety docs updated

Done when:

- feature modes can be enabled/disabled safely
- the model never sees tools disabled by profile or policy
- maintainers can audit why a physical action did or did not run

## Phase 05: Cross-Board Demo

Goal: prove BoardClaw is one multi-board control system.

Reference environment:

```text
Raspberry Pi 5:
  reads IoT sensors and publishes MQTT state

Orange Pi 5 Plus:
  receives MQTT, checks Home Assistant, owns dashboard and memory

Jetson Orin Nano:
  uses camera/perception and proposes robot inspection movement
```

Implementation steps:

1. Add node identity and registration.
2. Add cross-board event correlation id.
3. Add hub/node messaging through MQTT or local HTTP.
4. Add dashboard view for all nodes.
5. Add cross-board demo automation.
6. Add one audit trail across all three boards.

CI/simulation test result:

- three simulated profiles register with one hub
- Raspberry Pi event routes to Orange Pi automation
- Orange Pi can request Jetson perception summary
- incompatible tools are hidden per node
- one trace id links the full flow

Real environment test result:

- Raspberry Pi publishes real sensor event
- Orange Pi receives it and evaluates smart-home state
- Jetson captures or simulates perception for inspection
- BoardClaw dashboard shows all three board states
- final response explains what happened and what was denied or approved

Saved artifacts:

- `benchmarks/expected/phase-05-cross-board-demo.json`
- `benchmarks/results/phase-05-cross-board-demo-YYYYMMDD.json`
- demo wiring/topology notes

Done when:

- BoardClaw is visibly one platform across three physical boards
- the demo can be repeated by a maintainer from documentation
- every cross-board action is traceable

## Phase 06: More Board Profiles

Goal: expand only after the three-board triangle is stable.

Targets:

- Radxa ROCK 5B+
- Banana Pi BPI-M7
- ODROID-H3+
- BeagleBone AI-64
- Libre Computer Le Potato
- ASUS Tinker Board 2S

Implementation steps:

1. Add profile file for each board.
2. Classify each board as host, gateway, or satellite.
3. Reuse RK3588 family profile for Radxa and Banana Pi.
4. Keep low-end boards in tiny-model or LAN-model mode.
5. Default unknown boards to read-only.
6. Document unsupported features honestly.

CI/simulation test result:

- all profiles parse
- low-end boards do not default to large models
- unknown profile starts read-only
- family profile reuse is measured

Real environment test result:

- each board can at least report system/profile data when hardware is available
- no write tools are enabled until board-specific fixtures are validated
- support tier is recorded

Saved artifacts:

- `benchmarks/expected/phase-06-more-board-profiles.json`
- board-specific result files when tested
- support matrix update

Done when:

- new users can see which boards are real hosts, gateways, or satellites
- no board is marketed beyond its measured tier

## Phase 07: Provider Acceleration And Model Telemetry

Goal: make model routing measurable and honest.

Implementation steps:

1. Add provider capability reports.
2. Add model route telemetry.
3. Add memory, latency, and tool-call quality metrics.
4. Add CPU fallback route.
5. Add optional Raspberry Pi accelerator route when available.
6. Add RK route only as experimental until conversion is proven.
7. Add Jetson accelerated route benchmarks.

CI/simulation test result:

- provider route is recorded for every model call
- fallback is selected when preferred provider fails
- acceleration cannot become default without benchmark data

Real environment test result:

- model name, quantization, provider, memory, latency, and board are recorded
- CPU fallback works on each reference board
- acceleration route is marked experimental or supported based on result

Saved artifacts:

- `benchmarks/expected/phase-07-provider-acceleration.json`
- per-board/provider/model benchmark files

Done when:

- users can see why a model route was selected
- maintainers can change defaults based on data, not fashion

## Phase 08: Approval And Receipts

Goal: make high-risk actions pending, approvable, single-use, and verifiable.

Implementation steps:

1. Add action proposal object.
2. Add pending approval state.
3. Add local approval API and web approval surface.
4. Add single-use approval token.
5. Add expiration.
6. Add input/output hashes.
7. Add receipt metadata to event log.
8. Add optional mobile/PWA approval surface later in the phase.

CI/simulation test result:

- pending approval blocks execution
- expired approval is denied
- valid approval executes once
- proposal, approval, and execution metadata verify as a chain

Real environment test result:

- high-risk GPIO, Home Assistant service call, and ROS 2 movement proposal can
  all become pending
- user can approve or deny from local web
- approved action executes once and records receipt metadata

Saved artifacts:

- `benchmarks/expected/phase-08-approval-receipts.json`
- approval/receipt result file
- approval UX notes

Done when:

- BoardClaw can prove what was proposed, approved, executed, denied, or expired
- high-risk physical action cannot bypass approval state

## Phase 09: Release Hardening

Goal: make BoardClaw installable, understandable, and honest for open-source
users.

Implementation steps:

1. Add install scripts.
2. Add service files.
3. Add sample configs for each feature mode.
4. Add backup/restore docs.
5. Add threat model.
6. Add hardware fixture docs.
7. Add support matrix.
8. Add release checklist.
9. Add maintainer guide for CI, PRs, labels, and hardware reports.

CI/simulation test result:

- install scripts lint or dry-run
- configs validate
- support matrix is machine-readable
- release checklist blocks unsupported claims

Real environment test result:

- clean install works on Raspberry Pi 5, Orange Pi 5 Plus, and Jetson Orin Nano
- user can enable only one feature mode
- user can disable all write tools
- dashboard shows board, model route, enabled features, and recent events

Saved artifacts:

- `benchmarks/expected/phase-09-release-hardening.json`
- release candidate test reports
- support matrix

Done when:

- a new maintainer can reproduce v1 from docs
- a technical user can install BoardClaw without private help
- unsupported features are clearly marked experimental or unavailable

## Final BoardClaw V1 Result

BoardClaw v1 is complete when all of this is true:

- one device can run selected features when hardware is sufficient
- multiple devices can split work by role
- Raspberry Pi 5 proves IoT
- Orange Pi 5 Plus proves Smart Home gateway
- Jetson Orin Nano proves Robotics/vision
- all three share one core, one policy model, one event/audit model, and one
  benchmark workflow
- users can choose which features to enable
- dangerous tools are disabled or approval-gated by default
- model routes and hardware support are backed by saved benchmark results
