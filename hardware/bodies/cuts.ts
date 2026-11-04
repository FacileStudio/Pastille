import { box, cyl, prism, slot, type Shape, type Vec2 } from "./geom";
import { FIT, PART, type Opening, type Stack } from "./stack";

const SLOT_MARGIN = 0.3;
const SLOT_REACH = 4;

const unit = ([x, y]: Vec2): Vec2 => {
  const len = Math.hypot(x, y);
  return [x / len, y / len];
};

/** The USB-C opening: the plug overmold plus clearance, from inside the board pocket out through the wall. */
export const usbCut = (s: Stack): Shape => {
  const dir = unit([s.usb.x, s.usb.y]);
  const { w, h, clearance } = PART.plug;
  const zc = s.plug;
  const start = s.radius.pocket - 1.4;
  const end = s.radius.outer + 1;
  const from: Vec2 = [dir[0] * start, dir[1] * start];
  const to: Vec2 = [dir[0] * end, dir[1] * end];
  return slot(from, to, w / 2 + clearance, zc - h / 2 - clearance, zc + h / 2 + clearance);
};

const openingCut = (o: Opening): Shape => {
  const t = unit([o.to[0] - o.from[0], o.to[1] - o.from[1]]);
  const n = unit(o.normal);
  const at = (p: Vec2, along: number, out: number): Vec2 => [
    p[0] + t[0] * along + n[0] * out,
    p[1] + t[1] * along + n[1] * out,
  ];
  const quad = [
    at(o.from, -SLOT_MARGIN, -0.5),
    at(o.to, SLOT_MARGIN, -0.5),
    at(o.to, SLOT_MARGIN, SLOT_REACH),
    at(o.from, -SLOT_MARGIN, SLOT_REACH),
  ];
  return prism(quad, o.z0 - 0.1, o.z1 + 0.1);
};

/** One wall slot per kit opening, so no case wall stands in front of a part that reaches the board edge. */
export const openingCuts = (s: Stack): Shape[] => s.openings.map(openingCut);

/** The board's modeled part volumes grown by `grow`, so no case material reaches into them. */
export const keepOutCuts = (s: Stack, grow: number): Shape[] =>
  s.keepOut.map((b) => box([b.x, b.y], b.w + 2 * grow, b.h + 2 * grow, b.z0 - grow, b.z1 + grow));

/** Keep-clear cylinders around every plated through-hole, from `z0` to `z1`, so no part stands near a pin tail. */
export const pinZones = (s: Stack, z0: number, z1: number): Shape[] =>
  s.pins.map((p) => cyl([p.x, p.y], p.diameter / 2 + FIT.probeNear + 0.15, z0, z1));

/** The battery lead channel: a groove in the wall at the J_BAT angle, from the floor to the joint. */
export const leadChannel = (s: Stack): Shape => {
  const dir = unit(s.bat);
  const r0 = Math.hypot(...s.bat) - 2.5;
  const r1 = s.radius.pocket + 1.1;
  return slot([dir[0] * r0, dir[1] * r0], [dir[0] * r1, dir[1] * r1], 1.5, s.z.floor, s.z.joint + FIT.lipHeight + 0.1);
};
