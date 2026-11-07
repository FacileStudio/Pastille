# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- The top bar and the footer are muse's `LandingHeader` and `LandingFooter` (muse v0.14.2). The bar shrinks into a floating pill on scroll and opens a right-hand drawer under 1024 px. The footer lists the spec, releases and GitHub, with the licence line as its copyright.
- The bar shows six links (Inside, Board, Hardware, Protocol, Build, Pre-order) so it fits at 1024 px. Connectivity and Comparison stay on the page, and the active pill follows the section above them.

### Removed
- The hand-rolled `SiteHeader`, `NavDrawer` and `moveNavPill`.

## [0.7.0] - 2026-11-07

### Added
- `examples/bohs-switch`: a BOHS-01 field device for a Raspberry Pi. It pairs with Bulle by a typed code and switches a lamp on a relay when you ask. It ships an install script for Raspberry Pi OS and an rpi-image-gen image.
- The BOHS-01 spec has a core and two profiles, charm and field. It adds code pairing, the `0x10` switch capability with `switch.set` and `switch.get`, and the `<device>__<tool>` names the gateway shows the model.

## [0.6.3] - 2026-11-06

v0.6.2 was tagged but never published: its new strict bodies check failed on CI because STL tessellation differs between machines. v0.6.3 ships everything below plus that fix.

### Added
- Pre-order entry in the top bar and the mobile drawer.
- The release job fails unless the tag matches every version copy: both `package.json` files, `Cargo.toml` and `Cargo.lock`.
- `bun run check` runs the explode math and pre-order checks, and fails on any unregistered icon name.

### Changed
- Pre-order copy now tells one reservation story. You pay nothing now, we email to confirm, and payment details and shipping cost come later by email. Price and ship date come from one constant, labelled as the pre-order price and the estimated ship date.
- The page reads its version from `apps/web/package.json`, so it can no longer show a stale release.
- The header collapses into the drawer below 1024 px and shows nav icons from 1280 px. Header buttons are 44 px again.
- STEP and STL writers round every float, replacing a hand patch of one literal. Bun is pinned to 1.4.2 in mise and CI, and cargo runs with `--locked`.

### Fixed
- The header no longer overflows the page between 768 and 900 px.
- The CI bodies check catches geometry changes again and covers STL files. Before this, it ignored every `CARTESIAN_POINT` line.
- The bodies check compares STL files by geometry (volume, area and bounding box within 0.1 percent) and keeps STEP files byte-strict, so CI passes on any machine and still fails on a real shape change.
- Board model overlay parts sit where the circuit puts them, and the ESP32-S3-MINI-1 uses its real 20.5 x 15.4 x 2.4 mm size.
- The disassembly falls back to the assembly model if the board model fails to load, and frees the parts it replaces.
- Under reduced motion the step cards no longer animate.

### Removed
- Unused `ScreenBlob.svelte` and the per-frame screen projection.
- Pre-order promises nothing in the repo backs: "final quote", "lead time", "payment link" and "Tracked worldwide".

## [0.6.1] - 2026-11-05

### Fixed
- Stacked disassembly cards no longer throw a hard black band across the card above: the shadow is now a soft two-layer shadow that follows the light and dark theme.

## [0.6.0] - 2026-11-05

### Added
- Pre-order form split into a summary card and a grouped form with a live order recap (quantity, total, ship date, amount due) and a three-step "what happens next" strip.
- Generated Solar icon registry built from `@iconify-json/solar`; `bun run check` fails when a `solar:` name used in `src/` is missing from the set.
- Footer links now carry Solar icons.

### Changed
- Top bar nav, theme toggle and mobile drawer use Solar bold duotone glyphs. Board, Connectivity and Build rendered no icon before because their names were never registered; every icon now renders inline instead of falling back to the iconify network loader.
- Pre-order validation shows inline errors with `aria-invalid` and `aria-describedby`, and the success panel offers a copy button and the plain address when no mail client opens.

### Removed
- Hand-pasted icon modules `icons-solar.ts`, `icons-explode.ts` and `topology-icons.ts`, replaced by the generated registry.

## [0.5.2] - 2026-11-05

### Changed
- Hero copy now reads "Open hardware. Sovereign AI." with the new description.
- Pre-order button, hero and nav links use Solar icons, and the hero button smooth-scrolls to the form.
- Pre-order inputs drop their borders. The send button has an icon.
- Footer is a pill with inner padding.
- Battery comparison cell shortened, and quickstart badges drop their step numbers.

