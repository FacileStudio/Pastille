# Pastille

<p align="center">
  <img src="assets/logo.svg" alt="Pastille Logo" width="72" height="72" />
</p>

Pastille is an open-source wearable AI charm and the reference hardware implementation of the Bulle Open Hardware Standard (BOHS-01).

It is a 38 mm disc that clips to clothing or hangs as a pendant. Pastille captures your voice with digital microphones, streams bidirectional audio over WebSocket to your private Bulle gateway, displays animated companion expressions on a round LCD, and provides tactile haptic feedback.

Pastille costs under $25 to assemble, stays cool against your skin, wakes from deep sleep in under 50 ms, and lasts over two weeks on a single charge.

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

- [BOHS-01 Architecture Specification](docs/spec/BOHS-01-ARCH.md): System architecture, 38 mm physical geometry, thermal envelope under 40°C, $23.85 BOM, and 5-stage power state machine.
- [BOHS-01 Wire Protocol Specification](docs/spec/BOHS-01-WIRE.md): Multiplexed WebSocket protocol (`bohs.v1`), Opus 16 kHz audio streaming (Opcode `0x01`), and Reverse MCP edge tool execution (Opcode `0x02`).
- [BOHS-01 Gateway Integration Guide](docs/spec/BOHS-01-GATEWAY.md): Bulle server integration, mutual token authentication via Registre and Casier, Whisper.cpp ingestion, Piper TTS egress, and LLM edge tool calling.

## Architectural comparison

Standalone AI hardware often fails because designers build miniature smartphones running general-purpose operating systems like Android. BOHS-01 takes the opposite approach: dumb peripheral, intelligent gateway.

| Characteristic | Pastille (BOHS-01) | Muse Charm | OMI (Friend) | Rabbit R1 |
| :--- | :--- | :--- | :--- | :--- |
| Form factor | 38 mm circular disc | Screenless brooch | 40 mm circular pendant | Handheld rectangular slab |
| Target BOM cost | < $25 ($23.85) | ~ $18 | ~ $45 | ~ $199 retail |
| Display | 1.28-inch round IPS LCD (240x240) | None | None | 2.88-inch touchscreen |
| Battery life (standby) | > 14 days (18 uA deep sleep) | > 20 days | ~ 4 days | ~ 12 hours |
| Battery life (streaming) | > 8 hours active talk | > 12 hours | ~ 6 hours | ~ 4 hours |
| System architecture | Sensor / Actuator peripheral | Sensor peripheral | BLE audio streamer | Full Android OS (MediaTek MT6765) |
| Onboard operating system | None (Bare-metal Rust / Embassy) | None | None | Full Android AOSP fork |
| Boot / Wake latency | < 50 ms to wake, < 800 ms to audio | < 100 ms | Variable BLE pairing | > 45 seconds cold boot |
| Server protocol | WebSocket (`bohs.v1`) + Reverse MCP | WebSocket | BLE to smartphone app | Proprietary cloud bridge |
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

Generate the 3D snap-fit enclosure (`bodies/pastille_case.{step,stl,glb,brep}`) and export production-ready Gerbers, KiCad project files, STEP models, and vector schematics:

```bash
fusible body
fusible export --preset all
```

The output files land in `dist/fab/`, `dist/cad/`, `dist/docs/`, and `bodies/`.

## Firmware guide

Pastille firmware runs bare-metal on the ESP32-S3 using Rust (`embassy-executor`, `esp-hal`, and `esp-wifi`) with pin bindings generated directly by Fusible (`apps/firmware/src/pins.rs`).

### Host unit tests

Run the host test suite covering the BOHS-01 wire protocol, Reverse MCP tool validation, I2S DMA audio VAD pipeline, 5-stage power state machine, and integer trigonometric avatar renderer:

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
│   └── pastille_case.{step,stl,glb,brep}
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
