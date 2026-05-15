# Approval And Receipts

BoardClaw should support high-risk physical actions through explicit proposals,
approval state, audit events, and durable receipt metadata.

The first version can start with local approval and append-only logs. The final
version should allow stronger independent verification without changing the tool
system.

## Why This Matters

BoardClaw can control real devices. For serious automation, the user may need
to prove:

- what action was proposed
- which board and device were targeted
- which policy allowed, denied, or paused the action
- who approved it
- which tool actually executed
- what input and output were recorded
- whether secrets were used
- whether output was redacted
- when the action happened

Logs are useful for debugging. Receipts are useful for trust.

## First-Version Scope

BoardClaw first version:

- local event log
- typed proposal object
- approval-required state for high-risk tools
- single-use approval token
- denial and expiration behavior
- input/output hashes in event metadata
- mobile-ready approval API shape

No external proof system is required for the first version. The metadata must be
complete enough that one can be attached later.

## Final-Version Scope

BoardClaw final version:

- signed proposal receipts
- signed approval records
- signed execution receipts
- mobile approval and verification
- policy budget references
- receipt chain attached to BoardClaw events
- offline queueing with later sync

## Flow

```text
1. User sends request.
2. BoardClaw asks the selected local model.
3. Model proposes a typed tool call.
4. BoardClaw creates an action proposal.
5. Policy checks risk, board, channel, target, and budget.
6. Low-risk allowed action executes immediately.
7. High-risk action becomes pending.
8. Mobile or local approval surface asks the user to approve or deny.
9. Valid approval executes once before expiry.
10. Execution result is recorded with input/output hashes.
11. Response includes outcome and receipt reference.
```

## Approval Metadata

Every pending action should include:

- action id
- board id
- board role
- channel
- principal
- tool name
- target device
- target address or pin when applicable
- risk level
- model route
- policy rule
- expiration time
- dry-run result when available
- human-readable summary
- machine-readable input hash

The phone or web approval UI should show enough detail for the user to make a
real decision, not just a generic "allow" button.

## High-Risk Actions

Mobile or local approval should be required for:

- relay activation
- heater, pump, charger, lock, or motor control
- PWM changes
- robot motion
- firmware flashing
- I2C/SPI/UART writes to unknown devices
- destructive shell/file operations
- network calls that spend money, expose data, or control private systems

## Offline Behavior

If mobile approval is unavailable:

- read-only tools continue
- low-risk local automations continue
- high-risk tools remain pending or are denied by policy
- local physical confirmation can be added later
- receipts can sync after connectivity returns

## Tool Metadata Requirement

Every tool call should be able to produce:

```text
tool_name
action_kind
target
input_hash
output_hash
risk_level
principal
channel
board_id
device_id
approval_id
policy_decision
secrets_used
redaction_report
timestamp
```

This metadata is the bridge from basic logging to final-version receipt
verification.
