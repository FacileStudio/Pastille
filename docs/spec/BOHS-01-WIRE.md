# BOHS-01 Wire Protocol Specification

The BOHS-01 wire protocol multiplexes real-time streaming audio and bidirectional control messages over a single secure WebSocket connection.

## Transport layer

BOHS-01 requires a secure WebSocket connection over TLS (WSS).

During the WebSocket connection handshake, the client requests the dedicated subprotocol:

```http
GET /api/hardware/ws HTTP/1.1
Host: gateway.bulle.local
Upgrade: websocket
Connection: Upgrade
Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==
Sec-WebSocket-Version: 13
Sec-WebSocket-Protocol: bohs.v1
Authorization: Bearer dev_9f83a1b02c4d
```

The gateway accepts the connection by echoing `Sec-WebSocket-Protocol: bohs.v1` in its response header. Connections omitting this subprotocol are rejected.

## Binary frame multiplexing

All communication uses WebSocket binary messages. Every binary frame begins with a one-byte opcode identifying its payload type.

| Opcode | Frame type | Direction | Description |
| :--- | :--- | :--- | :--- |
| `0x01` | Audio frame | Bidirectional | Real-time compressed audio packets |
| `0x02` | Control frame | Bidirectional | JSON-RPC 2.0 messages for Reverse MCP and events |

Opcodes `0x03` through `0xFF` are reserved for future protocol revisions.

### Opcode 0x01: Audio data frame

The audio frame carries compressed voice packets between the peripheral microphone and the gateway, as well as synthesized speech from the gateway to the peripheral speaker.

```
+---------------+---------------+-------------------------------+---------------------+
| Opcode (0x01) |   Flags (1B)  |     Sequence Number (2B)      |  Opus Payload (NB)  |
|    1 Byte     |    1 Byte     |     16-bit Big-Endian uint    |      Variable       |
+---------------+---------------+-------------------------------+---------------------+
```

#### Field definitions

- `Opcode` (1 byte): Constant value `0x01`.
- `Flags` (1 byte): Bitfield indicating packet metadata:
  - Bit 0 (`0x01`): End-of-utterance (EOU) indicator. Set to 1 on the final frame of an utterance.
  - Bit 1 (`0x02`): Voice Activity Detection (VAD) active flag.
  - Bits 2-7: Reserved. Must be set to zero.
- `Sequence Number` (2 bytes): Monotonically increasing unsigned 16-bit integer in big-endian byte order (`0x0000` to `0xFFFF`). Wraps around to zero on overflow. Enables jitter buffer packet reordering and packet loss detection.
- `Opus Payload` (variable length): Encoded Opus audio frame.

#### Audio codec profile

- Sample rate: 16,000 Hz (16 kHz wideband speech).
- Channels: 1 (mono).
- Frame duration: 20 ms (320 samples per frame).
- Bitrate: 16 kbps to 24 kbps constant bitrate (CBR).
- Complexity: Opus complexity mode 3 or lower on ESP32-S3 hardware.

Upstream frames transmit recorded user speech to the gateway. Downstream frames transmit synthesized agent speech to the peripheral speaker. Both streams use identical framing and sequencing.

### Opcode 0x02: Control and tool frame

Control frames transport UTF-8 encoded JSON-RPC 2.0 messages.

```
+---------------+-----------------------------------------------+
| Opcode (0x02) |            UTF-8 JSON-RPC Payload             |
|    1 Byte     |                   Variable                    |
+---------------+-----------------------------------------------+
```

The payload follows the JSON-RPC 2.0 specification without modification.

## Reverse MCP specification

BOHS-01 uses Reverse Model Context Protocol (Reverse MCP). In standard MCP architectures, a client connects to external tool servers. In BOHS-01, the gateway acts as the MCP client, and the wearable peripheral acts as the MCP server.

This design enables the cloud or local LLM agent to directly manipulate the peripheral hardware as native tools.

### Handshake and tool advertisement

