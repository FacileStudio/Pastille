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

export interface SourceLink {
  label: string;
  href: string;
}

export interface ProtocolPlane {
  opcode: string;
  name: string;
  summary: string;
  lines: CodeLine[];
}

export const VERSION = "0.5.2";

export const PREORDER_PRICE = "$49";

export const RELEASE_TAG = `v${VERSION}`;

export const NAV_SECTIONS = [
  { id: "disassembly", label: "Inside", icon: "solar:layers-linear" },
  { id: "viewer", label: "Board", icon: "solar:cpu-linear" },
  { id: "connectivity", label: "Connectivity", icon: "solar:bluetooth-linear" },
  { id: "architecture", label: "Hardware", icon: "solar:widget-linear" },
  { id: "protocol", label: "Protocol", icon: "solar:code-linear" },
  { id: "comparison", label: "Comparison", icon: "solar:chart-2-linear" },
  { id: "quickstart", label: "Build", icon: "solar:box-linear" },
] as const;

export const BATTERY = {
  cell: "402020",
  capacity: "120 mAh",
  chargeCurrent: "about 67 mA",
};

export const HERO_METRICS: Metric[] = [
  { label: "Size", value: "42 mm", detail: "14 mm thick, with a lanyard loop" },
  { label: "Daily use", value: "3.5 days", detail: "With 20 min of talk a day (estimate)" },
  { label: "Standby", value: "80 days", detail: "Asleep, on one charge (estimate)" },
  { label: "Parts cost", value: "$21.80", detail: "Per unit, at 1,000 units" },
];

export const FEATURES: FeatureItem[] = [
  {
    icon: "solar:cpu-bolt-bold-duotone",
    title: "ESP32-S3 module",
    description: "An ESP32-S3 module with 4 MB of flash, 2 MB of PSRAM, Bluetooth and Wi-Fi. It runs bare-metal Rust.",
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
    description: "A 1.28 inch, 240x240 display that shows Bulle's face.",
    tag: "GC9A01 over SPI",
  },
  {
    icon: "solar:magic-stick-3-bold-duotone",
    title: "Touch and haptics",
    description: "Tap the pad to wake Pastille. A small vibration motor taps back for alerts.",
    tag: "Wake target: 50 ms",
  },
  {
    icon: "solar:battery-charge-bold-duotone",
    title: "Battery and charging",
    description:
      `A ${BATTERY.capacity} cell charges from USB-C in about 2.5 hours. ` +
      "The cell has its own protection circuit.",
    tag: `TP4056 at ${BATTERY.chargeCurrent}`,
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
  { metric: "Form factor", pastille: "42 mm disc, 14 mm thick", omi: "Pendant on a chain", rabbit: "Handheld, 115 g" },
  { metric: "Cost", pastille: "$21.80 in parts at 1,000 units", omi: "$169 retail", rabbit: "$199 retail" },
  { metric: "Display", pastille: '1.28" 240x240 round IPS', omi: "No screen", rabbit: '2.88" touchscreen' },
  {
    metric: "Battery",
    pastille: "About 1.3 h of talk (estimate)",
    omi: "10 to 14 hours",
    rabbit: "Under a day of moderate use",
  },
  {
    metric: "Runtime",
    pastille: "Bare-metal Rust on ESP32-S3",
    omi: "nRF5340, Bluetooth to a phone app",
    rabbit: "rabbit OS, built on AOSP",
  },
  {
    metric: "Your data",
    pastille: "Your own server",
    omi: "Omi Cloud, no supported self-hosting",
    rabbit: "Rabbit's cloud",
  },
];

export const COMPARISON_SOURCES: SourceLink[] = [
  { label: "Omi FAQ", href: "https://www.omi.me/pages/learn-omi-faq" },
  { label: "Omi product page", href: "https://www.omi.me/pages/omi-product" },
  { label: "Omi CV1 specs", href: "https://docs.omi.me/doc/hardware/OmiConsumer" },
  { label: "Omi Cloud", href: "https://docs.omi.me/doc/get_started/open-source-and-omi-cloud" },
  { label: "XDA's Rabbit R1 review", href: "https://www.xda-developers.com/rabbit-r1-review/" },
  { label: "The Shortcut's Rabbit R1 review", href: "https://www.theshortcut.com/p/rabbit-r1-review-ai" },
  {
    label: "Rabbit's note on AOSP",
    href: "https://www.rabbit.tech/blog/making-r1-more-accessible-to-the-developer-community",
  },
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
    badge: "Hardware",
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
    badge: "Firmware",
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
