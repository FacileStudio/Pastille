# BOHS-01 Wire Protocol Specification

The BOHS-01 wire protocol `bohs.v1` multiplexes real-time audio, Reverse MCP control messages, and BLE provisioning messages. The same frames run over three transports: Wi-Fi WSS, BLE 5.0 GATT, and USB-C CDC.

## Pastille v0.4.0 implementation status

The Pastille v0.4.0 firmware implements most of this specification. Hardware testing of the radio, display, touch and haptic drivers is still pending.

| Feature | Pastille v0.4.0 |
| :--- | :--- |
| Transport A, Wi-Fi WSS: station mode, TLS 1.3, `bohs.v1` WebSocket with the bearer token, reconnect with backoff | Runs on the device when the config has `prefer_ble: false` and a `wifi_ssid` |
| Transport B, BLE GATT: advertising, `B001` to `B004`, stream framing, LE Secure Connections with bonding | Runs on the device |
| Transport C, USB CDC (stream framing, `0x01`, `0x02`, `0x03` rejected with `-32001`) | Runs on the device |
| `0x03` provisioning on `B004`, the tap window, `prov.state`, config and bond kept in flash | Runs on the device |
| Reverse MCP: `tools/list`, `charm.display_avatar`, `charm.haptic_pulse`, `charm.get_telemetry` | Runs on the device |
| `charm.low_battery` and `charm.tap_event` notifications | Runs on the device |
| Microphone capture and speaker playback | Not implemented. The device validates inbound `0x01` frames and discards them, and sends none. Its capability bits are `0x0C` (display, haptic). |

On the device, `charm.get_telemetry` reports `rssi_dbm` and `temp_c` as `null`: the firmware does not read the radio RSSI yet, and esp-hal has no ESP32-S3 temperature sensor driver.

The device verifies the gateway's TLS chain against four pinned roots: ISRG Root X1, ISRG Root X2, GTS Root R4 and GlobalSign Root CA. It checks the host name but not the validity dates, because it has no trusted clock. A gateway behind another CA cannot be reached over Wi-Fi direct.

The firmware keeps the config and one BLE bond in a CRC-checked record in the `pastille` flash partition, written alternately to two sectors. Flash encryption is not enabled, so the device token and Wi-Fi PSK are stored in plain text on the chip's flash.

## Frames

A `bohs.v1` frame is a byte sequence that starts with a one-byte opcode. The opcode identifies the payload type.

| Opcode | Frame type | Direction | Description |
| :--- | :--- | :--- | :--- |
| `0x01` | Audio frame | Bidirectional | Real-time PCM16 audio |
| `0x02` | Control frame | Bidirectional | JSON-RPC 2.0 messages for Reverse MCP and events |
| `0x03` | Provisioning frame | Bidirectional | JSON-RPC 2.0 pairing, device token delivery, and Wi-Fi configuration. BLE only. |

Opcodes `0x04` through `0xFF` are reserved for future protocol revisions. A receiver discards a frame with a reserved opcode.

The maximum frame size is 1024 bytes, including the opcode. A receiver rejects a larger frame. A sender never emits one.

## Transport layer

The frame format is the same on every transport. Only the outer framing differs.

| Transport | Outer framing | Opcodes carried |
| :--- | :--- | :--- |
| A. Wi-Fi WSS | One frame per binary WebSocket message. No length prefix. | `0x01`, `0x02` |
| B. BLE 5.0 GATT | Stream framing: `u16be` length prefix, then chunks of at most ATT_MTU - 3 bytes. | `0x01` on audio, `0x02` on control, `0x03` on provisioning |
| C. USB-C CDC ACM | Stream framing: `u16be` length prefix. | `0x01`, `0x02` |

### Stream framing

BLE characteristics and USB CDC are byte streams. A frame on these transports has no natural boundary, so the sender adds one.

```
+-----------------------------+------------------------------------+
|     Length (2B, u16be)      |        bohs.v1 frame (N bytes)     |
| N, excluding these 2 bytes  |   opcode + header + payload        |
+-----------------------------+------------------------------------+
```

