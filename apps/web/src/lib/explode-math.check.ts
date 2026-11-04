import { Vector3 } from "three";
import { STEPS } from "$lib/explode";
import { partOffset, rotationAt } from "$lib/explode-math";

/**
 * Throws when `ok` is false, naming the failed expectation.
 */
function expect(ok: boolean, what: string) {
  if (!ok) throw new Error(`explode-math: ${what}`);
}

const out = new Vector3();
const last = STEPS.length - 1;
const bezel = STEPS[1].offsets.front_bezel ?? [0, 0, 0];
const shell = STEPS[last].offsets.back_shell ?? [0, 0, 0];

expect(partOffset("front_bezel", 0, out).length() === 0, "every part rests at step 1");
expect(partOffset("front_bezel", 1, out).z === bezel[2], "the bezel reaches its offset at step 2");
expect(partOffset("back_shell", last - 1, out).length() === 0, "the shell waits for the last step");
expect(partOffset("back_shell", last, out).z === shell[2], "the shell reaches its offset at the last step");
expect(partOffset("front_bezel", last, out).z === bezel[2], "earlier offsets hold to the end");
expect(rotationAt(0).pitch === 0 && rotationAt(1).pitch === STEPS[1].rotation.pitch, "pitch keyframes");
expect(Math.abs(partOffset("front_bezel", 0.5, out).z - bezel[2] / 2) < 1e-9, "ease is symmetric");

console.log("explode-math: ok");
