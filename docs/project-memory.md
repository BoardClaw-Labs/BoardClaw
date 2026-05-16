# BoardClaw Project Memory

This file preserves the current BoardClaw direction for future context resets,
contributors, and maintainers.

## Core Principle

```text
BoardClaw is one project.
Boards are profiles.
Features are optional.
Use cases are reference deployments.
```

Raspberry Pi 5, Orange Pi 5 Plus, and Jetson Orin Nano are not separate
products. They are the first three reference profiles that prove one shared
BoardClaw core can serve IoT, Smart Home, and Robotics.

## First-Version Shape

```text
BoardClaw Core
  + Raspberry Pi 5 profile      -> IoT / sensors / GPIO / MQTT
  + Orange Pi 5 Plus profile    -> Smart Home / gateway / local services
  + Jetson Orin Nano profile    -> Robotics / vision / ROS 2 / VLM
```

## User Choice

BoardClaw must let users enable only the features they need.

Valid deployments:

- IoT-only on Raspberry Pi 5.
- Smart Home-only on Orange Pi 5 Plus.
- Robotics-only on Jetson Orin Nano.
- one strong device running multiple feature modes.
- hub and node mode where specialized boards split work.

Disabled features must hide their tools from the model and from normal control
flows.

## Development Gates

Every phase must pass:

- CI gate
- simulation gate
- real environment gate when hardware support is claimed
- maintainer review gate for physical-control changes

Do not claim hardware support from CI-only evidence.

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

## Final V1 Result

BoardClaw v1 is complete when:

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
