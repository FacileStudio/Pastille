import { box, cyl, frame, k, ring, type Shape, type Vec2 } from "./geom";
import { pinZones } from "./cuts";
import { FIT, PART, type Stack } from "./stack";

const GRILLE = { hole: 1.0, pitch: 2.0, margin: 1.2 };
const RIM = 0.8;
const LUG = { width: 6, reach: 5.5, hole: 2.4, z: [1.0, 4.0] as [number, number] };

const grid = (at: Vec2, w: number, h: number): Vec2[] => {
  const cols = Math.floor((w - 2 * GRILLE.margin) / GRILLE.pitch);
  const rows = Math.floor((h - 2 * GRILLE.margin) / GRILLE.pitch);
  const spots: Vec2[] = [];
  for (let i = 0; i < cols; i++) {
    for (let j = 0; j < rows; j++) {
      spots.push([at[0] + (i - (cols - 1) / 2) * GRILLE.pitch, at[1] + (j - (rows - 1) / 2) * GRILLE.pitch]);
    }
  }
  return spots;
};

/** The 1 mm grille holes through the floor, under the speaker cone. */
export const grille = (s: Stack): Shape[] =>
  grid(PART.speaker.at, PART.speaker.w, PART.speaker.h).map((p) =>
    cyl(p, GRILLE.hole / 2, s.z.back - 0.01, s.z.floor + 0.01),
  );

/** Pocket rims for the battery, speaker and LRA, kept below the board hole probes. */
export const rims = (s: Stack): Shape[] => {
  const top: [number, number] = [s.z.floor, -s.z.half - FIT.probeDepth];
  const { cell, speaker, lra } = PART;
  const seat = speaker.w - 2 * GRILLE.margin + 0.4;
  return [
    frame(cell.at, cell.w + 2 * cell.swell, cell.h + 2 * cell.swell, RIM, top),
    frame(speaker.at, speaker.w + 0.4, speaker.h + 0.4, RIM, top),
    frame(speaker.at, seat, speaker.h - 2 * GRILLE.margin + 0.4, 1.2, [s.z.floor, s.z.floor + speaker.seat]),
    ring(lra.at, lra.d / 2 + 0.2, lra.d / 2 + 0.2 + RIM, s.z.floor, s.z.floor + 1.5),
  ];
};

/** The two screw bosses under the board holes and the mic tube that ends one gasket below the board. */
export const posts = (s: Stack): Shape[] => {
  const band = pinZones(s, -s.z.half - FIT.probeDepth - 0.05, -s.z.half + 0.01);
  const bosses = s.mounts.map((at) => cyl(at, FIT.boss.shell / 2, s.z.floor - 0.01, -s.z.half));
  const tube = cyl(s.mic, FIT.mic.tube / 2, s.z.floor - 0.01, -s.z.half - FIT.mic.gasket);
  return [...bosses.map((b) => k.cutAll(b, band)), tube];
};

/** Screw clearance, screw head counterbores, the mic port and its outer mesh recess. */
export const bores = (s: Stack): Shape[] => [
  ...s.mounts.map((at) => cyl(at, FIT.boss.clear / 2, s.z.back - 0.01, -s.z.half + 0.01)),
  ...s.mounts.map((at) => cyl(at, FIT.boss.head / 2, s.z.back - 0.01, s.z.back + FIT.boss.headDepth)),
  cyl(s.mic, FIT.mic.port / 2, s.z.back - 0.01, -s.z.half),
  cyl(s.mic, FIT.mic.mesh / 2, s.z.back - 0.01, s.z.back + FIT.mic.meshDepth),
];

/** The lanyard lug at 12 o'clock: a rounded tab on the back half of the wall with a split-ring hole. */
export const lug = (s: Stack): { tab: Shape; hole: Shape } => {
  const r = s.radius.outer;
  const tip: Vec2 = [0, r + LUG.reach - LUG.width / 2];
  const z0 = s.z.back + LUG.z[0];
  const z1 = s.z.back + LUG.z[1];
  const stem = box([0, (r - 1 + tip[1]) / 2], LUG.width, tip[1] - r + 1, z0, z1);
  const tab = k.fuse(stem, cyl(tip, LUG.width / 2, z0, z1));
  return { tab, hole: cyl(tip, LUG.hole / 2, z0 - 0.01, z1 + 0.01) };
};
