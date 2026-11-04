# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.2] - 2026-11-04

### Changed
- **Circular 38 mm PCB Layout & Silkscreen Organization**: Re-arranged all 35 components, mounting holes, fiducials, and board markings (`PASTILLE BOHS-01`, `REV 0.2.2`) across the circular 38 mm (`R = 19 mm`) matte-black PCB so every reference designator and signature sits in an open, unobstructed space with zero overlaps.
- **Updated Hardware Previews & Interactive 3D Models**: Re-baked `apps/web/static/models/pastille.glb`, `apps/web/static/models/pastille_case.glb`, `bodies/pastille_case.*`, and `assets/{pcb-2d,pcb-3d,schematic}.png`.
- **Two-Column Quickstart & Proportional Scroll-Spy**: Expanded `Quickstart` into dual Hardware (`fusible`) and Firmware (`cargo`) cards and replaced the abrupt bottom scroll-spy clamp with a continuous scroll-progress focal line so `Comparison` never gets skipped.

### Fixed
- **3D USB-C Receptacle, JST-PH Battery Header & Boot Switch Models**: Restored the 3D EasyEDA CAD models for `J_USB` (`C2765186`) and `J_BAT` (`C131337`), updated `SW_BOOT` to `pushbutton_w4.2_h4.2` to satisfy `jscad-electronics` rounding constraints, and replaced `TP_TOUCH` with a native 2 mm circular SMT test pad so `circuit-json-to-gltf` exports every 3D component without error.

## [0.2.1] - 2026-11-04

### Added
- **Floating Glassmorphic Topbar & Scroll-Spy Navigation**: Added scroll-reactive top navigation that morphs into a floating blurred pill on scroll, tracks the active section with a smooth sliding indicator, and smoothly scrolls to section anchors.
- **Responsive Mobile Sidebar**: Added slide-in mobile navigation drawer with GSAP open and close animations.
- **Syntax-Highlighted Code Blocks**: Formatted BOHS-01 binary frame layout, Reverse MCP JSON-RPC 2.0 payloads, and quickstart commands with multi-line theme-aware syntax highlighting.
- **Minimal Facile Studio Footer**: Replaced generic legal links with a minimal project footer crediting Facile Studio and linking specifications and releases.

### Changed
- **Tightened Landing Page Copy**: Removed redundant subsystem cards, topbar badge, hero badge, and intro loader caption; tightened section headings and descriptions.
- **Harmonized Topbar Actions**: Replaced generic code icon with the GitHub mark and matched theme toggle and GitHub button dimensions.

### Fixed
- **3D Viewer Control Contrast**: Fixed active and inactive button contrast in both dark and light themes by grouping controls inside a pill surface without overriding primary button accent tokens.
- **Circular PCB Outline & USB-C Pad Routing**: Added 64-vertex circular 38 mm outline (`BOARD_OUTLINE`), black solder mask, `REV 0.2.1` silkscreen, and dual-row USB-C shield/power net bindings in `hardware/index.tsx`.

## [0.2.0] - 2026-11-04

