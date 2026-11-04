import type { Vec2 } from "./geom";

export const PART = {
  usbHeight: 3.26,
  usbShell: 3.16,
  plug: { w: 12.4, h: 6.5, clearance: 0.3 },
  lcd: { glass: 35.6, active: 32.4, thick: 1.5, ledgeReach: 20.3, ledgeHalf: 8 },
  lens: 0.8,
  cell: { w: 20, h: 20, t: 4, swell: 0.5, at: [-1, -5] as Vec2 },
  speaker: { w: 15, h: 11, t: 2.5, seat: 0.3, at: [-0.5, 12] as Vec2 },
  lra: { d: 8, t: 2.7, at: [14.5, 6] as Vec2 },
};

export const FIT = {
  boardGap: 0.3,
  wall: 1.2,
  lip: 0.6,
  ledgeWall: 0.75,
  lipClear: 0.4,
  lipHeight: 1.0,
  floor: 1.0,
  air: 0.14,
  lensGap: 0.1,
  lcdGap: 0.15,
  boss: { shell: 5.0, bezel: 3.8, pilot: 1.6, clear: 2.4, head: 4.2, headDepth: 1.6 },
  mic: { tube: 3.2, port: 1.0, gasket: 0.2, mesh: 3.0, meshDepth: 0.2 },
  probeDepth: 2.0,
  probeNear: 4.0,
  unmeasured: 2.0,
};