- The sender writes the frame length `N` as an unsigned 16-bit big-endian integer, then the `N` frame bytes.
- `N` is between 1 and 1024. A receiver that reads `N = 0` or `N > 1024` treats the stream as broken. On BLE, it disconnects. On USB CDC, it discards its buffer and the rest of the bytes it has read, then parses the next bytes as a new length prefix. The host resynchronizes by reopening the port.
- On BLE, the sender splits the prefixed bytes into chunks of at most ATT_MTU - 3 bytes. Each chunk goes in one notification, indication, or write.
- The receiver appends each chunk to a buffer and extracts every complete frame from it. A chunk may end in the middle of a frame or hold the end of one frame and the start of the next. The receiver must not assume chunks align with frames.
- Each BLE characteristic is its own stream in each direction. The receiver keeps one reassembly buffer per characteristic per direction.

Example: a 644-byte audio frame with ATT_MTU 247 becomes 646 prefixed bytes, sent as three notifications of 244, 244, and 158 bytes.

### Transport A: Wi-Fi WebSocket (WSS)

BOHS-01 requires a secure WebSocket connection over TLS (WSS). During the handshake, the client requests the `bohs.v1` subprotocol and sends its device token:

```http
GET /api/hardware/ws HTTP/1.1
Host: gateway.bulle.local
Upgrade: websocket
Connection: Upgrade
Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==
Sec-WebSocket-Version: 13
Sec-WebSocket-Protocol: bohs.v1
Authorization: Bearer bohs_tok_3f9a1c07e25b48d6a0c4e19b7d2f8a61
```

The gateway accepts the connection by echoing `Sec-WebSocket-Protocol: bohs.v1` in its response header. It rejects connections that omit this subprotocol.

All communication uses binary WebSocket messages. Each message carries exactly one `bohs.v1` frame with no length prefix. The device never sends `0x03` frames over WSS.

### Transport B: Bluetooth Low Energy (BLE 5.0 GATT)

The device is a GATT server. The Bulle mobile or desktop app is the central. Centrals should request the 2M PHY and an ATT_MTU of 247. Stream framing works at any ATT_MTU, including the default of 23, but PCM16 audio needs about 256 kbit/s in each direction, which needs the 2M PHY and a large MTU in practice.

All UUIDs are full 128-bit UUIDs. They share the suffix `-f4c1-4e00-b001-000000000001`. The labels `B001` to `B004` are short names for this document only. They are not 16-bit Bluetooth SIG UUIDs.

| Label | UUID | Role | Properties | Carries |
| :--- | :--- | :--- | :--- | :--- |
| Service | `b0010000-f4c1-4e00-b001-000000000001` | BOHS-01 primary service | | |
| `B001` | `b0010001-f4c1-4e00-b001-000000000001` | Device info and capabilities | Read | 8-byte info record, no stream framing |
| `B002` | `b0010002-f4c1-4e00-b001-000000000001` | Audio plane | Notify, Write Without Response | Stream-framed `0x01` frames |
| `B003` | `b0010003-f4c1-4e00-b001-000000000001` | Control and Reverse MCP plane | Indicate, Write | Stream-framed `0x02` frames |
| `B004` | `b0010004-f4c1-4e00-b001-000000000001` | Provisioning plane | Indicate, Write (encrypted link required) | Stream-framed `0x03` frames |

A receiver rejects a frame whose opcode does not match its characteristic.

#### Advertising

The device advertises the service UUID with 6 bytes of service data:

| Byte | Field | Value |
| :--- | :--- | :--- |
| 0 | `state` | Provisioning state code (see below) |
| 1 | `caps` | Capability bitfield (see below) |
| 2-5 | `hw_id` | Hardware ID: the last 4 bytes of the eFuse base MAC. The first 3 bytes are the vendor OUI and repeat across devices. |

Pastille v0.4.0 advertises `[0x00, 0x0C, hw_id[0], hw_id[1], hw_id[2], hw_id[3]]` when unclaimed: display and haptic, no audio yet.

The advertising packet holds the flags and one Service Data (128-bit UUID) structure: the service UUID followed by these 6 bytes. The device name `Pastille` is in the scan response. The device restarts advertising whenever the state code changes.

#### Device info record (`B001`)

A read of `B001` returns 8 bytes:

| Byte | Field | Description |
| :--- | :--- | :--- |
| 0 | `version` | Info record version, `0x01` |
| 1 | `state` | Provisioning state code |
| 2 | `caps` | Capability bitfield |
| 3 | `battery_pct` | Battery charge, 0 to 100 |
| 4 | `fw_major` | Firmware major version |
| 5 | `fw_minor` | Firmware minor version |
| 6 | `fw_patch` | Firmware patch version |
| 7 | `active_transport` | Transport code |

