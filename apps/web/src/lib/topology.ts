import type { AppKey } from "$lib/i18n/messages";

export type TopologyModeId = "ble" | "wifi" | "pair";

export type DeviceId = "pastille" | "speaker" | "headset";

export interface TopologyPill {
  icon: string;
  value: AppKey;
}

export interface TopologyMode {
  id: TopologyModeId;
  button: AppKey;
  icon: string;
  summary: AppKey;
  devices: DeviceId[];
  direct: boolean;
  pills: TopologyPill[];
}

export interface TopologyNode {
  id: DeviceId;
  label: AppKey | "Pastille";
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
  { id: "speaker", label: "topo.speaker", icon: "solar:smart-speaker-bold-duotone" },
  { id: "headset", label: "topo.headset", icon: "solar:headphones-round-sound-bold-duotone" },
];

export const TOPOLOGY_HUB: { label: AppKey; icon: string } = {
  label: "topo.hub",
  icon: "solar:monitor-smartphone-bold-duotone",
};

export const TOPOLOGY_SERVER: { label: AppKey; icon: string } = {
  label: "topo.server",
  icon: "solar:server-square-bold-duotone",
};

export const TOPOLOGY_MODES: TopologyMode[] = [
  {
    id: "ble",
    button: "topo.ble.button",
    icon: "solar:bluetooth-wave-bold-duotone",
    summary: "topo.ble.summary",
    devices: ["pastille", "speaker", "headset"],
    direct: false,
    pills: [
      { icon: "solar:bluetooth-wave-bold-duotone", value: "topo.ble.link" },
      { icon: "solar:battery-full-bold-duotone", value: "topo.ble.battery" },
      { icon: "solar:monitor-smartphone-bold-duotone", value: "topo.ble.via" },
    ],
  },
  {
    id: "wifi",
    button: "topo.wifi.button",
    icon: "solar:wi-fi-router-bold-duotone",
    summary: "topo.wifi.summary",
    devices: ["pastille", "speaker", "headset"],
    direct: true,
    pills: [
      { icon: "solar:wi-fi-router-bold-duotone", value: "topo.wifi.link" },
      { icon: "solar:battery-full-bold-duotone", value: "topo.wifi.battery" },
      { icon: "solar:server-square-bold-duotone", value: "topo.wifi.phone" },
    ],
  },
  {
    id: "pair",
    button: "topo.pair.button",
    icon: "solar:hand-shake-bold-duotone",
    summary: "topo.pair.summary",
    devices: ["pastille"],
    direct: false,
    pills: [
      { icon: "solar:hand-shake-bold-duotone", value: "topo.pair.tap" },
      { icon: "solar:shield-check-bold-duotone", value: "topo.pair.encrypted" },
      { icon: "solar:key-bold-duotone", value: "topo.pair.key" },
    ],
  },
];
