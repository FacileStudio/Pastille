import type * as THREE from "three";
import { blobUri } from "@facile/blob";

type Three = typeof import("three");

export function enrichBoard(T: Three, root: THREE.Object3D) {
  if (root.getObjectByName("U_ESP32_S3")) return;
  const board = root.getObjectByName("board") ?? root;

  const mcu = new T.Group();
  mcu.name = "U_ESP32_S3";

  const shieldGeom = new T.BoxGeometry(12.0, 2.2, 13.2);
  const shieldMat = new T.MeshStandardMaterial({ color: 0xd0d4dc, metalness: 0.85, roughness: 0.25 });
  const shield = new T.Mesh(shieldGeom, shieldMat);
  shield.position.set(-1.17, 1.1, 0);
  mcu.add(shield);

  const antGeom = new T.BoxGeometry(4.6, 0.9, 13.2);
  const antMat = new T.MeshStandardMaterial({ color: 0x141414, roughness: 0.85 });
  const ant = new T.Mesh(antGeom, antMat);
  ant.position.set(7.17, 0.45, 0);
  mcu.add(ant);

  mcu.position.set(3.47, 0.8, 0);
  board.add(mcu);

  const micGeom = new T.BoxGeometry(2.65, 1.0, 3.5);
  const micMat = new T.MeshStandardMaterial({ color: 0xd4af37, metalness: 0.9, roughness: 0.2 });
  const mic = new T.Mesh(micGeom, micMat);
  mic.name = "U_MIC";
  mic.position.set(-13.5, 1.3, -0.8);
  board.add(mic);

  const fpcGeom = new T.BoxGeometry(8.6, 1.0, 3.8);
  const fpcMat = new T.MeshStandardMaterial({ color: 0x222225, roughness: 0.6 });
  const fpc = new T.Mesh(fpcGeom, fpcMat);
  fpc.name = "J_LCD";
  fpc.position.set(0, 1.3, 11.2);
  board.add(fpc);

  const esdGeom = new T.BoxGeometry(2.9, 1.1, 1.6);
  const esdMat = new T.MeshStandardMaterial({ color: 0x18181a, roughness: 0.7 });
  const esd = new T.Mesh(esdGeom, esdMat);
  esd.name = "U_ESD";
  esd.position.set(-7.2, 1.35, -8.6);
  board.add(esd);
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

export function styleAssemblyParts(T: Three, root: THREE.Object3D) {
  for (const [name, createMat] of Object.entries(PART_STYLES)) {
    const node = root.getObjectByName(name);
    if (!node) continue;
    node.traverse((child) => {
      if ((child as THREE.Mesh).isMesh) {
        (child as THREE.Mesh).material = createMat(T);
      }
    });
  }
}

export function attachLcdFace(T: Three, root: THREE.Object3D) {
  if (typeof document === "undefined") return;
  const lcd = root.getObjectByName("lcd");
  if (!lcd || lcd.getObjectByName("lcd_face")) return;

  const canvas = document.createElement("canvas");
  canvas.width = 512;
  canvas.height = 512;
  const ctx = canvas.getContext("2d");
  if (!ctx) return;

  ctx.fillStyle = "#050505";
  ctx.fillRect(0, 0, 512, 512);

  const texture = new T.CanvasTexture(canvas);
  texture.colorSpace = T.SRGBColorSpace;

  const img = new Image();
  img.onload = () => {
    ctx.fillStyle = "#050505";
    ctx.fillRect(0, 0, 512, 512);
    ctx.drawImage(img, 48, 48, 416, 416);
    texture.needsUpdate = true;
  };
  img.src = blobUri("Bulle");

  const geom = new T.CircleGeometry(16.0, 64);
  const mat = new T.MeshBasicMaterial({ map: texture, toneMapped: false });
  const face = new T.Mesh(geom, mat);
  face.name = "lcd_face";
  face.position.set(0, 0, 5.71);
  lcd.add(face);
}
