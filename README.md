# Pastille

<p align="center">
  <img src="assets/logo.svg" alt="Pastille Logo" width="72" height="72" />
</p>

Pastille is an open-source wearable AI charm and the reference hardware implementation of the Bulle Open Hardware Standard (BOHS-01).

It is a 38 mm disc that clips to clothing or hangs as a pendant. Pastille captures your voice with a digital microphone and streams bidirectional audio to your private Bulle gateway. It talks over BLE 5.0 through the Bulle app when your phone or laptop is near, and over Wi-Fi straight to the gateway when it is not. It shows animated companion expressions on a round LCD and gives haptic feedback.

Pastille targets a BOM under $25, a case that stays cool against your skin, a wake from deep sleep in under 50 ms, and over two weeks of standby on a single charge. These are design targets, not measurements.

Live landing page and interactive 3D board viewer: https://pastille.facile.studio

## PCB and 3D hardware previews

### 3D PCB render

![Pastille 3D PCB Preview](assets/pcb-3d.png)

### 2D PCB layout

![Pastille 2D PCB Layout](assets/pcb-2d.png)

### Electronic schematic

![Pastille Schematic](assets/schematic.png)

## Specifications and standard

Pastille implements the complete BOHS-01 specification suite:

- [BOHS-01 Architecture Specification](docs/spec/BOHS-01-ARCH.md): System architecture, 38 mm physical geometry, thermal envelope under 40°C, $23.85 BOM, TP4056 charging, transport selection, BLE pairing, and the 6-state power state machine.
- [BOHS-01 Wire Protocol Specification](docs/spec/BOHS-01-WIRE.md): The `bohs.v1` frame format over Wi-Fi WSS, BLE 5.0 GATT, and USB-C CDC, PCM16 16 kHz audio (opcode `0x01`), Reverse MCP edge tools (opcode `0x02`), and BLE provisioning (opcode `0x03`).
- [BOHS-01 Gateway Integration Guide](docs/spec/BOHS-01-GATEWAY.md): Bulle device claim and revocation, `bohs_tok_` device tokens, direct and app-relay sessions, Whisper.cpp ingestion, Piper TTS egress, and LLM edge tool calling.

## Connectivity and pairing

Pastille carries the same `bohs.v1` frames over three links:

- **BLE 5.0 relay.** Pastille talks to the Bulle mobile or desktop app over GATT service `b0010000-f4c1-4e00-b001-000000000001`. The app forwards frames to the gateway over its own WebSocket.
- **Wi-Fi direct.** Pastille connects to `wss://<gateway>/api/hardware/ws` with its `bohs_tok_` device token when no bonded phone or laptop is in range.
- **USB-C CDC.** Plugged into a computer running Bulle, Pastille sends frames over the USB serial port.

To pair, open the Bulle app and choose "Pair device". The app bonds over BLE and asks Pastille to identify itself. Pastille buzzes and shows a pairing ring. Tap the pad within 60 seconds. The app then claims the device, and sends it a device token and optional Wi-Fi credentials over the encrypted link.

## Architectural comparison

Standalone AI hardware often fails because designers build miniature smartphones running general-purpose operating systems like Android. BOHS-01 takes the opposite approach: dumb peripheral, intelligent gateway.

| Characteristic | Pastille (BOHS-01) | Muse Charm | OMI (Friend) | Rabbit R1 |
| :--- | :--- | :--- | :--- | :--- |
| Form factor | 38 mm circular disc | Screenless brooch | 40 mm circular pendant | Handheld rectangular slab |
| Target BOM cost | < $25 ($23.85) | ~ $18 | ~ $45 | ~ $199 retail |
| Display | 1.28-inch round IPS LCD (240x240) | None | None | 2.88-inch touchscreen |
| Battery life (standby) | > 14 days (18 uA deep sleep, target) | > 20 days | ~ 4 days | ~ 12 hours |
| Battery life (streaming) | > 24 h on BLE relay, > 8 h on Wi-Fi direct (targets) | > 12 hours | ~ 6 hours | ~ 4 hours |
| System architecture | Sensor / Actuator peripheral | Sensor peripheral | BLE audio streamer | Full Android OS (MediaTek MT6765) |
| Onboard operating system | None (Bare-metal Rust / Embassy) | None | None | Full Android AOSP fork |
| Boot / Wake latency | < 50 ms to wake, < 800 ms to audio | < 100 ms | Variable BLE pairing | > 45 seconds cold boot |
| Server protocol | Dual-transport `bohs.v1` (BLE 5.0 + Wi-Fi WSS) + Reverse MCP | WebSocket | BLE to smartphone app | Proprietary cloud bridge |
| Self-hostable | Yes (Local Bulle gateway) | Yes | Partial | No (Proprietary cloud only) |
| Privacy and data custody | Zero third-party telemetry | Zero third-party telemetry | Requires phone sync | Routes all traffic through cloud servers |
| Open hardware | Fully open (Fusible / KiCad) | Open | Partially open | Closed proprietary |

