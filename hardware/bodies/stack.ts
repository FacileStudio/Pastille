import type { Vec2 } from "./geom";
import { micHole, nearest, openingsOf, plugAxis, topClearance, type Kit, type KitHole } from "./kit";
import { FIT, PART } from "./parts";

export { FIT, PART } from "./parts";
export type { Kit, KeepOut, Opening } from "./kit";
export type Stack = ReturnType<typeof stackOf>;

const pinBand = (kit: Kit): number => {
  const { w, h, at } = PART.cell;
  const reach = FIT.probeNear + 1;
  const under = (p: KitHole) => Math.abs(p.x - at[0]) < w / 2 + reach && Math.abs(p.y - at[1]) < h / 2 + reach;
  return kit.holes.some((p) => p.plated && under(p)) ? FIT.probeDepth : 0;
};

const levels = (kit: Kit, half: number) => {
  const lcd0 = topClearance(kit, half) + FIT.air;
  const lens0 = lcd0 + PART.lcd.thick + FIT.lensGap;
  const cellTop = -half - Math.max(pinBand(kit), PART.cell.swell);
  const floor = cellTop - PART.cell.t - PART.cell.swell;
  const front = lens0 + PART.lens + FIT.lip;
  return { half, back: floor - FIT.floor, floor, cellTop, joint: half, lcd0, lens0, lens1: lens0 + PART.lens, front };
};

/** Derives every level and position of the case from the board kit, in the board frame (Z up, board centred on 0). */
export const stackOf = (kit: Kit) => {
  const half = kit.thickness / 2;
  const board = Math.max(...kit.outline.map(([x, y]) => Math.hypot(x, y)));
  const outer = PART.lcd.ledgeReach + FIT.lcdGap + FIT.ledgeWall;
  const radius = { board, pocket: board + FIT.boardGap, outer, under: outer - FIT.wall };
  const plain = kit.holes.filter((h) => !h.plated && h.diameter >= 2);
  const mounts = plain.map((h): Vec2 => [h.x, h.y]);
  const holeR = Math.max(...plain.map((h) => h.diameter / 2));
  const pins = kit.holes.filter((h) => h.plated);
  const bat = nearest(kit, "J_BAT");
  const parts = { usb: nearest(kit, "J_USB"), plug: plugAxis(half), bat: [bat.x, bat.y] as Vec2 };
  const keepOut = kit.keep_out ?? [];
  const openings = openingsOf(kit, half);
  const shape = { outline: kit.outline, radius, mounts, holeR, pins, mic: micHole(kit) };
  return { z: levels(kit, half), ...shape, ...parts, openings, keepOut };
};
