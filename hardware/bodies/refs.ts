import { box, cyl, k, type Shape } from "./geom";
import { PART, type Stack } from "./stack";

/** The GC9A01 panel outline (ZJY128R-IG01): the round glass plus the FPC ledge toward +Y, grown by `grow`. */
export const panelOutline = (grow: number, z0: number, z1: number): Shape => {
  const { glass, ledgeReach, ledgeHalf } = PART.lcd;
  const round = cyl([0, 0], glass / 2 + grow, z0, z1);
  const strip = box([0, ledgeReach / 2], 2 * (ledgeHalf + grow), ledgeReach, z0, z1);
  const ledge = k.common(cyl([0, 0], ledgeReach + grow, z0, z1), strip);
  return k.fuse(round, ledge);
};

/** The LCD reference body: the panel and its backlight frame, sitting just above the tallest board part. */
export const lcd = (s: Stack): Shape => panelOutline(0, s.z.lcd0, s.z.lcd0 + PART.lcd.thick);

/** The 402020 cell: swell room on every face, and clear of the through-hole pin tails under the board. */
export const battery = (s: Stack): Shape => {
  const { w, h, t, at } = PART.cell;
  return box(at, w, h, s.z.cellTop - t, s.z.cellTop);
};

/** The 15 x 11 x 2.5 mm speaker, face down on its seat so its sound reaches the floor grille. */
export const speaker = (s: Stack): Shape => {
  const { w, h, t, at, seat } = PART.speaker;
  const z0 = s.z.floor + seat;
  return box(at, w, h, z0, z0 + t);
};

/** The 0827 coin LRA, glued flat on the floor. */
export const lra = (s: Stack): Shape => cyl(PART.lra.at, PART.lra.d / 2, s.z.floor, s.z.floor + PART.lra.t);
