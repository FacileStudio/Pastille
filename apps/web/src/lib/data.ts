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

export interface SubsystemGroup {
  title: string;
  subtitle: string;
  items: string[];
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
  payload: string;
}

export const HERO_METRICS: Metric[] = [
  { label: "Diameter", value: "38.0 mm", detail: "4-layer ENIG circular PCB, 11.5 mm case" },
  { label: "Deep sleep", value: "18 uA", detail: "Over 14 days standby on 250 mAh LiPo" },
  { label: "Wake latency", value: "< 50 ms", detail: "Capacitive RTC wake to active firmware" },
  { label: "Unit BOM", value: "$23.85", detail: "100-unit PCBA turnkey assembly" },
];

export const FEATURES: FeatureItem[] = [
  {
    icon: "solar:cpu-bolt-bold-duotone",
    title: "ESP32-S3R8 dual-core silicon",
    description:
      "Dual-core Xtensa LX7 at 240 MHz with 8 MB octal PSRAM, hardware I2S DMA, and native USB-C JTAG/serial flashing.",
    tag: "Bare-metal Embassy Rust",
  },
  {
    icon: "solar:soundwave-bold-duotone",
    title: "Full-duplex I2S audio + VAD",
    description:
      "Omnidirectional INMP441 MEMS microphone paired with a MAX98357A Class-D speaker " +
      "amplifier and energy-based voice activity detection.",
    tag: "Opus 16 kHz · 20 ms frames",
  },
  {
    icon: "solar:eye-bold-duotone",
    title: "240x240 round IPS expression display",
    description:
      "1.28-inch GC9A01 circular display driven at 60 FPS using zero-float integer " +
      "trigonometry for Bulle companion states.",
    tag: "Idle · Listening · Thinking · Speaking",
  },
  {
    icon: "solar:magic-stick-3-bold-duotone",
    title: "DRV2605L LRA haptic engine",
    description:
      "Closed-loop linear resonant actuator controller over I2C plus a 510-ohm " +
      "ESD-damped capacitive touch wake pad on RTC GPIO 7.",
    tag: "Sub-50 ms tactile response",
  },
  {
    icon: "solar:battery-charge-bold-duotone",
    title: "Schottky power-path load sharing",
    description:
      "TP4056 charger isolated from system load via dual B5819W Schottky diodes, " +
      "SMF5.0A VBUS TVS protection, and 1 uA quiescent 3.3 V LDO.",
    tag: "Safe charge termination",
  },
  {
    icon: "solar:shield-keyhole-bold-duotone",
    title: "Private Bulle gateway + Reverse MCP",
    description:
      "Multiplexed WebSocket protocol (bohs.v1) authenticated via Registre and Casier " +
      "tokens. Zero third-party telemetry or cloud lock-in.",
    tag: "CERN-OHL-S v2 · Apache 2.0",
  },
];

export const SUBSYSTEMS: SubsystemGroup[] = [
  {
    title: "Compute & display",
    subtitle: "MCU, reset timing, and SPI LCD",
    items: [
      "ESP32-S3R8 (QFN-32 6x6 mm) with 8 MB Octal PSRAM",
      "Hardware CHIP_PU power-on reset RC delay (10k + 1 uF)",
      "TL3305AF160QG tactile BOOT switch on GPIO 0",
      "GC9A01 1.28-inch 240x240 round IPS LCD over 40 MHz SPI",
    ],
  },
  {
    title: "Power & protection",
    subtitle: "USB-C, load sharing, and telemetry",
    items: [
      "USB-C receptacle with 5.1k CC1/CC2 Rd and 22R D+/D- series termination",
      "SMF5.0A VBUS transient voltage suppressor for IEC 61000-4-2 ESD",
      "TP4056 charger (250 mA R_PROG = 4.7k) + dual B5819W Schottky load sharing",
      "XC6206P332MR 3.3 V LDO + 100k/100k battery ADC voltage divider",
    ],
  },
  {
    title: "Audio & haptics",
    subtitle: "Digital I2S duplex and I2C actuators",
    items: [
      "INMP441 bottom-port I2S MEMS microphone (61 dBA SNR)",
      "MAX98357AETE+ I2S Class-D amplifier on VSYS with 100k SD_MODE pull-down",
      "DRV2605LDGSR I2C LRA haptic driver with 4.7k bus pull-ups",
      "RTC GPIO 7 capacitive touch pad with 510R series ESD damping",
    ],
  },
];

export const COMPARISON: ComparisonRow[] = [
  {
    metric: "Form factor",
    pastille: "38 mm circular disc (14 g)",
    omi: "40 mm circular pendant",
    rabbit: "Handheld slab (115 g)",
  },
  {
    metric: "Unit BOM cost",
    pastille: "$23.85 open BOM",
    omi: "~ $45 BOM",
    rabbit: "$199 retail",
  },
  {
    metric: "Display",
    pastille: "1.28-inch 240x240 round IPS",
    omi: "None",
    rabbit: "2.88-inch TFT touchscreen",
  },
  {
    metric: "Standby battery life",
    pastille: "> 14 days (18 uA deep sleep)",
    omi: "~ 4 days",
    rabbit: "~ 12 hours",
  },
  {
    metric: "Runtime architecture",
    pastille: "Bare-metal Rust (Embassy)",
    omi: "BLE streamer to phone app",
    rabbit: "Full Android AOSP fork",
  },
  {
    metric: "Wake-to-speech latency",
    pastille: "< 50 ms wake · < 800 ms stream",
    omi: "Variable BLE pairing",
    rabbit: "> 45 s cold boot",
  },
  {
    metric: "Data custody",
    pastille: "Self-hosted Bulle gateway",
    omi: "Requires companion app",
    rabbit: "Proprietary cloud relay",
  },
];

export const PROTOCOL_PLANES: ProtocolPlane[] = [
  {
    opcode: "0x01 · Binary frame",
    name: "Bidirectional Opus 16 kHz audio plane",
    summary:
      "Every 20 ms voice frame carries an 8-byte BOHS-01 header (magic 0xB0, version 0x01, " +
      "flags for VAD and End-of-Utterance, sequence number, and payload length) followed by Opus audio.",
    payload: "Header: [0xB0, 0x01, flags, seq_hi, seq_lo, len_hi, len_lo, rsv] + Opus packet",
  },
  {
    opcode: "0x02 · Text frame",
    name: "Reverse MCP JSON-RPC 2.0 control plane",
    summary:
      "The Bulle gateway invokes edge hardware capabilities over the same WebSocket connection: " +
      "updating the circular avatar expression, firing LRA haptic waveforms, or receiving tap and battery events.",
    payload:
      '{"jsonrpc":"2.0","id":42,"method":"tools/call",' +
      '"params":{"name":"charm.display_avatar","arguments":{"state":"thinking"}}}',
  },
];

export const QUICKSTART_COMMANDS: string[] = [
  "git clone https://github.com/FacileStudio/Pastille.git && cd Pastille",
  "fusible check",
  "fusible export --preset all",
  "cd apps/firmware && cargo test",
];
