import { keepOutCuts, openingCuts, pinZones, usbCut } from "./cuts";
import { carve, cyl, grown, k, slot, type Shape, type Vec2 } from "./geom";
import { panelOutline } from "./refs";
import { FIT, PART, type Stack } from "./stack";

const LENS_CLEAR = 0.1;

/** Radius of the clear lens: wider than the window, narrower than the panel glass. */
export const lensRadius = (): number => PART.lcd.glass / 2 - 0.3;

const skirtTop = (s: Stack): number => s.z.joint + FIT.lipHeight + FIT.lipClear;

const boss = (s: Stack, at: Vec2): Shape[] => {
  const top = s.z.lcd0 - FIT.lcdGap;
  const dir = Math.hypot(...at);
  const out: Vec2 = [(at[0] / dir) * (s.radius.pocket + 0.5), (at[1] / dir) * (s.radius.pocket + 0.5)];
  return [cyl(at, FIT.boss.bezel / 2, s.z.joint, top), slot(at, out, 1, skirtTop(s), top)];
};

const cavities = (s: Stack): Shape[] => [
  grown(s.outline, FIT.boardGap + FIT.lip + FIT.lipClear, s.z.joint - 0.01, skirtTop(s)),
  grown(s.outline, FIT.boardGap, s.z.joint - 0.01, s.z.lcd0 - FIT.lcdGap),
  panelOutline(FIT.lcdGap, s.z.lcd0 - FIT.lcdGap, s.z.lens0),
  cyl([0, 0], lensRadius() + LENS_CLEAR, s.z.lens0 - 0.01, s.z.lens1),
  cyl([0, 0], PART.lcd.active / 2, s.z.lens1 - 0.01, s.z.front + 0.01),
];

/** The front bezel: a cap with the round window over the active area, the lens recess and two screw bosses. */
export const frontBezel = (s: Stack): Shape => {
  const shell = k.cutAll(cyl([0, 0], s.radius.outer, s.z.joint, s.z.front), cavities(s));
  const band = pinZones(s, s.z.joint - 0.01, s.z.joint + FIT.probeDepth + 0.05);
  const bosses = s.mounts.flatMap((at) => boss(s, at)).map((b) => k.cutAll(b, band));
  const pilots = s.mounts.map((at) => cyl(at, FIT.boss.pilot / 2, s.z.joint - 0.01, s.z.lcd0 - 0.6));
  return carve(shell, bosses, [...pilots, usbCut(s), ...openingCuts(s), ...keepOutCuts(s, 0.2)]);
};

/** The clear lens disc, held from the front by the bezel lip. */
export const lens = (s: Stack): Shape => cyl([0, 0], lensRadius(), s.z.lens0, s.z.lens1);
