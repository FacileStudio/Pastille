# BOHS-01 Gateway Integration Guide

This guide describes how the Bulle gateway integrates BOHS-01 hardware peripherals like Pastille into its voice and agent execution loops.

## Endpoints

The Bulle API exposes four hardware endpoints:

| Method and path | Caller | Purpose |
| :--- | :--- | :--- |
| `POST /api/hardware/devices/claim` | Bulle app, user session | Claim a device and mint its device token |
| `GET /api/hardware/devices` | Bulle app, user session | List the user's devices |
| `DELETE /api/hardware/devices/{id}` | Bulle app, user session | Revoke a device |
| `GET /api/hardware/ws` | Device or Bulle app | Open a `bohs.v1` WebSocket session |

The WebSocket endpoint terminates TLS, authenticates the caller, negotiates the `bohs.v1` subprotocol, and carries audio and tool frames:

```
wss://<gateway-host>/api/hardware/ws
```

## Connection paths

A device reaches the gateway on one of two paths. The gateway sees the same `bohs.v1` frames on both.

```mermaid
flowchart LR
    DEV["BOHS-01 device"]
    APP["Bulle app (mobile or desktop)"]
    GW["Bulle gateway /api/hardware/ws"]
    DEV -- "BLE GATT or USB CDC, stream framing" --> APP
    APP -- "WSS, user session, ?device_id=hw_..." --> GW
    DEV -- "WSS direct, Bearer bohs_tok_" --> GW
```

1. **Direct Wi-Fi.** The device opens WSS to `/api/hardware/ws` and authenticates with `Authorization: Bearer bohs_tok_...`.
2. **App relay.** The device talks to the Bulle app over BLE GATT or USB CDC. The app opens WSS to `/api/hardware/ws?device_id=hw_...` with the user's authenticated session and forwards frames in both directions.

## Device claim

The Bulle app claims a device after the user authorizes pairing with a physical tap. See the `0x03` provisioning frames in [BOHS-01-WIRE.md](BOHS-01-WIRE.md).

### Claim request

`POST /api/hardware/devices/claim` requires an authenticated user session. Cookie-authenticated calls also send the `X-Facile-CSRF` header.

```json
{"hardware_id": "4f2a91c0", "name": "Pastille", "device_class": "charm"}
```

`hardware_id` is the 4-byte hardware ID from the device's advertising data, in hex. `device_class` is one of `charm`, `field`, `totem`.

### Claim response

The gateway mints a device token of the form `bohs_tok_` followed by 32 lowercase hex characters. It stores only the SHA-256 hash of the token, bound to the user. It returns the plaintext token once:

```json
{
  "device": {"id": "hw_01JC8Q3ZK7R4M2T9V6X1N5B0PD", "name": "Pastille", "device_class": "charm"},
  "device_token": "bohs_tok_3f9a1c07e25b48d6a0c4e19b7d2f8a61",
  "gateway_ws_url": "wss://bulle.facile.studio/api/hardware/ws"
}
```

The app passes `device_token` and `gateway_ws_url` to the device in `prov.configure` over the encrypted BLE link, then drops the token from memory.

### Listing and revoking

- `GET /api/hardware/devices` returns the user's devices with `last_seen_at` and `last_transport` (`ble`, `wifi`, `usb`).
- `DELETE /api/hardware/devices/{id}` revokes the device. The gateway deletes the token hash and closes every open `/api/hardware/ws` session for that device, direct or relayed, with WebSocket close code `4401`.

A revoked device can no longer open a session. To use it again, the user pairs it again from the Bulle app, which needs a physical tap on the device.

## Session authentication

The caller authenticates during the WebSocket handshake. Every connection must request `Sec-WebSocket-Protocol: bohs.v1`.

### Direct device connection

The device sends its token in the `Authorization` header:

```http
Authorization: Bearer bohs_tok_3f9a1c07e25b48d6a0c4e19b7d2f8a61
```

The gateway:

1. Checks the token format.
2. Hashes the token with SHA-256 and looks up the hash in its device table.
3. Resolves the owning user and agent workspace.
4. Returns HTTP `401 Unauthorized` and refuses the upgrade if any step fails.

### App relay connection

The app connects to `/api/hardware/ws?device_id=hw_...` with the user's authenticated session. The gateway:

1. Validates the user session. It returns HTTP `401 Unauthorized` if the session is missing or invalid.
2. Checks that `device_id` belongs to that user. It returns HTTP `404 Not Found` if it does not.

### Relay rules

The app forwards frames without changing them.

