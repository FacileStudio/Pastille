import type * as THREE from "three";
import type { GLTFLoader } from "three/examples/jsm/loaders/GLTFLoader.js";
import { PARTS, type ModelSource, type PartName } from "$lib/explode";
import { partOffset } from "$lib/explode-math";
import { disposeObject } from "$lib/pcb-scene";
import { enrichBoard, styleAssemblyParts } from "$lib/pcb-components";

type Three = typeof import("three");

export interface PartNode {
  node: THREE.Object3D;
  rest: THREE.Vector3;
  basis: THREE.Matrix3;
}

export interface ExplodeModel {
  root: THREE.Group;
  parts: Map<PartName, PartNode>;
  anchor: THREE.Vector3;
  anchorPart: PartName | null;
  restSphere: THREE.Sphere;
}

/**
 * Moves every found part from its rest position along its offset at step position `s`.
 */
export function applyParts(model: ExplodeModel, s: number, tmp: THREE.Vector3) {
  for (const [name, part] of model.parts) {
    partOffset(name, s, tmp).applyMatrix3(part.basis);
    part.node.position.copy(part.rest).add(tmp);
  }
  model.root.updateMatrixWorld(true);
}

/**
 * Loads one glTF source, wrapping it in a group named after its part when the source names one.
 */
async function loadSource(T: Three, loader: GLTFLoader, src: ModelSource) {
  const gltf = await loader.loadAsync(src.url);
  if (!src.name) return gltf.scene;
  if (src.rotation) gltf.scene.rotation.set(...src.rotation);
  if (src.position) gltf.scene.position.set(...src.position);
  const group = new T.Group();
  group.name = src.name;
  group.add(gltf.scene);
  return group;
}

/**
 * Loads every source of one candidate list, or disposes what loaded and throws if any file fails.
 */
async function loadAll(T: Three, loader: GLTFLoader, sources: ModelSource[]) {
  const results = await Promise.allSettled(sources.map((src) => loadSource(T, loader, src)));
  const loaded = results.flatMap((r) => (r.status === "fulfilled" ? [r.value] : []));
  if (loaded.length === sources.length) return loaded;
  loaded.forEach(disposeObject);
  throw new Error("model source failed to load");
}

/**
 * Finds the named parts, records their rest pose, and measures the rest bounds.
 */
function buildModel(T: Three, objects: THREE.Object3D[]): ExplodeModel {
  const root = new T.Group();
  const explicitMain = objects.find((obj) => obj.name === "main");
  for (const obj of objects) {
    if (explicitMain && obj !== explicitMain) {
      const dummy = obj.getObjectByName("main");
      if (dummy && dummy.parent) dummy.parent.remove(dummy);
    }
    root.add(obj);
  }
  enrichBoard(T, root);
  styleAssemblyParts(T, root);
  root.updateMatrixWorld(true);
  const parts = new Map<PartName, PartNode>();
  for (const name of PARTS) {
    const node = root.getObjectByName(name);
    if (!node || !node.parent) continue;
    const toParent = new T.Matrix4().copy(node.parent.matrixWorld).invert();
    parts.set(name, { node, rest: node.position.clone(), basis: new T.Matrix3().setFromMatrix4(toParent) });
  }
  const lcd = parts.get("lcd")?.node;
  const box = new T.Box3().setFromObject(lcd ?? root);
  const anchor = new T.Vector3((box.min.x + box.max.x) / 2, (box.min.y + box.max.y) / 2, box.max.z);
  const restSphere = new T.Box3().setFromObject(root).getBoundingSphere(new T.Sphere());
  const anchorPart = lcd ? "lcd" : null;
  return { root, parts, anchor, anchorPart, restSphere };
}

/**
 * Loads the first candidate source list that resolves completely and builds the explode model.
 */
export async function loadExplodeModel(T: Three, loader: GLTFLoader, candidates: ModelSource[][]) {
  for (const sources of candidates) {
    if (sources.length === 0) continue;
    try {
      return buildModel(T, await loadAll(T, loader, sources));
    } catch {
      continue;
    }
  }
  throw new Error("no explode model could be loaded");
}
