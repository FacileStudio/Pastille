export type TokenTone = "fg" | "muted" | "key" | "str" | "num" | "kw";

export interface CodeSpan {
  text: string;
  tone?: TokenTone;
}

export type CodeLine = CodeSpan[];

export interface Metric {
  label: string;
  value: string;
  detail: string;
}

export interface FeatureItem {
  icon: string;
  title: string;
  description: string;
  tag: string;
}

export interface ComparisonRow {
  metric: string;
  pastille: string;
  omi: string;
  rabbit: string;
}

export interface ProtocolPlane {
  opcode: string;
  name: string;
  summary: string;
  lines: CodeLine[];
}

export const NAV_SECTIONS = [
  { id: "viewer", label: "3D Board" },
  { id: "connectivity", label: "Connectivity" },
  { id: "architecture", label: "Architecture" },
  { id: "protocol", label: "Protocol" },
  { id: "comparison", label: "Comparison" },
  { id: "quickstart", label: "Quickstart" },
] as const;

export const HERO_METRICS: Metric[] = [
  { label: "Diameter", value: "38.0 mm", detail: "4-layer ENIG PCB, 11.5 mm case" },
  { label: "Deep sleep", value: "18 uA", detail: "18+ days standby · 24h BLE talk (target)" },
  { label: "Wake time", value: "< 6 ms", detail: "BLE 5.0 2M PHY & <50 ms RTC wake" },
  { label: "Unit BOM", value: "$23.85", detail: "1,000-unit PCBA turnkey" },
];

export const FEATURES: FeatureItem[] = [
  {
    icon: "solar:cpu-bolt-bold-duotone",
    title: "ESP32-S3R8 MCU",
    description: "240 MHz dual-core Xtensa LX7 with 8 MB octal PSRAM and native USB-C JTAG.",
    tag: "Embassy Rust",
  },
  {
    icon: "solar:soundwave-bold-duotone",
    title: "Full-duplex I2S audio",
    description: "SPH0645LM4H-B / INMP441 I2S MEMS mic and MAX98357A Class-D amp with on-chip VAD.",
    tag: "PCM16 16 kHz · 20 ms",
  },
  {
    icon: "solar:eye-bold-duotone",
    title: "Round IPS display",
    description: "1.28-inch 240x240 GC9A01 panel driven over 40 MHz SPI at 60 FPS.",
    tag: "Integer vector UI",
  },
  {
    icon: "solar:magic-stick-3-bold-duotone",
    title: "LRA haptics & touch",
    description: "DRV2605L I2C actuator driver and ESD-damped capacitive pad on RTC GPIO 7.",
    tag: "< 50 ms wake",
  },
  {
    icon: "solar:battery-charge-bold-duotone",
    title: "Schottky load sharing",
    description: "TP4056 charger, dual B5819W power-path diodes, SMF5.0A TVS, and 1 uA 3.3 V LDO.",
    tag: "Safe charge cutoff",
  },
  {
    icon: "solar:shield-keyhole-bold-duotone",
    title: "Dual BLE 5.0 & Wi-Fi WSS",
    description: "One-tap BLE pairing in Bulle Mobile/Desktop with automatic Wi-Fi WSS roaming.",
    tag: "bohs.v1 · CERN-OHL-S",
  },
];

export const COMPARISON: ComparisonRow[] = [
  { metric: "Form factor", pastille: "38 mm disc (14 g)", omi: "40 mm pendant", rabbit: "Handheld slab (115 g)" },
  { metric: "Unit BOM", pastille: "$23.85 open BOM", omi: "~$45 BOM", rabbit: "$199 retail" },
  { metric: "Display", pastille: '1.28" 240x240 IPS', omi: "None", rabbit: '2.88" TFT' },
  { metric: "Standby life", pastille: "18+ days (18 uA)", omi: "~4 days", rabbit: "~12 hours" },
  { metric: "Runtime", pastille: "Bare-metal Rust", omi: "BLE to phone app", rabbit: "Android AOSP" },
  { metric: "Wake latency", pastille: "< 50 ms", omi: "BLE pairing delay", rabbit: "> 45 s cold boot" },
  { metric: "Data custody", pastille: "Self-hosted Bulle", omi: "Companion app", rabbit: "Vendor cloud" },
];

