import { strFromU8, unzipSync } from "fflate";

const ROOT = `${import.meta.dir}/../..`;
const RELEASE = `${ROOT}/.github/workflows/release.yml`;

const releaseFiles = async (): Promise<string[]> => {
  const text = await Bun.file(RELEASE).text();
  const block = text.split("files: |")[1] ?? "";
  return block
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => /^[\w./-]+\.\w+$/.test(line));
};

const gerberProblems = async (): Promise<string[]> => {
  const zip = unzipSync(new Uint8Array(await Bun.file(`${ROOT}/dist/fab/gerbers.zip`).arrayBuffer()));
  const text = (name: string) => (zip[name] ? strFromU8(zip[name]) : "");
  const problems: string[] = [];
  const pour = "B_Cu.gbr has no ground pour region (run bun hardware/fab/fab.ts)";
  if (!text("B_Cu.gbr").includes("G36*")) problems.push(pour);
  if (!text("drill_pth.drl").includes("Plated,1,4,PTH")) problems.push("drill_pth.drl does not span 4 layers");
  return problems;
};

/** Fails when a release file is missing or empty, or when the committed Gerbers lack the pour or the layer span. */
const verify = async (): Promise<void> => {
  const files = await releaseFiles();
  const sizes = await Promise.all(files.map((path) => Bun.file(`${ROOT}/${path}`).size));
  const missing = files.filter((_, i) => !sizes[i]).map((path) => `${path} is missing or empty`);
  const problems = [...missing, ...(await gerberProblems())];
  if (!files.length) problems.push("release.yml lists no files");
  if (problems.length) throw new Error(problems.join("\n"));
  console.log(`verify: ${files.length} release files present, gerbers carry the pour and a 4-layer drill span`);
};

await verify();