## [0.5.1] - 2026-11-05

### Changed
- Pre-order form redesigned as a two-column block with the price, and moved to the bottom of the landing page.
- Stacked disassembly cards cast their shadow upward and sink into a pile under the active card.
- `@facile/blob` bumped from v0.1.0 to v0.2.1.

## [0.5.0] - 2026-11-05

### Added
- Pre-order button on the landing page and a pre-order form for the pre-built Pastille at $49. Submitting opens a mail draft to contact@facile.studio. The full release ships in March 2027.

## [0.4.0] - 2026-11-05

### Added
- Printable enclosure CAD (back shell, front bezel, lens, internal pockets) with a 14.0 mm assembled thickness and automated clearance verification.
- ESP32-S3 device firmware image (1.35 MB) targeting the ESP32-S3-MINI-1 module with esp-hal 1.2 and esp-rtos.
- BLE GATT server with LE Secure Connections bonding and Wi-Fi TLS 1.3 WebSocket client connecting to the Bulle gateway.
- Full-duplex I2S audio streaming for the SPH0645 microphone and MAX98357A amplifier, plus SNTP wall-clock synchronization.
- Capacitive touch support via RTC IO registers and wear-levelled settings storage with flash encryption support.
- Complete fabrication outputs: 4-layer Gerbers with ground pour, BOM, and pick-and-place CPL.
- Interactive scroll disassembly section and social share cards on the landing page.

### Changed
- Moved hardware design to the ESP32-S3-MINI-1-N4R2 module, 600 mA regulator, 12-pin LCD flex connector, and SMD-only components.
- Adjusted battery specifications to a protected 120 mAh cell (about 1.3 hours of continuous talk, about 3.5 days at 20 minutes of talk a day, about 80 days asleep).
- Resolved all PCB placement errors; board autorouting now completes with clean copper spans.
- Web viewer now lazy-loads models on demand and serves GLB assets with gzip compression.

## [0.3.0] - 2026-11-04

### Added
- BOHS-01 now runs over BLE 5.0 GATT, Wi-Fi WSS and USB-C. The spec defines a 128-bit GATT service with four characteristics, and a u16be length prefix with chunking for BLE and USB.
- Opcode `0x03` provisioning (`prov.identify`, `prov.status`, `prov.configure`): one-tap pairing from the Bulle app, with a 60 s tap window and an encrypted BLE link required before the device accepts its token or Wi-Fi credentials.
- Firmware modules `ble.rs`, `prov.rs` and `stream.rs` with host tests, an outbound packet queue, and the `BleFastConnect` power state.
- Landing page connectivity section with an animated topology diagram for the BLE relay, direct Wi-Fi and pairing modes.
- `mise.toml` with toolchain pins and `check`, `dev`, `drc`, `export` and `flash` tasks.

### Changed
- Audio frames are documented as what the firmware sends: PCM16 LE, 16 kHz, 640-byte payload behind a 4-byte header. No Opus.
- Battery figures for BLE and Wi-Fi streaming are labelled as targets.
- `tools/list` no longer advertises `charm.snapshot`, so the response fits the 1024-byte frame limit.
- Release button on the landing page now reads "Enclosure CAD", which matches the attached files.

### Fixed
- Charger `U_CHG` is the TP4056 its pinout matches, and `R_PROG` is 3.6k, which sets charge current to about 333 mA (0.95C) instead of 600 mA.
- Amp `U_AMP` uses the real MAX98357A TQFN-16 footprint and pinout, with both VDD pins and every GND pin plus the exposed pad wired.
- Haptic driver `U_HAP` uses the DRV2605L VSSOP-10 footprint and drives the LRA differentially, as the datasheet requires. `C_HAP` is now 1 uF.
- Firmware inbound packets are 1024 bytes, so a full 644-byte audio frame no longer fails to decode. JSON-RPC responses are now queued instead of dropped.
- The landing page protocol diagram matches the wire spec and the firmware.
- PCB viewer ignores stale model loads after quick mode switches.
- `bom.yml` is populated with keys fusible matches.

### Known issues
- The PCB has 10 placement errors carried over from v0.2.2, so tscircuit skips autorouting and the board has no traces. No Gerbers are attached to this release. v0.4.0 fixes placement and routing.

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
