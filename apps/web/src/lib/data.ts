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
  { id: "architecture", label: "Architecture" },
  { id: "protocol", label: "Protocol" },
  { id: "comparison", label: "Comparison" },
  { id: "quickstart", label: "Quickstart" },
] as const;

export const HERO_METRICS: Metric[] = [
  { label: "Diameter", value: "38.0 mm", detail: "4-layer ENIG PCB, 11.5 mm case" },
  { label: "Deep sleep", value: "18 uA", detail: "14+ days standby on 250 mAh LiPo" },
  { label: "Wake time", value: "< 50 ms", detail: "Capacitive RTC touch wake" },
  { label: "Unit BOM", value: "$23.85", detail: "100-unit PCBA turnkey" },
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
    description: "INMP441 MEMS microphone and MAX98357A Class-D amplifier with on-chip VAD.",
    tag: "Opus 16 kHz · 20 ms",
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
    title: "Private Bulle gateway",
    description: "Streams over bohs.v1 WebSocket with Casier bearer tokens and zero cloud relay.",
    tag: "CERN-OHL-S v2",
  },
];

export const COMPARISON: ComparisonRow[] = [
  { metric: "Form factor", pastille: "38 mm disc (14 g)", omi: "40 mm pendant", rabbit: "Handheld slab (115 g)" },
  { metric: "Unit BOM", pastille: "$23.85 open BOM", omi: "~$45 BOM", rabbit: "$199 retail" },
  { metric: "Display", pastille: '1.28" 240x240 IPS', omi: "None", rabbit: '2.88" TFT' },
  { metric: "Standby life", pastille: "14+ days (18 uA)", omi: "~4 days", rabbit: "~12 hours" },
  { metric: "Runtime", pastille: "Bare-metal Rust", omi: "BLE to phone app", rabbit: "Android AOSP" },
  { metric: "Wake latency", pastille: "< 50 ms", omi: "BLE pairing delay", rabbit: "> 45 s cold boot" },
  { metric: "Data custody", pastille: "Self-hosted Bulle", omi: "Companion app", rabbit: "Vendor cloud" },
];

export const PROTOCOL_PLANES: ProtocolPlane[] = [
  {
    opcode: "0x01 · Binary frame",
    name: "Opus 16 kHz audio plane",
    summary: "Each 20 ms voice frame prepends an 8-byte header with VAD and end-of-utterance flags.",
    lines: [
      [
        { text: "0x00  ", tone: "muted" },
        { text: "magic     ", tone: "key" },
        { text: "0xB0        ", tone: "num" },
        { text: "# BOHS sync byte", tone: "muted" },
      ],
      [
        { text: "0x01  ", tone: "muted" },
        { text: "version   ", tone: "key" },
        { text: "0x01        ", tone: "num" },
        { text: "# bohs.v1", tone: "muted" },
      ],
      [
        { text: "0x02  ", tone: "muted" },
        { text: "flags     ", tone: "key" },
        { text: "0b0000_0011 ", tone: "num" },
        { text: "# bit0: VAD, bit1: EOU", tone: "muted" },
      ],
      [
        { text: "0x03  ", tone: "muted" },
        { text: "seq       ", tone: "key" },
        { text: "u16be       ", tone: "kw" },
        { text: "# frame counter", tone: "muted" },
      ],
      [
        { text: "0x05  ", tone: "muted" },
        { text: "length    ", tone: "key" },
        { text: "u16be       ", tone: "kw" },
        { text: "# payload bytes", tone: "muted" },
      ],
      [
        { text: "0x07  ", tone: "muted" },
        { text: "reserved  ", tone: "key" },
        { text: "0x00", tone: "num" },
      ],
      [
        { text: "0x08  ", tone: "muted" },
        { text: "payload   ", tone: "key" },
        { text: "[u8; N]     ", tone: "kw" },
        { text: "# 16 kHz Opus packet", tone: "muted" },
      ],
    ],
  },
  {
    opcode: "0x02 · Text frame",
    name: "Reverse MCP control plane",
    summary:
      "Bulle calls edge tools over the same socket to set expressions, trigger haptics, or read voltage.",
    lines: [
      [{ text: "{", tone: "muted" }],
      [
        { text: '  "jsonrpc"', tone: "key" },
        { text: ': "2.0",', tone: "str" },
      ],
      [
        { text: '  "id"', tone: "key" },
        { text: ": 42,", tone: "num" },
      ],
      [
        { text: '  "method"', tone: "key" },
        { text: ": ", tone: "muted" },
        { text: '"tools/call"', tone: "str" },
        { text: ",", tone: "muted" },
      ],
      [{ text: '  "params"', tone: "key" }, { text: ": {", tone: "muted" }],
      [
        { text: '    "name"', tone: "key" },
        { text: ": ", tone: "muted" },
        { text: '"charm.display_avatar"', tone: "str" },
        { text: ",", tone: "muted" },
      ],
      [
        { text: '    "arguments"', tone: "key" },
        { text: ": { ", tone: "muted" },
        { text: '"state"', tone: "key" },
        { text: ": ", tone: "muted" },
        { text: '"thinking"', tone: "str" },
        { text: " }", tone: "muted" },
      ],
      [{ text: "  }", tone: "muted" }],
      [{ text: "}", tone: "muted" }],
    ],
  },
];

export const QUICKSTART_LINES: CodeLine[] = [
  [
    { text: "$ ", tone: "muted" },
    { text: "git ", tone: "key" },
    { text: "clone ", tone: "kw" },
    { text: "https://github.com/FacileStudio/Pastille.git", tone: "str" },
    { text: " && ", tone: "muted" },
    { text: "cd ", tone: "key" },
    { text: "Pastille", tone: "fg" },
  ],
  [
    { text: "$ ", tone: "muted" },
    { text: "fusible ", tone: "key" },
    { text: "check", tone: "kw" },
    { text: "                     # ERC, DRC, and pin contract", tone: "muted" },
  ],
  [
    { text: "$ ", tone: "muted" },
    { text: "fusible ", tone: "key" },
    { text: "export ", tone: "kw" },
    { text: "--preset ", tone: "num" },
    { text: "all", tone: "str" },
    { text: "       # Gerbers, BOM, STEP, glTF", tone: "muted" },
  ],
  [
    { text: "$ ", tone: "muted" },
    { text: "cargo ", tone: "key" },
    { text: "test ", tone: "kw" },
    { text: "--manifest-path ", tone: "num" },
    { text: "apps/firmware/Cargo.toml", tone: "str" },
  ],
];
