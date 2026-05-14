# Uniclaw and SecuClaw Integration

BoardClaw should be designed for Uniclaw, but not blocked by it.

Uniclaw is the proof layer: it creates signed, chained receipts for proposals,
approvals, and tool executions. BoardClaw is the board/runtime/control layer.

## Why Integrate

BoardClaw controls physical devices. For serious automation, the user may need
to prove:

- what action was proposed
- which policy allowed or denied it
- who approved it
- what tool actually executed
- whether a secret was used
- whether output was redacted
- when the action happened

Logs are useful for debugging. Receipts are useful for trust.

## Integration Timing

BoardClaw MVP:

- local logs
- typed tools
- approval state machine
- no Uniclaw dependency

BoardClaw security milestone:

- optional `uniclaw-host` sidecar
- proposal receipts
- tool execution receipts
- pending approval flow

SecuClaw milestone:

- mobile verification
- passkey/biometric approval
- receipt verification on phone
- high-risk tool receipts required

## SecuClaw Definition

```text
SecuClaw = BoardClaw + Uniclaw + mobile verification
```

SecuClaw is not a separate hardware runtime. It is the secured profile of
BoardClaw.

## Flow

```text
1. User sends request.
2. BoardClaw asks local model.
3. Model proposes a typed tool call.
4. BoardClaw builds an action proposal.
5. Uniclaw checks constitution and budget.
6. If allowed, BoardClaw executes the tool.
7. If pending, mobile app asks the user to approve or deny.
8. BoardClaw executes only after approval.
9. BoardClaw records tool execution with Uniclaw.
10. Receipt URL/hash is attached to the event log.
```

## Capability Mapping

BoardClaw tool capabilities should map cleanly to Uniclaw-style capabilities.

| BoardClaw capability | Uniclaw-style capability |
|---|---|
| `gpio.read` | device read |
| `gpio.write` | device write |
| `i2c.read` | bus read |
| `i2c.write` | bus write |
| `uart.write` | serial write |
| `mqtt.publish` | network/control publish |
| `home_assistant.call_service` | API/service call |
| `shell.safe_exec` | shell exec |
| `file.write_allowed` | file write |
| `model.remote_query` | LLM query |
| `secret.read` | secret read |

Uniclaw does not need to know every GPIO pin detail at first. BoardClaw can put
pin/device detail in the action target and input hash.

## Mobile Verification

Mobile verification should be used for high-risk actions:

- remote unlock
- relay activation
- heater/pump/motor control
- robot motion
- firmware flashing
- destructive shell/file operations
- network calls that spend money or expose data

The phone should show:

- action summary
- board identity
- tool name
- target device
- risk level
- policy rule
- expiration time
- receipt hash

Approval should expire quickly.

## Offline Mode

If the phone is offline:

- low-risk read-only tools continue
- high-risk tools remain pending or denied
- local physical approval button can be supported later
- receipts can sync when connectivity returns

## Design Requirement for BoardClaw

Every tool call should produce enough metadata for Uniclaw later:

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
secrets_used
redaction_report
```

That is the bridge from BoardClaw to SecuClaw.

