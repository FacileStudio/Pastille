import { readGlb, writeGlb, type Gltf } from "./glb";

type Scene = { nodes: number[] };
type Node = { name?: string; children?: number[]; rotation?: number[] };
type SceneGltf = Gltf & { scene?: number; scenes: Scene[]; nodes: Node[] };

const HALF_TURN_ABOUT_Y = [0, 1, 0, 0];

/** Turns the board GLB half a turn about Y, so board +Y points away from a front camera and the silkscreen reads. */
const upright = async (input: string, output: string): Promise<void> => {
  const { gltf, bin } = await readGlb(input);
  const doc = gltf as SceneGltf;
  const scene = doc.scenes[doc.scene ?? 0];
  doc.nodes.push({ name: "board", children: scene.nodes, rotation: HALF_TURN_ABOUT_Y });
  scene.nodes = [doc.nodes.length - 1];
  await writeGlb(output, doc, bin);
};

await upright(Bun.argv[2], Bun.argv[3]);
