export type TopologyModeId = "ble" | "wifi" | "pair";

export type DeviceId = "pastille" | "speaker" | "headset";

export interface TopologyPill {
  icon: string;
  value: string;
}

export interface TopologyMode {
  id: TopologyModeId;
  button: string;
  icon: string;
  summary: string;
  devices: DeviceId[];
  direct: boolean;
  pills: TopologyPill[];
}

export interface TopologyNode {
  id: DeviceId;
  label: string;
  icon: string;
}

export interface Point {
  x: number;
  y: number;
}

export interface TopologyLayout {
  viewBox: string;
  devices: Point[];
  hub: Point;
  server: Point;
  direct: Point;
  uplink: Point;
  wifi: Point;
  labelDy: number;
}

export const LAYOUTS: Record<"horizontal" | "vertical", TopologyLayout> = {
  horizontal: {
    viewBox: "0 0 880 360",
    devices: [{ x: 110, y: 60 }, { x: 110, y: 180 }, { x: 110, y: 300 }],
    hub: { x: 440, y: 110 },
    server: { x: 770, y: 180 },
    direct: { x: 440, y: 340 },
    uplink: { x: 640, y: 110 },
    wifi: { x: 440, y: 262 },
    labelDy: 52,
  },
  vertical: {
    viewBox: "0 0 360 590",
    devices: [{ x: 60, y: 84 }, { x: 180, y: 84 }, { x: 300, y: 84 }],
    hub: { x: 100, y: 300 },
    server: { x: 230, y: 490 },
    direct: { x: 330, y: 300 },
    uplink: { x: 100, y: 440 },
    wifi: { x: 272, y: 300 },
    labelDy: -48,
  },
};

export const TOPOLOGY_DEVICES: TopologyNode[] = [
  { id: "pastille", label: "Pastille", icon: "solar:circle-dot-bold-duotone" },
  { id: "speaker", label: "Speaker", icon: "solar:smart-speaker-bold-duotone" },
  { id: "headset", label: "Headset", icon: "solar:headphones-round-sound-bold-duotone" },
];

export const TOPOLOGY_HUB = { label: "Bulle app", icon: "solar:monitor-smartphone-bold-duotone" };

export const TOPOLOGY_SERVER = { label: "Your server", icon: "solar:server-square-bold-duotone" };

export const TOPOLOGY_MODES: TopologyMode[] = [
  {
    id: "ble",
    button: "Bluetooth",
    icon: "solar:bluetooth-wave-bold-duotone",
    summary: "Every device talks to the Bulle app, which relays to your server.",
    devices: ["pastille", "speaker", "headset"],
    direct: false,
    pills: [
      { icon: "solar:bluetooth-wave-bold-duotone", value: "Bluetooth LE" },
      { icon: "solar:battery-full-bold-duotone", value: "24 h target" },
      { icon: "solar:monitor-smartphone-bold-duotone", value: "Via the app" },
    ],
  },
  {
    id: "wifi",
    button: "Wi-Fi",
    icon: "solar:wi-fi-router-bold-duotone",
    summary: "At home, every device connects straight to your server.",
    devices: ["pastille", "speaker", "headset"],
    direct: true,
    pills: [
      { icon: "solar:wi-fi-router-bold-duotone", value: "Wi-Fi" },
      { icon: "solar:battery-full-bold-duotone", value: "8 h target" },
      { icon: "solar:server-square-bold-duotone", value: "No phone needed" },
    ],
  },
  {
    id: "pair",
    button: "Pairing",
    icon: "solar:hand-shake-bold-duotone",
    summary: "Tap Pastille once. The app hands it a key over an encrypted link.",
    devices: ["pastille"],
    direct: false,
    pills: [
      { icon: "solar:hand-shake-bold-duotone", value: "One tap" },
      { icon: "solar:shield-check-bold-duotone", value: "Encrypted" },
      { icon: "solar:key-bold-duotone", value: "Key saved" },
    ],
  },
];