Provisioning state codes: `0x00` unclaimed, `0x01` awaiting_tap, `0x02` authorized, `0x03` provisioned.

Capability bits: `0x01` microphone, `0x02` speaker, `0x04` display, `0x08` haptic. Bits 4-7 are reserved and set to zero.

Transport codes: `0x00` BLE, `0x01` Wi-Fi, `0x02` USB.

`B001` is readable without encryption so the app can identify a device before bonding.

#### Link security

- The device accepts writes to `B004`, and subscriptions to it, only over an encrypted link. On an unencrypted link, the device rejects the write with ATT error Insufficient Encryption (`0x0F`). The central then pairs and retries. The provisioner also requires the key to come from a bonded LE Secure Connections pairing; on an encrypted link without a bond, every `0x03` request gets `-32001`.
- When numeric comparison is used, the LCD shows the 6-digit code and the user confirms it on the device with a tap within 30 seconds. No tap cancels the pairing.
- The device keeps the most recent bond in flash and restores it at boot, so a bonded phone reconnects without pairing again.
- The device advertises IO capability DisplayYesNo. When the central supports it, the pairing uses Numeric Comparison: the LCD shows the 6-digit code and the user confirms on both sides. When the central only supports Just Works, the physical tap during the `prov.identify` window is the proof that the user holds the device.
- The device token and the Wi-Fi PSK never travel over an unencrypted link.
- The app should bond before it subscribes to `B002` and `B003`, so voice audio and tool calls also travel encrypted.

### Transport C: USB-C CDC ACM

The device exposes a USB CDC ACM serial port on `GPIO19` (D-) and `GPIO20` (D+). Frames use stream framing in both directions. The host app relays them to the gateway the same way the BLE relay does.

The device rejects `0x03` frames on USB CDC. It answers every provisioning request on this transport with JSON-RPC error `-32001`.

## Opcode 0x01: Audio data frame

The audio frame carries user speech from the peripheral microphone to the gateway, and synthesized speech from the gateway to the peripheral speaker.

```
+---------------+---------------+-------------------------------+--------------------------+
| Opcode (0x01) |   Flags (1B)  |     Sequence Number (2B)      |  PCM16 LE payload (640B) |
|    1 Byte     |    1 Byte     |     16-bit Big-Endian uint    |   320 samples, 20 ms     |
+---------------+---------------+-------------------------------+--------------------------+
```

An audio frame is 644 bytes: a 4-byte header and a 640-byte payload. On BLE it spans several notifications (see stream framing).

### Field definitions

- `Opcode` (1 byte): Constant value `0x01`.
- `Flags` (1 byte): Bitfield indicating packet metadata:
  - Bit 0 (`0x01`): End-of-utterance (EOU) indicator. Set to 1 on the final frame of an utterance.
  - Bit 1 (`0x02`): Voice Activity Detection (VAD) active flag.
  - Bits 2-7: Reserved. Must be set to zero.
- `Sequence Number` (2 bytes): Monotonically increasing unsigned 16-bit integer in big-endian byte order (`0x0000` to `0xFFFF`). Wraps around to zero on overflow. Enables jitter buffer packet reordering and packet loss detection. Each direction keeps its own counter.
- `Payload` (640 bytes): 320 signed 16-bit PCM samples, little-endian.

A receiver discards an audio frame that is not exactly 644 bytes or that has a reserved flag bit set.

### Audio codec profile (v1)

- Encoding: raw PCM, signed 16-bit, little-endian.
- Sample rate: 16,000 Hz (16 kHz wideband speech).
- Channels: 1 (mono).
- Frame duration: 20 ms (320 samples, 640 bytes per frame).
- Bitrate: 256 kbit/s per direction, before framing.

Every audio frame carries exactly 640 payload bytes. The sender pads the last frame of an utterance with zero samples.

Upstream frames transmit recorded user speech to the gateway. Downstream frames transmit synthesized agent speech to the peripheral speaker. Both streams use identical framing and sequencing.

A future protocol revision may add a compressed codec such as Opus. `bohs.v1` defines PCM16 only.

## Opcode 0x02: Control and tool frame

Control frames transport UTF-8 encoded JSON-RPC 2.0 messages.

```
+---------------+-----------------------------------------------+
| Opcode (0x02) |            UTF-8 JSON-RPC Payload             |
|    1 Byte     |              Up to 1023 bytes                 |
+---------------+-----------------------------------------------+
```

