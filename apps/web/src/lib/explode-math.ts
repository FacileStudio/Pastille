import type { Vector3 } from "three";
import { STEPS, type PartName } from "$lib/explode";

/**
 * Cubic ease-in-out curve on the unit interval.
 */
export const ease = (t: number) => (t < 0.5 ? 4 * t * t * t : 1 - Math.pow(-2 * t + 2, 3) / 2);

/**
 * Clamps a number to the unit interval.
 */
export const clamp01 = (v: number) => Math.min(1, Math.max(0, v));

/**
 * Writes a part's eased explode offset at step position `s` into `out`, in assembly coordinates.
 */
export function partOffset(name: PartName, s: number, out: Vector3) {
  out.set(0, 0, 0);
  for (let k = 1; k < STEPS.length; k++) {
    const off = STEPS[k].offsets[name];
    if (!off) continue;
    const w = ease(clamp01(s - (k - 1)));
    out.x += off[0] * w;
    out.y += off[1] * w;
    out.z += off[2] * w;
  }
  return out;
}

/**
 * Returns the eased pitch and yaw keyframe, in degrees, at step position `s`.
 */
export function rotationAt(s: number) {
  const last = STEPS.length - 1;
  const i = Math.min(last, Math.floor(s));
  const a = STEPS[i].rotation;
  const b = STEPS[Math.min(last, i + 1)].rotation;
  const w = ease(clamp01(s - i));
  return { pitch: a.pitch + (b.pitch - a.pitch) * w, yaw: a.yaw + (b.yaw - a.yaw) * w };
}
