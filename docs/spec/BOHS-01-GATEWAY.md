# BOHS-01 Gateway Integration Guide

This guide describes how the Bulle gateway integrates BOHS-01 hardware peripherals like Pastille into its voice and agent execution loops.

## Endpoint configuration

The gateway exposes a dedicated WebSocket endpoint:

```
wss://<gateway-host>/api/hardware/ws
```

This endpoint terminates TLS, performs device authentication, negotiates the `bohs.v1` subprotocol, and manages audio streams and tool channels.

## Device authentication

Authentication uses pre-shared device tokens issued by the Facile Registre service and stored securely in Casier.

### Authentication handshake

The peripheral transmits its credential in the standard HTTP `Authorization` header during the initial WebSocket handshake:

```http
Authorization: Bearer bohs_tok_e814a729c15d4872
```

The gateway checks this token against Registre:

1. Validate the cryptographic token signature and expiration date.
2. Confirm the device MAC address or eFuse ID matches the token binding.
3. Resolve the associated user account and agent workspace.
4. If validation fails, return HTTP `401 Unauthorized` and terminate the connection.

### Token revocation and rotation

Users revoke or cycle hardware tokens from the Bulle settings interface or through the Casier CLI:

```bash
casier secret set BULLE_DEVICE_TOKEN_PASTILLE "bohs_tok_$(openssl rand -hex 16)"
```

When a token is revoked in Registre, the gateway closes all active sockets associated with that token immediately with WebSocket close code `4401` (Unauthorized).

## Voice pipeline integration

The gateway connects the peripheral to its real-time speech-to-text (STT) and text-to-speech (TTS) engines.

```
+---------------------------------------------------------------------------------------------------+
|                                      Ingestion (Speech to Text)                                   |
|                                                                                                   |
|  [ Pastille Mic ]                                                                                 |
|         |                                                                                         |
|         v (Opus 16kHz via 0x01)                                                                   |
|  [ WebSocket Gateway ] ---> [ Jitter Buffer ] ---> [ Opus Decoder ] ---> [ 16kHz PCM Ring Buffer ] |
|                                                                                    |              |
|                                                                                    v              |
|                                                                            [ Silero VAD ]         |
|                                                                                    | (Speech End) |
|                                                                                    v              |
|                                                                           [ Whisper.cpp Stream ]  |
|                                                                                    |              |
|                                                                                    v (Transcript) |
|                                                                            [ Bulle LLM Agent ]    |
+---------------------------------------------------------------------------------------------------+
|                                      Egress (Text to Speech)                                      |
|                                                                                                   |
|  [ Bulle LLM Agent ]                                                                              |
|         | (Streaming Text Tokens)                                                                 |
|         v                                                                                         |
|  [ Piper TTS Engine ]                                                                             |
|         | (16kHz Raw PCM)                                                                         |
|         v                                                                                         |
|  [ Opus Encoder ]                                                                                 |
|         | (20ms Opus frames)                                                                      |
|         v                                                                                         |
|  [ WebSocket Gateway ] (Opcode 0x01) ---> [ Pastille Speaker ]                                    |
+---------------------------------------------------------------------------------------------------+
```

### Ingestion: Speech recognition

1. **Packet reception**: The gateway reads binary frames tagged with opcode `0x01`.
2. **Jitter buffer**: Packets enter a 60 ms adaptive jitter buffer that reorders sequence numbers and smooths network transit jitter.
3. **Opus decoding**: The server decodes Opus packets into 16 kHz 16-bit linear PCM audio.
4. **Voice activity detection**: A lightweight Silero VAD filter analyzes PCM chunks in real time. It detects speech onset and speech completion.
5. **Whisper.cpp inference**: When VAD marks end-of-utterance or the device sends the EOU flag, the buffered audio feeds into `whisper.cpp`. The model transcribes the user query and routes the text into the active conversation turn.

### Egress: Voice synthesis

1. **Token streaming**: As the LLM generates response tokens, Bulle buffers complete sentences or comma-delimited phrases.
2. **Piper TTS synthesis**: Sentences pass to Piper TTS, producing 16 kHz 16-bit mono PCM chunks.
3. **Opus encoding**: The gateway encodes the audio into 20 ms Opus packets at 24 kbps.
4. **Packetization**: The gateway attaches sequence numbers, sets opcode `0x01`, and streams frames to the peripheral over WebSocket. The peripheral plays them directly into its I2S audio amplifier.

## Reverse MCP tool registration

When a BOHS-01 peripheral connects, Bulle queries its tools via `tools/list` over opcode `0x02`. Bulle injects these edge tools into the LLM context.

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
- If the WebSocket disconnects, the gateway immediately removes peripheral tools from the active model session. Future alerts fall back to desktop and mobile push channels.
