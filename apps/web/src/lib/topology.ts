export type TopologyModeId = "ble" | "wifi" | "pair";

export type TopologyLink = "relay" | "direct" | "provision";

export interface TelemetryPill {
  label: string;
  value: string;
}

export interface TopologyMode {
  id: TopologyModeId;
  button: string;
  title: string;
  summary: string;
  links: TopologyLink[];
  telemetry: TelemetryPill[];
}

export const TOPOLOGY_MODES: TopologyMode[] = [
  {
    id: "ble",
    button: "BLE 5.0 Relay (14 mA)",
    title: "BLE 5.0 Relay (on the go & desktop)",
    summary:
      "Pastille streams over BLE 5.0 GATT (B002 audio, B003 control) to the Bulle app. Each frame gets a u16be " +
      "length prefix and is split into MTU-sized chunks. The app relays it over a WSS tunnel to your Bulle Gateway.",
    links: ["relay"],
    telemetry: [
      { label: "Active transport", value: "BLE 5.0 GATT · 2M PHY" },
      { label: "Wake latency", value: "< 6 ms link resume" },
      { label: "Power budget", value: "14 mA · 24h talk (target)" },
      { label: "Account sync", value: "Relayed by the Bulle app" },
    ],
  },
  {
    id: "wifi",
    button: "Direct Wi-Fi WSS",
    title: "Direct Wi-Fi WSS (home & office)",
    summary:
      "On a known network, Pastille opens a TLS WebSocket (bohs.v1) straight to the Bulle Gateway. " +
      "It sends one frame per WebSocket message. No phone needed.",
    links: ["direct"],
    telemetry: [
      { label: "Active transport", value: "Wi-Fi WSS · bohs.v1" },
      { label: "Wake latency", value: "< 50 ms RTC touch wake" },
      { label: "Power budget", value: "42 mA · 8h talk (target)" },
      { label: "Account sync", value: "Direct to Bulle Gateway" },
    ],
  },
  {
    id: "pair",
    button: "1-Tap BLE Pairing",
    title: "One-tap BLE pairing & fleet sync",
    summary:
      "Tap Pastille (GPIO7). The Bulle app reads B001, Facile Registre mints a bohs_tok_* token, and the app " +
      "writes the token and Wi-Fi credentials to NVS (B004) over an encrypted, bonded link within 60 s of the tap.",
    links: ["relay", "provision"],
    telemetry: [
      { label: "Active transport", value: "BLE · encrypted & bonded" },
      { label: "Wake latency", value: "Tap confirms within 60 s" },
      { label: "Power budget", value: "BLE only during setup" },
      { label: "Account sync", value: "bohs_tok_* to NVS (B004)" },
    ],
  },
];