Immediately following the WebSocket connection upgrade, the gateway queries the device for supported tools:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/list",
  "params": {}
}
```

The device responds with its registered tool definitions:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "tools": [
      {
        "name": "charm.display_avatar",
        "description": "Updates the face expression on the circular LCD display",
        "inputSchema": {
          "type": "object",
          "properties": {
            "state": {
              "type": "string",
              "enum": ["idle", "listening", "thinking", "speaking", "error"]
            },
            "emotion": {
              "type": "string",
              "enum": ["neutral", "happy", "focused", "calm", "curious", "warn"]
            }
          },
          "required": ["state"]
        }
      },
      {
        "name": "charm.haptic_pulse",
        "description": "Triggers a localized vibration pattern on the wearable motor",
        "inputSchema": {
          "type": "object",
          "properties": {
            "pattern": {
              "type": "string",
              "enum": ["tap", "double_tap", "alert", "error"]
            },
            "duration_ms": {
              "type": "integer",
              "minimum": 10,
              "maximum": 500
            },
            "intensity": {
              "type": "integer",
              "minimum": 1,
              "maximum": 100
            }
          },
          "required": ["pattern"]
        }
      },
      {
        "name": "charm.get_telemetry",
        "description": "Reads physical battery status, RSSI, and temperature from the peripheral",
        "inputSchema": {
          "type": "object",
          "properties": {}
        }
      },
      {
        "name": "charm.snapshot",
        "description": "Captures a still image from an optional camera sensor",
        "inputSchema": {
          "type": "object",
          "properties": {
            "resolution": {
              "type": "string",
              "enum": ["160x120", "320x240", "640x480"]
            },
            "format": {
              "type": "string",
              "enum": ["jpeg"]
            }
          }
        }
      }
    ]
  }
}
```

### Edge tool execution

When the language model invokes an edge tool, the gateway dispatches a JSON-RPC request to the peripheral.

#### Avatar display tool

Request:

```json
{
  "jsonrpc": "2.0",
  "id": 42,
  "method": "tools/call",
  "params": {
    "name": "charm.display_avatar",
    "arguments": {
      "state": "speaking",
      "emotion": "happy"
    }
  }
}
```

Response:

```json
{
  "jsonrpc": "2.0",
  "id": 42,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "Avatar state updated to speaking (happy)"
      }
    ]
  }
}
```

#### Haptic pulse tool

Request:

```json
{
  "jsonrpc": "2.0",
  "id": 43,
  "method": "tools/call",
  "params": {
    "name": "charm.haptic_pulse",
    "arguments": {
      "pattern": "double_tap",
      "duration_ms": 60,
      "intensity": 80
    }
  }
}
```

Response:

```json
{
  "jsonrpc": "2.0",
  "id": 43,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "Haptic pulse dispatched successfully"
      }
    ]
  }
}
```

#### Telemetry tool

Request:

```json
{
  "jsonrpc": "2.0",
  "id": 44,
  "method": "tools/call",
  "params": {
    "name": "charm.get_telemetry",
    "arguments": {}
  }
}
```

Response:

```json
{
  "jsonrpc": "2.0",
  "id": 44,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"battery_pct\":87,\"is_charging\":false,\"rssi_dbm\":-54,\"temp_c\":31.8}"
      }
    ]
  }
}
```

#### Snapshot tool (future camera support)

For devices equipped with an image sensor, the gateway requests an image frame:

Request:

```json
{
  "jsonrpc": "2.0",
  "id": 45,
  "method": "tools/call",
  "params": {
    "name": "charm.snapshot",
    "arguments": {
      "resolution": "320x240",
      "format": "jpeg"
    }
  }
}
```

Response:

```json
{
  "jsonrpc": "2.0",
  "id": 45,
  "result": {
    "content": [
      {
        "type": "image",
        "data": "/9j/4AAQSkZJRgABAQEASABIAAD...",
        "mimeType": "image/jpeg"
      }
    ]
  }
}
```

If the peripheral does not carry a camera sensor, it returns a standard JSON-RPC error:

```json
{
  "jsonrpc": "2.0",
  "id": 45,
  "error": {
    "code": -32601,
    "message": "Hardware capability not available on this device"
  }
}
```

## Event push notifications

The peripheral pushes spontaneous physical events to the gateway without waiting for a request. These use JSON-RPC 2.0 notifications (messages without an `id` field).

### Tap event notification

Triggered when the user taps or holds the capacitive front surface.

```json
{
  "jsonrpc": "2.0",
  "method": "charm.tap_event",
  "params": {
    "type": "double_tap",
    "timestamp_ms": 1730728912401
  }
}
```

Supported `type` values: `single_tap`, `double_tap`, `long_press`.

### Low battery alert notification

Triggered when battery charge drops below 15 percent or 5 percent thresholds.

```json
{
  "jsonrpc": "2.0",
  "method": "charm.low_battery",
  "params": {
    "battery_pct": 12,
    "voltage_mv": 3480
  }
}
```

The gateway uses this notification to gracefully warn the user via voice or queue a reminder before power cuts out.