- **Device to gateway.** The app reassembles each BLE characteristic or USB stream (see stream framing in [BOHS-01-WIRE.md](BOHS-01-WIRE.md)), strips the 2-byte length prefix, and sends each frame as one binary WebSocket message.
- **Gateway to device.** The app prefixes each WebSocket message with its `u16be` length and writes it to the device. On BLE, it splits the bytes into chunks of at most ATT_MTU - 3 bytes and writes `0x01` frames to `B002` and `0x02` frames to `B003`.
- The app never forwards `0x03` provisioning frames. The gateway closes a session that sends one.

## Voice pipeline integration

The gateway connects the peripheral to its real-time speech-to-text (STT) and text-to-speech (TTS) engines. Audio frames carry raw PCM16 little-endian at 16 kHz mono, 640 bytes per 20 ms frame, in both directions. The gateway runs no audio codec in `bohs.v1`.

```mermaid
flowchart LR
    subgraph Ingest["Ingestion (speech to text)"]
        MIC["Device mic"] -- "0x01 PCM16 frames" --> JB["Jitter buffer"]
        JB --> RING["16 kHz PCM ring buffer"]
        RING --> VAD["Silero VAD"]
        VAD -- "speech end or EOU flag" --> STT["whisper.cpp stream"]
        STT -- "transcript" --> LLM["Bulle LLM agent"]
    end
    subgraph Egress["Egress (text to speech)"]
        LLM2["Bulle LLM agent"] -- "streaming text" --> TTS["Piper TTS"]
        TTS -- "PCM" --> RS["Resample to 16 kHz mono PCM16"]
        RS --> PK["Split into 640-byte frames"]
        PK -- "0x01 frames" --> SPK["Device speaker"]
    end
```

### Ingestion: Speech recognition

1. **Packet reception**: The gateway reads frames tagged with opcode `0x01`.
2. **Jitter buffer**: Frames enter a 60 ms adaptive jitter buffer that reorders sequence numbers and smooths network transit jitter.
3. **PCM buffering**: The gateway appends each 640-byte payload to a 16 kHz 16-bit PCM ring buffer. No decoding step is needed.
4. **Voice activity detection**: A lightweight Silero VAD filter analyzes PCM chunks in real time. It detects speech onset and speech completion.
5. **Whisper.cpp inference**: When VAD marks end-of-utterance or the device sends the EOU flag, the buffered audio feeds into `whisper.cpp`. The model transcribes the user query and routes the text into the active conversation turn.

### Egress: Voice synthesis

1. **Token streaming**: As the LLM generates response tokens, Bulle buffers complete sentences or comma-delimited phrases.
2. **Piper TTS synthesis**: Sentences pass to Piper TTS. The gateway resamples its output to 16 kHz 16-bit mono PCM when the voice model uses another rate.
3. **Packetization**: The gateway splits the audio into 640-byte frames (320 samples, 20 ms), pads the last frame with zero samples, attaches sequence numbers, and sets opcode `0x01`.
4. **Delivery**: The gateway sends one frame per WebSocket message. On the relay path, the app forwards them to the device. The device plays them directly into its I2S audio amplifier.

## Reverse MCP tool registration

When a BOHS-01 session opens, on either path, Bulle queries the device's tools via `tools/list` over opcode `0x02`. Bulle injects these edge tools into the LLM context.

### Tool manifest injection

Bulle exposes peripheral tools to the model with a dedicated hardware prefix:

- `charm.display_avatar`: Changes the graphic expression on the wearable screen.
- `charm.haptic_pulse`: Vibrates the charm to notify the user tactilely.
- `charm.get_telemetry`: Checks remaining battery level and signal quality.

The model system prompt receives context explaining physical presence:

```
You are connected to the user through their Pastille physical wearable charm.
You can express emotions on its circular screen with charm.display_avatar.
You can get the user's attention with charm.haptic_pulse.
Check battery status with charm.get_telemetry when relevant.
```

### Turn execution lifecycle

During conversation turns, the LLM can call edge tools concurrently with voice output:

1. User asks: "Are you listening?"
2. Agent initiates generation:
   - Dispatches `charm.display_avatar(state="listening", emotion="focused")`
   - Dispatches `charm.haptic_pulse(pattern="tap", duration_ms=40, intensity=50)`
3. Agent answers via speech: "Yes, I am here."
4. When voice synthesis begins:
   - Dispatches `charm.display_avatar(state="speaking", emotion="happy")`
5. When playback completes:
   - Dispatches `charm.display_avatar(state="idle", emotion="neutral")`

### Latency bounds and timeouts

Physical tool execution must not block conversational flow:

- Maximum RPC round-trip timeout: 350 ms.
- If the peripheral fails to acknowledge a tool execution within 350 ms, the gateway aborts the call and continues the dialogue.
- If the session closes, direct or relayed, the gateway immediately removes peripheral tools from the active model session. Future alerts fall back to desktop and mobile push channels.
