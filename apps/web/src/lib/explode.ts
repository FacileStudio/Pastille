import { VERSION } from "$lib/data";

export type PartName = "front_bezel" | "lens" | "lcd" | "main" | "speaker" | "lra" | "battery" | "back_shell";

export type Vec3 = [number, number, number];

export interface ModelSource {
  url: string;
  name?: PartName;
  rotation?: Vec3;
  position?: Vec3;
}

export type StepId = "front" | "bezel" | "lcd" | "pcb" | "audio" | "haptics" | "battery" | "shell";

export interface ExplodeStep {
  id: StepId;
  icons: string[];
  offsets: Partial<Record<PartName, Vec3>>;
  rotation: { pitch: number; yaw: number };
}

export const PARTS: PartName[] = ["front_bezel", "lens", "lcd", "main", "speaker", "lra", "battery", "back_shell"];

export const ASSEMBLY_SOURCES: ModelSource[] = [
  { url: `/models/pastille_assembly.glb?v=${VERSION}` },
  { url: `/models/pastille.glb?v=${VERSION}`, name: "main", rotation: [Math.PI / 2, 0, 0] },
];

const FRONT = { pitch: 0, yaw: 0 };
const THREE_QUARTER = { pitch: 35, yaw: -25 };

export const STEPS: ExplodeStep[] = [
  {
    id: "front",
    icons: ["solar:watch-round-bold-duotone"],
    offsets: {},
    rotation: FRONT,
  },
  {
    id: "bezel",
    icons: ["solar:eye-bold-duotone"],
    offsets: { front_bezel: [0, 0, 56], lens: [0, 0, 42] },
    rotation: THREE_QUARTER,
  },
  {
    id: "lcd",
    icons: ["solar:monitor-smartphone-bold-duotone"],
    offsets: { lcd: [0, 0, 28] },
    rotation: THREE_QUARTER,
  },
  {
    id: "pcb",
    icons: ["solar:cpu-bolt-bold-duotone"],
    offsets: { main: [0, 0, 14] },
    rotation: THREE_QUARTER,
  },
  {
    id: "audio",
    icons: ["solar:microphone-3-bold-duotone", "solar:volume-loud-bold-duotone"],
    offsets: { speaker: [0, -44, -18] },
    rotation: THREE_QUARTER,
  },
  {
    id: "haptics",
    icons: ["solar:smartphone-vibration-bold-duotone"],
    offsets: { lra: [30, 0, -10] },
    rotation: THREE_QUARTER,
  },
  {
    id: "battery",
    icons: ["solar:battery-charge-bold-duotone"],
    offsets: { battery: [-28, -24, -14] },
    rotation: THREE_QUARTER,
  },
  {
    id: "shell",
    icons: ["solar:usb-bold-duotone"],
    offsets: { back_shell: [0, 0, -34] },
    rotation: THREE_QUARTER,
  },
];
