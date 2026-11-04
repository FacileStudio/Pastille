import { OcctKernel, type ShapeHandle } from "occt-wasm";

export type Vec2 = [number, number];
export type Shape = ShapeHandle;

export const k = await OcctKernel.init();

/** Extrudes the closed polygon `points` from `z0` to `z1`. */
export const prism = (points: Vec2[], z0: number, z1: number): Shape => {
  const ring = points.map(([x, y]) => ({ x, y, z: z0 }));
  const edges = ring.map((p, i) => k.makeLineEdge(p, ring[(i + 1) % ring.length]));
  return k.extrude(k.makeFace(k.makeWire(edges)), 0, 0, z1 - z0);
};

/** The closed polygon `points` grown outward by `grow`, extruded from `z0` to `z1`. */
export const grown = (points: Vec2[], grow: number, z0: number, z1: number): Shape => {
  const ring = points.map(([x, y]) => ({ x, y, z: z0 }));
  const wire = k.makeWire(ring.map((p, i) => k.makeLineEdge(p, ring[(i + 1) % ring.length])));
  return k.extrude(k.makeFace(k.offsetWire2D(wire, grow)), 0, 0, z1 - z0);
};

/** A vertical cylinder of radius `r` centred on `[x, y]`, from `z0` to `z1`. */
export const cyl = ([x, y]: Vec2, r: number, z0: number, z1: number): Shape =>
  k.translate(k.makeCylinder(r, z1 - z0), x, y, z0);

/** An axis-aligned box of size `w` x `h` centred on `[x, y]`, from `z0` to `z1`. */
export const box = ([x, y]: Vec2, w: number, h: number, z0: number, z1: number): Shape =>
  k.makeBoxFromCorners({ x: x - w / 2, y: y - h / 2, z: z0 }, { x: x + w / 2, y: y + h / 2, z: z1 });

/** A rectangle that runs from `from` to `to` with half-width `half`, extruded from `z0` to `z1`. */
export const slot = (from: Vec2, to: Vec2, half: number, z0: number, z1: number): Shape => {
  const [dx, dy] = [to[0] - from[0], to[1] - from[1]];
  const len = Math.hypot(dx, dy);
  const [nx, ny] = [(-dy / len) * half, (dx / len) * half];
  const quad: Vec2[] = [
    [from[0] + nx, from[1] + ny],
    [to[0] + nx, to[1] + ny],
    [to[0] - nx, to[1] - ny],
    [from[0] - nx, from[1] - ny],
  ];
  return prism(quad, z0, z1);
};

/** A ring between radii `r0` and `r1` around `c`, from `z0` to `z1`. */
export const ring = (c: Vec2, r0: number, r1: number, z0: number, z1: number): Shape =>
  k.cut(cyl(c, r1, z0, z1), cyl(c, r0, z0 - 0.01, z1 + 0.01));

/** A rectangular frame: the box `w` x `h` minus the box shrunk by `wall` on every side. */
export const frame = (c: Vec2, w: number, h: number, wall: number, z: [number, number]): Shape =>
  k.cut(box(c, w + 2 * wall, h + 2 * wall, z[0], z[1]), box(c, w, h, z[0] - 0.01, z[1] + 0.01));

/** Fuses every shape of `tools` into `base`, then cuts every shape of `cuts` out of it. */
export const carve = (base: Shape, tools: Shape[], cuts: Shape[]): Shape => {
  const whole = tools.length ? k.fuseAll([base, ...tools]) : base;
  return cuts.length ? k.cutAll(whole, cuts) : whole;
};