const s = (text: string, tone?: TokenTone): CodeSpan => ({ text, tone });

export const PROTOCOL_PLANES: ProtocolPlane[] = [
  {
    opcode: "0x01 · Binary audio frame",
    name: "PCM16 16 kHz audio plane",
    summary:
      "Each 20 ms PCM16 voice frame carries a 4-byte header over BLE GATT (B002) or Wi-Fi WSS. " +
      "BLE and USB add a u16be length prefix and splits frames into MTU-sized chunks.",
    lines: [
      [s("0x00  ", "muted"), s("opcode    ", "key"), s("0x01        ", "num"), s("# Audio data frame", "muted")],
      [s("0x01  ", "muted"), s("flags     ", "key"), s("0b0000_0011 ", "num"), s("# bit0: EOU, bit1: VAD", "muted")],
      [s("0x02  ", "muted"), s("seq       ", "key"), s("u16be       ", "kw"), s("# big-endian frame counter", "muted")],
      [s("0x04  ", "muted"), s("payload   ", "key"), s("[u8; 640]   ", "kw"), s("# PCM16 LE, 16 kHz, 20 ms", "muted")],
    ],
  },
  {
    opcode: "0x02 / 0x03 · Binary control & pairing frame",
    name: "Reverse MCP control plane",
    summary:
      "Opcode 0x02 dispatches Reverse MCP edge tools; Opcode 0x03 pairs and syncs tokens over BLE (B004).",
    lines: [
      [s("{", "muted")],
      [s('  "jsonrpc"', "key"), s(': "2.0",', "str")],
      [s('  "id"', "key"), s(": 42,", "num")],
      [s('  "method"', "key"), s(": ", "muted"), s('"tools/call"', "str"), s(",", "muted")],
      [s('  "params"', "key"), s(": {", "muted")],
      [s('    "name"', "key"), s(": ", "muted"), s('"charm.display_avatar"', "str"), s(",", "muted")],
      [
        s('    "arguments"', "key"),
        s(": { ", "muted"),
        s('"state"', "key"),
        s(': "thinking"', "str"),
        s(" }", "muted"),
      ],
      [s("  }", "muted")],
      [s("}", "muted")],
    ],
  },
];

export interface QuickstartStep {
  badge: string;
  title: string;
  summary: string;
  lines: CodeLine[];
}

export const QUICKSTART_STEPS: QuickstartStep[] = [
  {
    badge: "01 · Hardware & CAD",
    title: "Compile circuit, enclosure, and Gerbers",
    summary: "Validate ERC/DRC rules, build the snap-fit case, and export manufacturing files.",
    lines: [
      [
        s("$ ", "muted"),
        s("git ", "key"),
        s("clone ", "kw"),
        s("https://github.com/FacileStudio/Pastille.git", "str"),
      ],
      [s("$ ", "muted"), s("cd ", "key"), s("Pastille && ", "fg"), s("fusible ", "key"), s("check", "kw")],
      [
        s("$ ", "muted"),
        s("fusible ", "key"),
        s("body", "kw"),
        s("                      # STEP, STL, glTF case", "muted"),
      ],
      [
        s("$ ", "muted"),
        s("fusible ", "key"),
        s("export ", "kw"),
        s("--preset all", "str"),
        s("       # Gerbers, KiCad, BOM", "muted"),
      ],
    ],
  },
  {
    badge: "02 · Embassy Firmware",
    title: "Test and flash bare-metal Rust",
    summary: "Run host unit tests for the BOHS-01 stack or flash directly over USB-C JTAG.",
    lines: [
      [s("$ ", "muted"), s("cd ", "key"), s("apps/firmware", "fg")],
      [
        s("$ ", "muted"),
        s("cargo ", "key"),
        s("test", "kw"),
        s("                        # VAD, wire, MCP, avatar", "muted"),
      ],
      [s("$ ", "muted"), s("cargo ", "key"), s("clippy ", "kw"), s("--all-targets", "num")],
      [s("$ ", "muted"), s("cargo ", "key"), s("espflash flash ", "kw"), s("--release --monitor", "num")],
    ],
  },
];
