import { BATTERY, VERSION } from "$lib/data";

export type PartName = "front_bezel" | "lens" | "lcd" | "main" | "speaker" | "lra" | "battery" | "back_shell";

export type Vec3 = [number, number, number];

export interface ModelSource {
  url: string;
  name?: PartName;
  rotation?: Vec3;
  position?: Vec3;
}

export interface ExplodeStep {
  id: string;
  icons: string[];
  title: string;
  description: string;
  motionNote?: string;
  offsets: Partial<Record<PartName, Vec3>>;
  rotation: { pitch: number; yaw: number };
}

export const PARTS: PartName[] = ["front_bezel", "lens", "lcd", "main", "speaker", "lra", "battery", "back_shell"];

export const ASSEMBLY_SOURCES: ModelSource[] = [
  { url: `/models/pastille_assembly.glb?v=${VERSION}` },
  { url: `/models/pastille.glb?v=${VERSION}`, name: "main", rotation: [Math.PI / 2, 0, 0] },
];

export const LCD_WINDOW_RADIUS_MM = 16.2;

export const BLOB_FADE_STEPS: [number, number] = [1.5, 2];

const FRONT = { pitch: 0, yaw: 0 };
const THREE_QUARTER = { pitch: 35, yaw: -25 };

export const STEPS: ExplodeStep[] = [
  {
    id: "front",
    icons: ["solar:watch-round-bold-duotone"],
    title: "The assembled charm",
    description: "Pastille as you wear it, a 42 mm disc with Bulle on a round screen.",
    motionNote: "Move your pointer and Bulle's eyes follow it.",
    offsets: {},
    rotation: FRONT,
  },
  {
    id: "bezel",
    icons: ["solar:eye-bold-duotone"],
    title: "Lens and bezel",
    description:
      "A printed front bezel holds a clear 0.8 mm lens over the screen. " +
      "Its round window matches the visible area of the display.",
    offsets: { front_bezel: [0, 0, 56], lens: [0, 0, 42] },
    rotation: THREE_QUARTER,
  },
  {
    id: "lcd",
    icons: ["solar:monitor-smartphone-bold-duotone"],
    title: "Round LCD",
    description:
      'A 1.28" GC9A01 round LCD shows 240x240 pixels. ' +
      "The main board drives it over SPI.",
    offsets: { lcd: [0, 0, 28] },
    rotation: THREE_QUARTER,
  },
  {
    id: "pcb",
    icons: ["solar:cpu-bolt-bold-duotone"],
    title: "Main board",
    description:
      "A 38 mm round board carries an ESP32-S3 module with 4 MB of flash, 2 MB of PSRAM, Bluetooth and Wi-Fi. " +
      "It runs bare-metal Rust firmware built on Embassy.",
    offsets: { main: [0, 0, 14] },
    rotation: THREE_QUARTER,
  },
  {
    id: "audio",
    icons: ["solar:microphone-3-bold-duotone", "solar:volume-loud-bold-duotone"],
    title: "Mic and speaker",
    description:
      "An SPH0645LM4H-B MEMS mic sends your voice to the ESP32-S3 over I2S. " +
      "A MAX98357A amplifier takes I2S audio back out and drives a 15 x 11 mm speaker.",
    offsets: { speaker: [0, -44, -18] },
    rotation: THREE_QUARTER,
  },
  {
    id: "haptics",
    icons: ["solar:smartphone-vibration-bold-duotone"],
    title: "Haptics",
    description:
      "A DRV2605L driver runs a small LRA motor for taps and alerts. " +
      "The ESP32-S3 controls it over I2C.",
    offsets: { lra: [30, 0, -10] },
    rotation: THREE_QUARTER,
  },
  {
    id: "battery",
    icons: ["solar:battery-charge-bold-duotone"],
    title: "Battery and charger",
    description:
      `A ${BATTERY.capacity} ${BATTERY.cell} lithium polymer cell powers the charm. ` +
      `A TP4056 charges it from USB-C at ${BATTERY.chargeCurrent}.`,
    offsets: { battery: [-28, -24, -14] },
    rotation: THREE_QUARTER,
  },
  {
    id: "shell",
    icons: ["solar:usb-bold-duotone"],
    title: "Back shell",
    description:
      "The printed back shell has the USB-C opening, a port for the mic and a grille for the speaker. " +
      "Two M2 screws close it onto the board's mounting holes.",
    offsets: { back_shell: [0, 0, -34] },
    rotation: THREE_QUARTER,
  },
];
