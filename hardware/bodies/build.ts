import { frontBezel, lens } from "./bezel";
import { k, type Shape } from "./geom";
import { battery, lcd, lra, speaker } from "./refs";
import { backShell } from "./shell";
import { stackOf, type Kit, type Stack } from "./stack";

const PLA_G_PER_MM3 = 1.24e-3;
const STL_DEFLECTION = 0.05;
const ROOT = `${import.meta.dir}/../..`;
const STEP_STAMP = /(FILE_NAME\('[^']*',')[^']*'/;
const STEP_TRANSLATOR = /Open CASCADE STEP translator 8\.0 \d+/g;
const STEP_REAL = /'[^']*'|-?\d+\.\d*(?:E[+-]?\d+)?/g;
const STEP_DIGITS = 10;
const STEP_ZERO = 1e-10;
const STL_GRID = 1e4;

/** Reprints a STEP real at `STEP_DIGITS` significant digits so last-bit float noise cannot change the file. */
const stepReal = (text: string): string => {
  if (text.startsWith("'")) return text;
  const x = Number(text);
  const [mantissa, exponent] = String(Math.abs(x) < STEP_ZERO ? 0 : Number(x.toPrecision(STEP_DIGITS)))
    .toUpperCase()
    .split("E");
  return `${mantissa.includes(".") ? mantissa : `${mantissa}.`}${exponent ? `E${exponent}` : ""}`;
};

/** Pins the STEP timestamp and translator counter and rounds every real, so a rebuild is byte-identical. */
const stableStep = (step: string): string =>
  step
    .replace(STEP_STAMP, (_, head) => `${head}1970-01-01T00:00:00'`)
    .replace(STEP_TRANSLATOR, "Open CASCADE STEP translator 8.0 1")
    .replace(STEP_REAL, stepReal);

/** Snaps every normal and vertex float of a binary STL to a 1/`STL_GRID` mm grid. */
const stableStl = (stl: Uint8Array): Uint8Array => {
  const out = stl.slice();
  const view = new DataView(out.buffer, out.byteOffset, out.byteLength);
  const floats = view.getUint32(80, true) * 12;
  for (let i = 0; i < floats; i++) {
    const at = 84 + Math.floor(i / 12) * 50 + (i % 12) * 4;
    view.setFloat32(at, Math.round(view.getFloat32(at, true) * STL_GRID) / STL_GRID || 0, true);
  }
  return out;
};

const BODIES: Record<string, (s: Stack) => Shape> = {
  front_bezel: frontBezel,
  lens,
  lcd,
  speaker,
  lra,
  battery,
  back_shell: backShell,
};

const write = async (name: string, shape: Shape): Promise<void> => {
  if (!k.isValid(shape)) throw new Error(`${name} is not a valid solid`);
  const box = k.getBoundingBox(shape);
  const size = [box.xmax - box.xmin, box.ymax - box.ymin, box.zmax - box.zmin].map((v) => v.toFixed(2));
  const grams = (Math.abs(k.getVolume(shape)) * PLA_G_PER_MM3).toFixed(2);
  await Bun.write(`${ROOT}/bodies/${name}.step`, stableStep(k.exportStep(shape)));
  await Bun.write(`${ROOT}/bodies/${name}.stl`, stableStl(k.exportStl(shape, STL_DEFLECTION)));
  console.log(`${name}: ${size.join(" x ")} mm, z ${box.zmin.toFixed(2)}..${box.zmax.toFixed(2)}, ${grams} g PLA`);
};

const main = async (names: string[]): Promise<void> => {
  const kit = (await Bun.file(`${ROOT}/bodies/kit/main/kit.json`).json()) as Kit;
  const stack = stackOf(kit);
  const wanted = names.length ? names : Object.keys(BODIES);
  for (const name of wanted) {
    const build = BODIES[name];
    if (!build) throw new Error(`unknown body ${name}; expected one of ${Object.keys(BODIES).join(", ")}`);
    await write(name, build(stack));
  }
};

await main(Bun.argv.slice(2));
