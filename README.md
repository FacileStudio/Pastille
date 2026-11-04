# Pastille

Pastille is an open-source wearable AI charm and the reference hardware implementation of the Bulle Open Hardware Standard (BOHS-01).

It is a 38 mm disc that clips to clothing or hangs as a pendant. Pastille captures your voice with digital microphones, streams bidirectional audio over WebSocket to your private Bulle gateway, displays animated companion expressions on a round LCD, and provides tactile haptic feedback.

Pastille costs under $25 to assemble, stays cool against your skin, wakes from deep sleep in under 50 ms, and lasts over two weeks on a single charge.

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
| Target BOM cost | < $25 | ~ $18 | ~ $45 | ~ $199 retail |
| Display | 1.28-inch round IPS LCD (240x240) | None | None | 2.88-inch touchscreen |
| Battery life (standby) | > 14 days (18 uA deep sleep) | > 20 days | ~ 4 days | ~ 12 hours |
| Battery life (streaming) | > 8 hours active talk | > 12 hours | ~ 6 hours | ~ 4 hours |
| System architecture | Sensor / Actuator peripheral | Sensor peripheral | BLE audio streamer | Full Android OS (MediaTek MT6765) |
| Onboard operating system | None (Bare-metal Rust / FreeRTOS) | None | None | Full Android AOSP fork |
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

Install `bun` and project dependencies:

```bash
cd hardware
bun install
```

### Interactive development

Launch the Fusible live preview server:

```bash
cd hardware
bun run dev
```

Open `http://localhost:3020` in your browser to inspect the interactive 2D schematic, 3D PCB layout, and autorouted copper traces in real time.

### Exporting manufacturing artifacts

Export production-ready Gerber files, drill files, bill of materials tables, and pick-and-place files for PCB fabrication:

```bash
cd hardware
bun run build
```

The output files land in `hardware/dist/` ready for standard turnkey assembly houses.

## Firmware flashing guide

Pastille firmware runs on the ESP32-S3 using Rust (`esp-idf-hal`).

### Prerequisites

Install the ESP32 Rust toolchain and flashing utility:

```bash
cargo install espup
espup install
cargo install cargo-espflash
```

### Configuration

Set your Wi-Fi credentials and Bulle gateway endpoint in your environment or via Casier:

```bash
export PASTILLE_WIFI_SSID="YourNetworkName"
export PASTILLE_WIFI_PASS="YourNetworkPassword"
export PASTILLE_GATEWAY_URL="wss://gateway.bulle.local/api/hardware/ws"
export PASTILLE_DEVICE_TOKEN="bohs_tok_e814a729c15d4872"
```

### Build and flash

Connect Pastille to your computer with a USB-C cable, hold the boot button, and flash:

```bash
cargo espflash flash --release --monitor
```

The command compiles the firmware, writes it to the ESP32-S3 flash partition, and opens the serial monitor. You will see the RTC boot sequence, Wi-Fi fast association, and the established `bohs.v1` WebSocket session.

## Repository layout

```
Pastille/
├── docs/
│   └── spec/
│       ├── BOHS-01-ARCH.md
│       ├── BOHS-01-WIRE.md
│       └── BOHS-01-GATEWAY.md
├── hardware/
│   ├── index.tsx
│   ├── package.json
│   └── tsconfig.json
├── apps/
│   └── firmware/
├── fusible.yml
└── README.md
```

## Contributing and license

Pastille hardware designs and specifications are released under the CERN Open Hardware Licence Version 2 - Strongly Reciprocal (CERN-OHL-S). Firmware is licensed under the Apache 2.0 License.
