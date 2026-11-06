const ROOT = `${import.meta.dir}/../..`;
const TOLERANCE = 1e-3;

interface Mesh {
  triangles: number;
  volume: number;
  area: number;
  box: number[];
}

/**
 * Returns the signed volume of the tetrahedron spanned by the origin and one triangle.
 */
function tetra([a, b, c]: number[][]): number {
  const cross = [b[1] * c[2] - b[2] * c[1], b[2] * c[0] - b[0] * c[2], b[0] * c[1] - b[1] * c[0]];
  return (a[0] * cross[0] + a[1] * cross[1] + a[2] * cross[2]) / 6;
}

/**
 * Reads a binary STL and returns its triangle count, enclosed volume, surface area and bounding box.
 */
function measure(stl: Uint8Array): Mesh {
  const view = new DataView(stl.buffer, stl.byteOffset, stl.byteLength);
  const triangles = view.getUint32(80, true);
  const box = [Infinity, Infinity, Infinity, -Infinity, -Infinity, -Infinity];
  let volume = 0;
  let area = 0;
  for (let t = 0; t < triangles; t++) {
    const at = 84 + t * 50 + 12;
    const v = [0, 1, 2].map((i) => [0, 1, 2].map((a) => view.getFloat32(at + i * 12 + a * 4, true)));
    v.flat().forEach((c, i) => {
      box[i % 3] = Math.min(box[i % 3], c);
      box[3 + (i % 3)] = Math.max(box[3 + (i % 3)], c);
    });
    const [a, b, c] = v;
    const u = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    const w = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    const n = [u[1] * w[2] - u[2] * w[1], u[2] * w[0] - u[0] * w[2], u[0] * w[1] - u[1] * w[0]];
    area += Math.hypot(n[0], n[1], n[2]) / 2;
    volume += tetra(v);
  }
  return { triangles, volume: Math.abs(volume), area, box };
}

/**
 * Returns the names of the measures that differ by more than the relative tolerance.
 */
function drift(before: Mesh, after: Mesh): string[] {
  const near = (x: number, y: number) => Math.abs(x - y) <= TOLERANCE * Math.max(1, Math.abs(x), Math.abs(y));
  const out = [];
  if (!near(before.volume, after.volume)) out.push(`volume ${before.volume} -> ${after.volume}`);
  if (!near(before.area, after.area)) out.push(`area ${before.area} -> ${after.area}`);
  if (before.box.some((c, i) => !near(c, after.box[i]))) out.push(`box ${before.box} -> ${after.box}`);
  return out;
}

/**
 * Compares each rebuilt STL in bodies/ with its committed version and exits 1 when the geometry moved.
 */
async function main() {
  const files = [...new Bun.Glob("bodies/*.stl").scanSync({ cwd: ROOT })];
  let failed = false;
  for (const file of files) {
    const committed = Bun.spawnSync(["git", "show", `HEAD:${file}`], { cwd: ROOT });
    if (committed.exitCode !== 0) {
      console.error(`stale: ${file} is not committed`);
      failed = true;
      continue;
    }
    const rebuilt = new Uint8Array(await Bun.file(`${ROOT}/${file}`).arrayBuffer());
    const moved = drift(measure(committed.stdout), measure(rebuilt));
    if (moved.length) console.error(`stale: ${file}: ${moved.join("; ")}`);
    failed ||= moved.length > 0;
  }
  if (failed) throw new Error("stl bodies drifted from their committed geometry");
  console.log(`stale: ${files.length} stl bodies match their committed geometry`);
}

await main();