### Added
- **DRV2605L Haptic Subsystem**: Added `U_HAP` (`DRV2605LDGSR` VSSOP-10), `C_HAP` (100nF), `C_HAP_REG` (1uF REG bypass), 4.7k I2C pull-up resistors (`R_SDA`, `R_SCL` on `GPIO8`/`GPIO9`), and LRA motor terminal header (`J_HAP`).
- **Schottky Power-Path Load Sharing & USB ESD Protection**: Added `D_TVS` (`SMF5.0A` VBUS TVS diode), `R_USB_DM`/`R_USB_DP` (22-ohm USB D+/D- series termination to `GPIO19`/`GPIO20`), dual `B5819W` Schottky power-path diodes (`D_SYS`, `D_PATH`) isolating `VSYS` from `VBAT` so USB powers system load without preventing TP4056 charge termination, battery ADC divider (`R_BATT_TOP` 100k, `R_BATT_BOT` 100k, `C_BATT_SENSE` 100nF on `GPIO1`), and `CHRG` status pull-up (`R_CHG_PU` 10k on `GPIO2`).
- **Hardware Reset, Boot Button & DFM Features**: Replaced invalid `qfp32` footprint with `qfn32_w6_h6_p0.5` (`ESP32-S3R8`), added `CHIP_PU` power-on reset RC delay network (`R_PU` 10k, `C_PU` 1uF), tactile `SW_BOOT` switch (`TL3305AF160QG` on `GPIO0`), 100k `SD_MODE` pull-down (`R_AMP_SD`) on `MAX98357A`, 510-ohm series ESD/noise damping resistor (`R_TOUCH`) on `TOUCH_WAKE` (`GPIO7`), M2 mounting holes (`2.2mm`), three optical SMT fiducials (`FID1`-`FID3`), and silkscreen revision markings.
- **5-Stage Power State Machine & Full BOHS-01 Protocol**: Implemented `PowerManager` (`DeepSleep`, `BootWake`, `WifiFastAssociate`, `StreamFullDuplex`, `SleepGrace` with 30s timeout and battery voltage telemetry), end-of-utterance (`FLAG_END_OF_UTTERANCE`) detection, `charm.snapshot` capability error (`-32601`), `charm.tap_event` and `charm.low_battery` JSON-RPC 2.0 notifications, and strict parameter validation.
- **SvelteKit 5 Landing Page (`apps/web`)**: Built borderless `@facile/muse` landing page with `solar:circle-dot-bold-duotone` identity, concentric GSAP entrance loader, scroll-reveal animations, and interactive Three.js 3D PCB and enclosure viewer.
- **PCB & Schematic Previews**: Added high-resolution 2D PCB, 3D PCB, schematic previews, and SVG mark in `assets/`.

### Fixed
- **Xtensa Firmware Entrypoint**: Fixed non-compiling `xtensa` `main()` that referenced undefined `make_static!`, bound `gpio3` instead of `TOUCH_WAKE` (`GPIO7`), and unconditionally entered deep sleep before starting the Embassy executor.
- **Audio Capture Gain & Integer Math**: Applied `mic_gain` scaling in `AudioPipeline::capture_frame` and replaced manual Newton-Raphson loop with standard library `u64::isqrt`.
- **Enclosure Height**: Reduced enclosure height in `fusible.yml` from `12.0 mm` to `11.5 mm` to comply with `BOHS-01-ARCH.md` Section 2.1.

## [0.1.0] - 2026-11-04

### Added
- **Bulle Open Hardware Standard (BOHS-01)**: Comprehensive formal specifications in `docs/spec/` defining low-latency audio streaming (Opus 16kHz, 20ms frames) and bidirectional Reverse MCP tool execution over WebSocket (`bohs.v1`).
- **Physical Circuit Design**: Declarative 38mm 4-layer circular PCB designed with `tscircuit` and Fusible, featuring an ESP32-S3 (8MB PSRAM), USB-C charging with TP4056, ultra-low quiescent 3.3V LDO, I2S INMP441 MEMS microphone, I2S MAX98357A amplifier, GC9A01 1.28" round LCD connector, and capacitive touch wake pad.
- **Electrical Rule Checking**: Verified with Fusible's fail-closed Fuse Contract with zero DRC violations, zero strapping pin collisions, and verified decoupling capacitor bypass rules.
- **Bare-Metal Embassy Firmware**: Asynchronous Rust runtime for the ESP32-S3 with integer trigonometric vector rendering for Bulle's circular avatar states (`Idle`, `Listening`, `Thinking`, `Speaking`, `Error`), I2S DMA audio pipeline, and BOHS-01 protocol dispatcher.
- **Mechanical Enclosures**: Generated snap-fit 3D case models (`bodies/pastille_case.{step,stl,glb,brep}`) and export presets.
