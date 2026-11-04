import { concat, readGlb, writeGlb, type Accessor, type Gltf, type Primitive } from "./glb";

type Merged = { position: Float32Array; normal: Float32Array; index: Uint32Array; material?: number };

const FLOAT = 5126;
const UINT = 5125;
const USHORT = 5123;

const read = (gltf: Gltf, bin: Uint8Array, id: number): Float32Array | Uint32Array => {
  const a = gltf.accessors[id];
  const view = gltf.bufferViews[a.bufferView];
  const start = bin.byteOffset + (view.byteOffset ?? 0) + (a.byteOffset ?? 0);
  const width = a.type === "VEC3" ? 3 : 1;
  if (a.componentType === FLOAT) return new Float32Array(bin.buffer.slice(start, start + a.count * width * 4));
  if (a.componentType === USHORT) {
    return Uint32Array.from(new Uint16Array(bin.buffer.slice(start, start + a.count * 2)));
  }
  return new Uint32Array(bin.buffer.slice(start, start + a.count * 4));
};

const merge = (gltf: Gltf, bin: Uint8Array, prims: Primitive[]): Merged => {
  const parts = prims.map((p) => ({
    position: read(gltf, bin, p.attributes.POSITION) as Float32Array,
    normal: read(gltf, bin, p.attributes.NORMAL) as Float32Array,
    index: read(gltf, bin, p.indices) as Uint32Array,
  }));
  const total = (key: "position" | "normal" | "index") => parts.reduce((n, p) => n + p[key].length, 0);
  const out = { position: new Float32Array(total("position")), normal: new Float32Array(total("normal")) };
  const index = new Uint32Array(total("index"));
  let [v, i] = [0, 0];
  for (const p of parts) {
    out.position.set(p.position, v);
    out.normal.set(p.normal, v);
    index.set(p.index.map((n) => n + v / 3), i);
    [v, i] = [v + p.position.length, i + p.index.length];
  }
  return { ...out, index, material: prims[0]?.material };
};

const bounds = (position: Float32Array) => {
  const min = [Infinity, Infinity, Infinity];
  const max = [-Infinity, -Infinity, -Infinity];
  position.forEach((value, n) => {
    min[n % 3] = Math.min(min[n % 3], value);
    max[n % 3] = Math.max(max[n % 3], value);
  });
  return { min, max };
};

const pack = (gltf: Gltf, meshes: Merged[]): Uint8Array => {
  const chunks: Uint8Array[] = [];
  let offset = 0;
  const add = (data: Float32Array | Uint32Array, accessor: Omit<Accessor, "bufferView">, target: number) => {
    gltf.bufferViews.push({ buffer: 0, byteOffset: offset, byteLength: data.byteLength, target });
    gltf.accessors.push({ ...accessor, bufferView: gltf.bufferViews.length - 1 });
    chunks.push(new Uint8Array(data.buffer, data.byteOffset, data.byteLength));
    offset += data.byteLength;
    return gltf.accessors.length - 1;
  };
  [gltf.accessors, gltf.bufferViews] = [[], []];
  gltf.meshes = meshes.map((m) => {
    const count = m.position.length / 3;
    const POSITION = add(m.position, { componentType: FLOAT, count, type: "VEC3", ...bounds(m.position) }, 34962);
    const NORMAL = add(m.normal, { componentType: FLOAT, count, type: "VEC3" }, 34962);
    const indices = add(m.index, { componentType: UINT, count: m.index.length, type: "SCALAR" }, 34963);
    return { primitives: [{ attributes: { POSITION, NORMAL }, indices, material: m.material, mode: 4 }] };
  });
  gltf.buffers = [{ byteLength: offset }];
  return concat(chunks);
};

/** Merges every mesh of the assembly GLB `input` into one primitive per part and writes it to `output`. */
const slim = async (input: string, output: string): Promise<void> => {
  const { gltf, bin } = await readGlb(input);
  const merged = gltf.meshes.map((m) => merge(gltf, bin, m.primitives));
  await writeGlb(output, gltf, pack(gltf, merged));
};

await slim(Bun.argv[2], Bun.argv[3]);
