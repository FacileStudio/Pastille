import { keepOutCuts, leadChannel, openingCuts, usbCut } from "./cuts";
import { carve, cyl, grown, k, type Shape } from "./geom";
import { bores, grille, lug, posts, rims } from "./shell-parts";
import { FIT, type Stack } from "./stack";

const tongue = (s: Stack): Shape => {
  const [z0, z1] = [s.z.joint - 0.01, s.z.joint + FIT.lipHeight];
  const outer = grown(s.outline, FIT.boardGap + FIT.lip, z0, z1);
  const lip = k.cut(outer, grown(s.outline, FIT.boardGap, z0 - 0.01, z1 + 0.01));
  const clear = s.holeR + FIT.probeNear + 0.3;
  return k.cutAll(lip, s.mounts.map((at) => cyl(at, clear, s.z.joint - 0.1, s.z.joint + FIT.lipHeight + 0.1)));
};

const cup = (s: Stack): Shape => {
  const outer = cyl([0, 0], s.radius.outer, s.z.back, s.z.joint);
  const under = cyl([0, 0], s.radius.under, s.z.floor, -s.z.half);
  const pocket = grown(s.outline, FIT.boardGap, -s.z.half - 0.01, s.z.joint + 0.01);
  return k.fuse(k.cutAll(outer, [under, pocket]), tongue(s));
};

/** The back shell: a cup with the battery, speaker and LRA pockets, bosses, mic port, grille, USB-C and lug. */
export const backShell = (s: Stack): Shape => {
  const { tab, hole } = lug(s);
  const cuts = [...bores(s), ...grille(s), leadChannel(s), usbCut(s), ...openingCuts(s), ...keepOutCuts(s, 0.2), hole];
  return carve(cup(s), [...posts(s), ...rims(s), tab], cuts);
};
