import { strFromU8, strToU8, unzipSync, zipSync } from "fflate";
import { withLayerSpan, withPours, type Pour } from "./gerber";
import { bomCsv, cplCsv } from "./parts";

const ROOT = `${import.meta.dir}/../..`;
const FAB = `${ROOT}/dist/fab`;
const COPPER_LAYERS = 4;
const FIXED_DATE = new Date("1980-01-01T00:00:00Z");

const readGerbers = async (): Promise<Record<string, string>> => {
  const zip = unzipSync(new Uint8Array(await Bun.file(`${FAB}/gerbers.zip`).arrayBuffer()));
  return Object.fromEntries(Object.entries(zip).map(([name, data]) => [name, strFromU8(data)]));
};

const writeGerbers = async (files: Record<string, string>): Promise<void> => {
  const entries = Object.keys(files).sort().map((name) => [name, [strToU8(files[name]), { mtime: FIXED_DATE }]]);
  await Bun.write(`${FAB}/gerbers.zip`, zipSync(Object.fromEntries(entries)));
};

const main = async (): Promise<void> => {
  const circuit = (await Bun.file(`${FAB}/circuit.json`).json()) as Record<string, any>[];
  const pours = circuit.filter((e) => e.type === "pcb_copper_pour") as Pour[];
  const files = withLayerSpan(withPours(await readGerbers(), pours), COPPER_LAYERS);
  if (pours.some((p) => p.layer === "bottom") && !files["B_Cu.gbr"].includes("G36*")) {
    throw new Error("B_Cu.gbr has no ground pour region");
  }
  await writeGerbers(files);
  await Bun.write(`${FAB}/bom.csv`, bomCsv(circuit));
  await Bun.write(`${FAB}/cpl.csv`, cplCsv(circuit));
  console.log(`fab: ${pours.length} pour(s) written into the gerbers, bom.csv and cpl.csv in dist/fab`);
};

await main();
