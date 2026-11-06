import type * as THREE from "three";
import { blobUri } from "@facile/blob";

type Three = typeof import("three");

interface OverlayBox {
  size: [number, number, number];
  at: [number, number];
  lift?: number;
  color: number;
  metal?: boolean;
}

interface BoardOverlay {
  name: string;
  pcb: [number, number];
  rotation: number;
  boxes: OverlayBox[];
}

const BOARD_TOP_MM = 0.8;
const DEG = Math.PI / 180;

/**
 * Parts that tscircuit exports without a 3D model, drawn as plain boxes.
 * `pcb` and `rotation` copy pcbX, pcbY and pcbRotation from hardware/subcircuits; `size` is
 * [x, height, y] in millimetres, and `at` is the body centre in the footprint frame.
 * The ESP32-S3-MINI-1 is 15.4 x 20.5 x 2.4 mm (Espressif datasheet), its antenna toward -X.
 */
const BOARD_OVERLAYS: BoardOverlay[] = [
  {
    name: "U_ESP32_S3",
    pcb: [-3.47, 0],
    rotation: 0,
    boxes: [
      { size: [20.5, 0.8, 15.4], at: [-2.55, 0], color: 0x141414 },
      { size: [15.1, 1.6, 15.4], at: [0.15, 0], lift: 0.8, color: 0xd0d4dc, metal: true },
    ],
  },
  { name: "U_MIC", pcb: [10.8, -11.01], rotation: 180, boxes: [
    { size: [2.65, 0.98, 3.5], at: [0, -0.15], color: 0xd4af37, metal: true },
  ] },
  { name: "J_LCD", pcb: [0, 11.2], rotation: 0, boxes: [{ size: [8.6, 1.0, 3.8], at: [0, 0.1], color: 0x222225 }] },
  { name: "U_ESD", pcb: [7.2, -8.6], rotation: 90, boxes: [{ size: [2.9, 1.1, 1.6], at: [0, 0], color: 0x18181a }] },
];

/**
 * Builds one overlay part as a group placed in the GLB board frame (local x = -pcbX, z = pcbY).
 */
function overlayGroup(T: Three, part: BoardOverlay): THREE.Group {
  const group = new T.Group();
  group.name = part.name;
  group.position.set(-part.pcb[0], BOARD_TOP_MM, part.pcb[1]);
  group.rotation.y = part.rotation * DEG;
  for (const box of part.boxes) {
    const material = box.metal
      ? new T.MeshStandardMaterial({ color: box.color, metalness: 0.85, roughness: 0.25 })
      : new T.MeshStandardMaterial({ color: box.color, metalness: 0.1, roughness: 0.75 });
    const mesh = new T.Mesh(new T.BoxGeometry(...box.size), material);
    mesh.position.set(-box.at[0], (box.lift ?? 0) + box.size[1] / 2, box.at[1]);
    group.add(mesh);
  }
  return group;
}

/**
 * Adds the parts the tscircuit GLB lacks to its `board` node, skipping any part it already carries.
 */
export function enrichBoard(T: Three, root: THREE.Object3D) {
  const board = root.getObjectByName("board");
  if (!board) return;
  for (const part of BOARD_OVERLAYS) {
    if (!board.getObjectByName(part.name)) board.add(overlayGroup(T, part));
  }
}

/**
 * Releases a material and every texture it holds.
 */
export function disposeMaterial(material: unknown) {
  if (!material || typeof material !== "object") return;
  for (const val of Object.values(material as Record<string, unknown>)) {
    const tex = val as { isTexture?: boolean; dispose?: () => void };
    if (tex?.isTexture && tex.dispose) tex.dispose();
  }
  (material as { dispose?: () => void }).dispose?.();
}

const PART_STYLES: Record<string, (T: Three) => THREE.Material> = {
  lcd: (T) => new T.MeshStandardMaterial({ color: 0x050505, roughness: 0.25, metalness: 0.1 }),
  battery: (T) => new T.MeshStandardMaterial({ color: 0xc8d0db, metalness: 0.85, roughness: 0.28 }),
  lra: (T) => new T.MeshStandardMaterial({ color: 0xc58f4a, metalness: 0.88, roughness: 0.22 }),
  speaker: (T) => new T.MeshStandardMaterial({ color: 0x1e2026, roughness: 0.75, metalness: 0.2 }),
  lens: (T) =>
    new T.MeshStandardMaterial({
      color: 0xffffff,
      transparent: true,
      opacity: 0,
      roughness: 0.05,
      metalness: 0.1,
    }),
  front_bezel: (T) => new T.MeshStandardMaterial({ color: 0x2d313b, roughness: 0.7, metalness: 0.1 }),
  back_shell: (T) => new T.MeshStandardMaterial({ color: 0x242730, roughness: 0.7, metalness: 0.1 }),
};

/**
 * Gives each printed or bought assembly part one shared material, and disposes the materials it replaces.
 */
export function styleAssemblyParts(T: Three, root: THREE.Object3D) {
  for (const [name, createMat] of Object.entries(PART_STYLES)) {
    const node = root.getObjectByName(name);
    if (!node) continue;
    const material = createMat(T);
    node.traverse((child) => {
      const mesh = child as THREE.Mesh;
      if (!mesh.isMesh) return;
      (Array.isArray(mesh.material) ? mesh.material : [mesh.material]).forEach(disposeMaterial);
      mesh.material = material;
    });
  }
}

/**
 * Draws Bulle's face on a canvas texture once the image has decoded, or a black screen if it fails.
 */
async function faceTexture(T: Three): Promise<THREE.CanvasTexture | null> {
  const canvas = document.createElement("canvas");
  canvas.width = 512;
  canvas.height = 512;
  const ctx = canvas.getContext("2d");
  if (!ctx) return null;
  ctx.fillStyle = "#050505";
  ctx.fillRect(0, 0, 512, 512);
  const img = new Image();
  img.src = blobUri("Bulle");
  try {
    await img.decode();
    ctx.drawImage(img, 48, 48, 416, 416);
  } catch {}
  const texture = new T.CanvasTexture(canvas);
  texture.colorSpace = T.SRGBColorSpace;
  return texture;
}

/**
 * Puts Bulle's face on the LCD part, resolving once the face is drawn so the first frame shows it.
 */
export async function attachLcdFace(T: Three, root: THREE.Object3D) {
  if (typeof document === "undefined") return;
  const lcd = root.getObjectByName("lcd");
  if (!lcd || lcd.getObjectByName("lcd_face")) return;
  const texture = await faceTexture(T);
  if (!texture) return;
  const face = new T.Mesh(
    new T.CircleGeometry(16.0, 64),
    new T.MeshBasicMaterial({ map: texture, toneMapped: false }),
  );
  face.name = "lcd_face";
  face.position.set(0, 0, 5.71);
  lcd.add(face);
}
