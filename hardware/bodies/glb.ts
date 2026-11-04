export type Accessor = { bufferView: number; byteOffset?: number; count: number; componentType: number; type: string };
type View = { buffer: number; byteOffset?: number; byteLength: number; byteStride?: number; target?: number };
export type Primitive = { attributes: Record<string, number>; indices: number; material?: number; mode?: number };
export type Gltf = {
  accessors: Accessor[];
  bufferViews: View[];
  buffers: { byteLength: number }[];
  meshes: { primitives: Primitive[] }[];
};

const GLB_MAGIC = 0x46546c67;
const JSON_CHUNK = 0x4e4f534a;
const BIN_CHUNK = 0x004e4942;

/** Joins byte arrays end to end. */
export const concat = (parts: Uint8Array[]): Uint8Array => {
  const out = new Uint8Array(parts.reduce((n, p) => n + p.length, 0));
  parts.reduce((at, p) => (out.set(p, at), at + p.length), 0);
  return out;
};

const chunk = (type: number, data: Uint8Array, pad: number): Uint8Array => {
  const size = Math.ceil(data.length / 4) * 4;
  const out = new Uint8Array(8 + size).fill(pad, 8);
  const head = new DataView(out.buffer);
  head.setUint32(0, size, true);
  head.setUint32(4, type, true);
  out.set(data, 8);
  return out;
};

/** Reads a GLB file into its glTF JSON and its binary chunk. */
export const readGlb = async (path: string): Promise<{ gltf: Gltf; bin: Uint8Array }> => {
  const glb = new Uint8Array(await Bun.file(path).arrayBuffer());
  const view = new DataView(glb.buffer);
  if (view.getUint32(0, true) !== GLB_MAGIC) throw new Error(`${path} is not a GLB file`);
  const jsonLength = view.getUint32(12, true);
  const gltf = JSON.parse(new TextDecoder().decode(glb.subarray(20, 20 + jsonLength))) as Gltf;
  return { gltf, bin: glb.subarray(28 + jsonLength) };
};

/** Writes `gltf` and its binary buffer `bin` as one GLB file. */
export const writeGlb = async (path: string, gltf: Gltf, bin: Uint8Array): Promise<void> => {
  const body = chunk(BIN_CHUNK, bin, 0);
  const json = chunk(JSON_CHUNK, new TextEncoder().encode(JSON.stringify(gltf)), 0x20);
  const header = new Uint8Array(12);
  const h = new DataView(header.buffer);
  [GLB_MAGIC, 2, 12 + json.length + body.length].forEach((n, i) => h.setUint32(i * 4, n, true));
  await Bun.write(path, concat([header, json, body]));
};
