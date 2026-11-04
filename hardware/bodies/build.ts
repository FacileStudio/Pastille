import { frontBezel, lens } from "./bezel";
import { k, type Shape } from "./geom";
import { battery, lcd, lra, speaker } from "./refs";
import { backShell } from "./shell";
import { stackOf, type Kit, type Stack } from "./stack";

const PLA_G_PER_MM3 = 1.24e-3;
const STL_DEFLECTION = 0.05;
const ROOT = `${import.meta.dir}/../..`;

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
  await Bun.write(`${ROOT}/bodies/${name}.step`, k.exportStep(shape));
  await Bun.write(`${ROOT}/bodies/${name}.stl`, k.exportStl(shape, STL_DEFLECTION));
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
