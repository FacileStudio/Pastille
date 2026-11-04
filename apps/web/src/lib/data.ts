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
  { id: "disassembly", label: "Inside" },
  { id: "viewer", label: "Board" },
  { id: "connectivity", label: "Connectivity" },
  { id: "architecture", label: "Hardware" },
  { id: "protocol", label: "Protocol" },
  { id: "comparison", label: "Comparison" },
  { id: "quickstart", label: "Build" },
] as const;

export const BATTERY = {
  cell: "402020",
  capacity: "150 mAh",
  chargeCurrent: "about 146 mA",
};

export const HERO_METRICS: Metric[] = [
  { label: "Size", value: "42 mm", detail: "17 mm thick, with a lanyard loop" },
  { label: "Talk time", value: "10 h", detail: "Over Bluetooth (design target)" },
  { label: "Standby", value: "7 days", detail: "On one charge (design target)" },
  { label: "Parts cost", value: "$23.85", detail: "Per unit, at 1,000 units" },
];

export const FEATURES: FeatureItem[] = [
  {
    icon: "solar:cpu-bolt-bold-duotone",
    title: "ESP32-S3 chip",
    description: "A 240 MHz dual-core chip with 8 MB of memory, Bluetooth and Wi-Fi. It runs bare-metal Rust.",
    tag: "Embassy Rust",
  },
  {
    icon: "solar:soundwave-bold-duotone",
    title: "Mic and speaker",
    description: "A MEMS mic hears you and a small amp talks back. Both run at once, so you can interrupt Bulle.",
    tag: "16 kHz voice",
  },
  {
    icon: "solar:eye-bold-duotone",
    title: "Round screen",
    description: "A 1.28 inch, 240x240 display where Bulle's face lives. It redraws at 60 FPS.",
    tag: "GC9A01 over SPI",
  },
  {
    icon: "solar:magic-stick-3-bold-duotone",
    title: "Touch and haptics",
    description: "Tap the pad to wake Pastille. A small vibration motor taps back for alerts.",
    tag: "Wakes in under 50 ms",
  },
  {
    icon: "solar:battery-charge-bold-duotone",
    title: "Battery and charging",
    description: "A 150 mAh cell charges from USB-C in about 90 minutes. A TVS diode guards the port.",
    tag: "TP4056 at 146 mA",
  },
  {
    icon: "solar:shield-keyhole-bold-duotone",
    title: "Bluetooth and Wi-Fi",
    description:
      "Pair it in one tap from the Bulle app. Out and about it talks through your phone. At home it uses Wi-Fi.",
    tag: "bohs.v1 · CERN-OHL-S",
  },
];

export const COMPARISON: ComparisonRow[] = [
  { metric: "Form factor", pastille: "42 mm disc", omi: "40 mm pendant", rabbit: "Handheld slab (115 g)" },
  { metric: "Unit BOM", pastille: "$23.85 open BOM", omi: "~$45 BOM", rabbit: "$199 retail" },
  { metric: "Display", pastille: '1.28" 240x240 IPS', omi: "None", rabbit: '2.88" TFT' },
  { metric: "Standby life", pastille: "7+ days (18 uA)", omi: "~4 days", rabbit: "~12 hours" },
  { metric: "Runtime", pastille: "Bare-metal Rust", omi: "BLE to phone app", rabbit: "Android AOSP" },
  { metric: "Wake latency", pastille: "< 50 ms", omi: "BLE pairing delay", rabbit: "> 45 s cold boot" },
  { metric: "Your data", pastille: "Your own server", omi: "Companion app", rabbit: "Vendor cloud" },
];

const s = (text: string, tone?: TokenTone): CodeSpan => ({ text, tone });

export const PROTOCOL_PLANES: ProtocolPlane[] = [
  {
    opcode: "0x01 · Audio frame",
    name: "Voice",
    summary:
      "Every 20 ms of voice is one frame: a 4-byte header, then 640 bytes of PCM16. " +
      "Bluetooth and USB add a 2-byte length so a frame can be split into packets.",
    lines: [
      [s("0x00  ", "muted"), s("opcode    ", "key"), s("0x01        ", "num"), s("# Audio data frame", "muted")],
      [s("0x01  ", "muted"), s("flags     ", "key"), s("0b0000_0011 ", "num"), s("# bit0: EOU, bit1: VAD", "muted")],
      [s("0x02  ", "muted"), s("seq       ", "key"), s("u16be       ", "kw"), s("# big-endian frame counter", "muted")],
      [s("0x04  ", "muted"), s("payload   ", "key"), s("[u8; 640]   ", "kw"), s("# PCM16 LE, 16 kHz, 20 ms", "muted")],
    ],
  },
  {
    opcode: "0x02 / 0x03 · Control frames",
    name: "Tool calls and pairing",
    summary:
      "Opcode 0x02 carries JSON-RPC tool calls from your server to the charm. " +
      "Opcode 0x03 handles pairing, over Bluetooth only.",
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
    badge: "01 · Hardware",
    title: "Build the hardware files",
    summary: "Check the circuit, then export the printable case, the Gerbers and the KiCad project.",
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
    badge: "02 · Firmware",
    title: "Test and flash the firmware",
    summary: "Run the tests on your computer, then flash Pastille over USB-C.",
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