The payload follows the JSON-RPC 2.0 specification without modification. Each frame holds one complete JSON-RPC message. A message must fit in 1023 bytes, so the device keeps its tool list and tool results under that limit.

### Request handling

These rules apply to `0x02` and `0x03` messages alike.

- The device answers every request that has an `id` with a result or an error object. It never lets a request time out silently.
- A message without an `id` is a notification. The device ignores it: it neither runs it nor answers it.
- The `id` must be a string of at most 64 bytes, a number, or `null`. Any other `id` gets error `-32600` with `"id":null`.
- A payload that is not valid JSON gets error `-32700` with `"id":null`. A JSON value that is not an object, such as a batch array, gets `-32600`.

| Code | Message | Meaning |
| :--- | :--- | :--- |
| `-32700` | `parse error` | The payload is not valid JSON |
| `-32600` | `invalid request` | Not an object, or the `id` has a forbidden type or length |
| `-32601` | `method not found` | Unknown method, or `charm.snapshot` on a device without a camera |
| `-32602` | `unknown tool` | `tools/call` names a tool the device does not have |
| `-32602` | `invalid params` | Missing or out-of-range arguments, such as `intensity` 500 |
| `-32603` | `internal error` | The response did not fit in a frame |
| `-32001` | `unauthorized` | `0x03` only: see the provisioning error codes |

## Reverse MCP specification

BOHS-01 uses Reverse Model Context Protocol (Reverse MCP). In standard MCP architectures, a client connects to external tool servers. In BOHS-01, the gateway acts as the MCP client, and the wearable peripheral acts as the MCP server.

This design enables the cloud or local LLM agent to directly manipulate the peripheral hardware as native tools.

### Handshake and tool advertisement

Immediately after a session opens, over direct WSS or through a relay, the gateway queries the device for supported tools:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/list",
  "params": {}
}
```

The device responds with the tools it supports. A device lists only tools its hardware can run, and the response must fit in one frame. Pastille lists three tools:

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

`pattern` sets the number of pulses: 1 for `tap`, 2 for `double_tap`, 3 for `alert`, 4 for `error`. Each pulse lasts `duration_ms` (default 50) at `intensity` percent (default 80), with 60 ms between pulses. The device answers once the pulse is queued for the motor, not when it ends.

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

`rssi_dbm` and `temp_c` are `null` when the device has no source for them.

#### Snapshot tool (future camera support)

`charm.snapshot` is an optional tool for devices with an image sensor. Pastille has no camera and does not list it. A device that lists it must keep each response within the 1024-byte frame limit, so it returns only images that fit. The gateway requests an image frame:

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

If the gateway calls `charm.snapshot` on a device without a camera sensor, the device returns a standard JSON-RPC error:

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

Pastille v0.4.0 sends this notification to the transport the gateway session last used, and drops it when no session is open. `timestamp_ms` counts milliseconds since boot, because the device has no wall clock. A press held 800 ms is a `long_press`, and two taps within 350 ms make a `double_tap`.

### Low battery alert notification

Triggered when battery charge drops below 15 percent or 5 percent thresholds. Each threshold alerts once, and alerts again only after the charge has climbed 3 points above it. A device that boots below a threshold alerts once for the lowest threshold it is under.

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

## Opcode 0x03: Provisioning frame

Provisioning frames carry UTF-8 JSON-RPC 2.0 messages on BLE characteristic `B004` only. The Bulle app uses them to pair a device with a user account and to deliver its device token and Wi-Fi credentials.

```
+---------------+-----------------------------------------------+
| Opcode (0x03) |            UTF-8 JSON-RPC Payload             |
|    1 Byte     |              Up to 1023 bytes                 |
+---------------+-----------------------------------------------+
```

Every write to `B004` requires an encrypted, bonded link (see link security). The app never forwards `0x03` frames to the gateway.

### Provisioning states

```mermaid
stateDiagram-v2
    [*] --> Unclaimed
    Unclaimed --> AwaitingTap: prov.identify
    AwaitingTap --> Authorized: User taps pad (GPIO7) within 60 s
    AwaitingTap --> Unclaimed: 60 s window elapsed
    Authorized --> Provisioned: prov.configure on encrypted link
    Authorized --> Unclaimed: 60 s window elapsed
    Provisioned --> AwaitingTap: prov.identify (re-pair)
