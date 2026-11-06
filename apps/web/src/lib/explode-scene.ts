import type * as THREE from "three";
import { ASSEMBLY_SOURCES, STEPS } from "$lib/explode";
import { clamp01, rotationAt } from "$lib/explode-math";
import { applyParts, loadExplodeModel, type ExplodeModel } from "$lib/explode-model";
import { addLights, disposeObject } from "$lib/pcb-scene";

type Three = typeof import("three");

export interface ExplodeControl {
  setProgress: (t: number) => void;
  dispose: () => void;
}

const DEG = Math.PI / 180;

/**
 * Creates a transparent WebGL renderer sized to its host and appends its canvas.
 */
function createRenderer(T: Three, host: HTMLElement) {
  const renderer = new T.WebGLRenderer({ antialias: true, alpha: true, powerPreference: "high-performance" });
  renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, 2));
  renderer.setSize(host.clientWidth || 600, host.clientHeight || 480, false);
  renderer.toneMapping = T.ACESFilmicToneMapping;
  renderer.toneMappingExposure = 1.08;
  renderer.domElement.style.display = "block";
  renderer.domElement.style.width = "100%";
  renderer.domElement.style.height = "100%";
  host.appendChild(renderer.domElement);
  return renderer;
}

/**
 * Renders the explode model on demand: one frame per progress or size change, never a loop.
 */
class ExplodeScene {
  private scene: THREE.Scene;
  private camera: THREE.PerspectiveCamera;
  private pivot: THREE.Group;
  private tmp: THREE.Vector3;
  private box: THREE.Box3;
  private sphere: THREE.Sphere;
  private progress = 0;
  private frameId = 0;

  constructor(
    T: Three,
    private host: HTMLElement,
    private renderer: THREE.WebGLRenderer,
    private model: ExplodeModel,
  ) {
    this.scene = new T.Scene();
    addLights(T, this.scene);
    this.camera = new T.PerspectiveCamera(30, 1, 0.1, 5000);
    this.pivot = new T.Group();
    const frame = new T.Group();
    frame.position.copy(model.restSphere.center).negate();
    frame.add(model.root);
    this.pivot.add(frame);
    this.scene.add(this.pivot);
    this.tmp = new T.Vector3();
    this.box = new T.Box3();
    this.sphere = new T.Sphere();
    this.resize();
  }

  setProgress(t: number) {
    this.progress = clamp01(t);
    this.schedule();
  }

  resize() {
    const w = this.host.clientWidth;
    const h = this.host.clientHeight;
    if (!w || !h) return;
    this.camera.aspect = w / h;
    this.renderer.setSize(w, h, false);
    this.schedule();
  }

  schedule() {
    if (this.frameId) return;
    this.frameId = requestAnimationFrame(() => this.render());
  }

  private render() {
    this.frameId = 0;
    const s = this.progress * (STEPS.length - 1);
    applyParts(this.model, s, this.tmp);
    const rot = rotationAt(s);
    this.pivot.rotation.set(-rot.pitch * DEG, rot.yaw * DEG, 0);
    this.pivot.updateMatrixWorld(true);
    this.frameCamera();
    this.renderer.render(this.scene, this.camera);
  }

  private frameCamera() {
    const { center, radius } = this.box.setFromObject(this.model.root).getBoundingSphere(this.sphere);
    const half = Math.atan(Math.tan((this.camera.fov / 2) * DEG) * Math.min(1, this.camera.aspect));
    const distance = (Math.max(radius, 1) / Math.sin(half)) * 0.92;
    this.camera.near = distance / 100;
    this.camera.far = distance * 10;
    this.camera.position.set(center.x, center.y, center.z + distance);
    this.camera.lookAt(center);
    this.camera.updateProjectionMatrix();
  }

  dispose() {
    cancelAnimationFrame(this.frameId);
    this.frameId = 0;
    disposeObject(this.model.root);
    this.renderer.dispose();
    this.renderer.domElement.remove();
  }
}

/**
 * Loads three.js and the assembly, then mounts an on-demand explode scene inside `host`.
 */
export async function mountExplodeScene(host: HTMLElement): Promise<ExplodeControl> {
  const [T, { GLTFLoader }] = await Promise.all([
    import("three"),
    import("three/examples/jsm/loaders/GLTFLoader.js"),
  ]);
  const model = await loadExplodeModel(T, new GLTFLoader(), [ASSEMBLY_SOURCES, ASSEMBLY_SOURCES.slice(0, 1)]);
  let view: ExplodeScene;
  try {
    view = new ExplodeScene(T, host, createRenderer(T, host), model);
  } catch (err) {
    disposeObject(model.root);
    throw err;
  }
  const observer = new ResizeObserver(() => view.resize());
  observer.observe(host);
  return {
    setProgress: (t) => view.setProgress(t),
    dispose: () => {
      observer.disconnect();
      view.dispose();
    },
  };
}
