# BoardClaw Benchmarks

BoardClaw uses benchmark expectation files as release gates.

- `expected/`: committed thresholds for each development phase.
- `results/`: committed baseline results for meaningful milestones.

Temporary local or CI results should use `*.local.json` or `*.latest.json`.

The first version benchmark path is:

1. Raspberry Pi 5 IoT profile
2. Orange Pi 5 Plus smart-home profile
3. Jetson Orin Nano robotics profile
4. shared safety layer
5. cross-board demo