```

- The 60 second window opens when the device accepts `prov.identify`. The tap and `prov.configure` must both happen before it ends.
- If the window ends or the encrypted link drops before `prov.configure` succeeds, the device returns to the state it held before `prov.identify`. A new device returns to `unclaimed`. A device being re-paired returns to `provisioned` and keeps its old config.
- A provisioned device accepts `prov.identify` to re-pair. The new config replaces the old one only after a tap and a successful `prov.configure`.
- A `prov.identify` while the window is open returns the status and changes nothing. It neither restarts nor extends the window.
- The window belongs to the BLE connection that sent `prov.identify`. A disconnect closes it, and `prov.configure` from any other connection gets `-32001`.
- The device answers every `0x03` request with `-32001` unless it arrived from a connected central over an encrypted link. This includes `prov.identify` and `prov.status`.

### Error codes

| Code | Message | Meaning |
| :--- | :--- | :--- |
| `-32001` | `unauthorized` | Wrong state, link not encrypted, or wrong transport |
| `-32601` | `method not found` | Unknown method |
| `-32602` | `invalid params` | Missing field, field too long, or bad token format |

### prov.identify

Asks the device to show itself. The device pulses a `double_tap` haptic pattern, draws a pairing ring on the LCD, and opens the 60 second tap window.

Request:

```json
{"jsonrpc":"2.0","id":1,"method":"prov.identify","params":{}}
```

Response, a status result:

```json
{"jsonrpc":"2.0","id":1,"result":{"state":"awaiting_tap","transport":"ble","link_encrypted":true,"hardware_id":"4f2a91c0"}}
```

### prov.status

Returns the provisioning state and the configured transport. Valid in every state.

Request:

```json
{"jsonrpc":"2.0","id":2,"method":"prov.status","params":{}}
```

Response:

```json
{"jsonrpc":"2.0","id":2,"result":{"state":"authorized","transport":"ble","link_encrypted":true,"hardware_id":"4f2a91c0"}}
```

A status result has four fields:

- `state`: one of `unclaimed`, `awaiting_tap`, `authorized`, `provisioned`.
- `transport`: the transport the device uses to reach the gateway once provisioned, set by `prefer_ble`: one of `ble`, `wifi`, `usb`.
- `link_encrypted`: `true` when the BLE link is encrypted.
- `hardware_id`: 8 lowercase hex characters. They encode the same 4 bytes as `hw_id` in the advertising service data, in the same order. The value is fixed per device, so the app can use it to recognize a charm across browsers and reinstalls.

### prov.state notification

The device sends this notification on `B004` every time its provisioning state changes. The app waits for `authorized` before it claims the device. An app can poll `prov.status` instead.

```json
{"jsonrpc":"2.0","method":"prov.state","params":{"state":"authorized"}}
```

### prov.configure

Delivers the gateway URL, the device token, and optional Wi-Fi credentials. The device accepts it only when both hold:

1. The provisioning state is `authorized`.
2. The BLE link is encrypted.

Otherwise the device answers with error `-32001` `unauthorized` and stores nothing.

| Field | Type | Required | Rule |
| :--- | :--- | :--- | :--- |
| `gateway_url` | string | Yes | `wss://` URL with a non-empty host, at most 96 characters |
| `device_token` | string | Yes | `bohs_tok_` followed by 32 lowercase hex characters |
| `wifi_ssid` | string | No | At most 32 characters. Required when `prefer_ble` is `false`. |
| `wifi_psk` | string | No | At most 64 characters. Omit for an open network. |

A present optional field with a type other than string gets `-32602`.
| `prefer_ble` | boolean | Yes | `true` selects BLE relay first, `false` selects Wi-Fi direct first |

Request:

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "prov.configure",
  "params": {
    "gateway_url": "wss://bulle.facile.studio/api/hardware/ws",
    "device_token": "bohs_tok_3f9a1c07e25b48d6a0c4e19b7d2f8a61",
    "wifi_ssid": "Atelier",
    "wifi_psk": "correct horse battery staple",
    "prefer_ble": true
  }
}
```

Response:

```json
{"jsonrpc":"2.0","id":3,"result":{"status":"provisioned"}}
```

Error response:

```json
{"jsonrpc":"2.0","id":3,"error":{"code":-32001,"message":"unauthorized"}}
```

On success the device writes the config to flash, moves to `provisioned`, and sends a `prov.state` notification. When the config selects Wi-Fi direct, the device then joins the network and opens the gateway session.
