import type { Vec2 } from "./geom";
import { FIT, PART } from "./parts";

export type KitHole = { x: number; y: number; diameter: number; plated: boolean };
export type KitPart = { name: string; x: number; y: number; w: number; h: number; z?: number; height?: number };
type KitOpening = { component: string; normal: Vec2; from: Vec2; to: Vec2 };
export type KeepOut = {
  component: string;
  layer: string;
  x: number;
  y: number;
  w: number;
  h: number;
  z0: number;
  z1: number;
};
export type Kit = {
  outline: Vec2[];
  thickness: number;
  holes: KitHole[];
  components: KitPart[];
  openings: KitOpening[];
  keep_out?: KeepOut[];
};
export type Opening = { from: Vec2; to: Vec2; normal: Vec2; z0: number; z1: number };

export const nearest = (kit: Kit, name: string): KitPart => {
  const part = kit.components.find((c) => c.name === name);
  if (!part) throw new Error(`kit has no component ${name}`);
  return part;
};

export const openingsOf = (kit: Kit, half: number): Opening[] =>
  kit.openings.map((o) => {
    const part = nearest(kit, o.component);
    const z0 = part.z ?? half;
    return { from: o.from, to: o.to, normal: o.normal, z0, z1: z0 + (part.height ?? FIT.unmeasured) };
  });

export const topClearance = (kit: Kit, half: number): number => {
  const tops = (kit.keep_out ?? []).filter((b) => b.layer === "top").map((b) => b.z1);
  return Math.max(...tops, half + PART.usbHeight);
};

export const plugAxis = (half: number): number => half + PART.usbShell / 2;

export const micHole = (kit: Kit): Vec2 => {
  const mic = nearest(kit, "U_MIC");
  const plain = kit.holes.filter((h) => !h.plated && h.diameter < 1);
  const dist = (h: KitHole) => Math.hypot(h.x - mic.x, h.y - mic.y);
  const hole = plain.sort((a, b) => dist(a) - dist(b))[0];
  return [hole.x, hole.y];
};