### Why dedicated operating systems fail on wearables

Devices like the Rabbit R1 and Humane Ai Pin placed a high-power mobile processor running Android inside a small enclosure. This creates structural flaws:

1. High quiescent power drain: Android kernels and background daemons consume tens of milliamperes even in standby. Pastille draws 18 microamperes in deep sleep.
2. Thermal throttling: Running a full operating system generates excess heat. Skin-contact electronics must stay below 40°C. Heavy processors quickly reach thermal limits and throttle performance.
3. Slow wake latency: Booting an operating system takes between 30 and 60 seconds. Pastille uses direct RTC wake interrupts that run application code in under 50 milliseconds.
4. Unnecessary bill of materials: Application processors, multirail PMICs, high-speed LPDDR4 memory, and eMMC flash storage drive BOM costs past $80 before assembly. Pastille uses a single ESP32-S3 microcontroller costing under $3.

Pastille treats the wearable as a low-latency remote sensor and display. Heavy speech recognition, language models, and tool integrations run on your home server, office workstation, or private cloud instance.

## Hardware development with Fusible

Pastille defines its printed circuit board programmatically using Fusible and tscircuit. You write board schematics, component placement, and net connections in TypeScript and React.

### Prerequisites

Install `bun`, `fusible`, and project dependencies:

```bash
cd hardware
bun install
```

### Interactive development and DRC verification

Validate the electrical rules, strapping pins, decoupling capacitors, and generate typed Rust firmware pin bindings:

```bash
fusible check
```

Launch the Fusible Studio live preview server:

```bash
fusible dev
```

### Exporting manufacturing and CAD artifacts

Build the enclosure bodies (`bodies/{front_bezel,lens,back_shell}.{step,stl}` and the reference parts), check that they fit, and export Gerbers, KiCad project files, STEP models and schematics:

```bash
fusible body
fusible assembly -f all
fusible export --preset all
```

`fusible body` runs `hardware/bodies/build.ts`, which reads `bodies/kit/main/kit.json` (from `fusible body kit main`). `fusible assembly` fails if any part collides. It writes `dist/assembly.glb` and `dist/assembly.step`. To refresh the landing page model, run `bun hardware/bodies/slim.ts dist/assembly.glb apps/web/static/models/pastille_assembly.glb`.

The output files land in `dist/fab/`, `dist/cad/`, `dist/docs/`, and `bodies/`. See the enclosure section of [BOHS-01-ARCH.md](docs/spec/BOHS-01-ARCH.md) for the stack, the battery choice and the print guide.

## Firmware guide

Pastille firmware runs bare-metal on the ESP32-S3 using Rust (`embassy-executor`, `esp-hal`, and `esp-wifi`) with pin bindings generated directly by Fusible (`apps/firmware/src/pins.rs`).

### Host unit tests

Run the host test suite covering the BOHS-01 wire protocol, BLE provisioning, Reverse MCP tool validation, I2S DMA audio VAD pipeline, power state machine, and integer trigonometric avatar renderer:

```bash
cd apps/firmware
cargo test
```

### Flashing onto ESP32-S3

Install the ESP32 Rust toolchain and flashing utility:

```bash
cargo install espup
espup install
cargo install cargo-espflash
```

Connect Pastille to your computer with a USB-C cable, hold `SW_BOOT`, and flash:

```bash
cd apps/firmware
cargo espflash flash --release --monitor
```

## Landing page (`apps/web`)

Pastille includes a SvelteKit 5 landing page styled with `@facile/muse` and featuring an interactive Three.js 3D PCB and enclosure viewer:

```bash
cd apps/web
bun install
bun run dev
```

## Repository layout

```text
Pastille/
├── assets/
│   ├── logo.svg
│   ├── pcb-2d.png
│   ├── pcb-3d.png
│   └── schematic.png
├── bodies/
│   ├── kit/main/kit.json
│   └── {front_bezel,lens,back_shell,lcd,battery,speaker,lra}.{step,stl}
├── dist/
│   ├── cad/
│   ├── docs/
│   └── fab/
├── docs/
│   └── spec/
│       ├── BOHS-01-ARCH.md
│       ├── BOHS-01-WIRE.md
│       └── BOHS-01-GATEWAY.md
├── hardware/
│   ├── index.tsx
│   ├── subcircuits/
│   ├── bodies/
│   ├── package.json
│   └── tsconfig.json
├── apps/
│   ├── firmware/
│   └── web/
├── Dockerfile
├── docker-compose.yml
├── filet.yml
├── fusible.yml
└── README.md
```

## Contributing and license

Pastille hardware designs and specifications are released under the CERN Open Hardware Licence Version 2 - Strongly Reciprocal (CERN-OHL-S). Firmware is licensed under the Apache 2.0 License.
