import { createViewport, demandLoop, stepTransition, type Transition } from "$lib/pcb-viewport";

export type Preset = "iso" | "top" | "side";

export interface SceneControl {
  applyPreset: (preset: Preset) => void;
  setRotating: (value: boolean) => void;
  setInteractive: (value: boolean) => void;
  loadUrl: (url: string) => Promise<void>;
  dispose: () => void;
}

export interface SceneHooks {
  onUserInteract: () => void;
  onHidden: () => void;
}

type Vec3 = import("three").Vector3;

/**
 * Configures three-point studio lighting on a Three.js scene.
 */
export function addLights(THREE: typeof import("three"), scene: import("three").Scene) {
  scene.add(new THREE.HemisphereLight(0xffffff, 0x181818, 1.6));
  const key = new THREE.DirectionalLight(0xffffff, 2.5);
  key.position.set(70, 130, 90);
  scene.add(key);
  const fill = new THREE.DirectionalLight(0xd4e2dc, 1.0);
  fill.position.set(-90, 70, -70);
  scene.add(fill);
  const rim = new THREE.DirectionalLight(0xffffff, 0.6);
  rim.position.set(0, -80, -60);
  scene.add(rim);
}

/**
 * Releases geometry, material, and texture allocations for a Three.js object tree.
 */
export function disposeObject(root: import("three").Object3D) {
  root.traverse((obj: unknown) => {
    const mesh = obj as {
      isMesh?: boolean;
      geometry?: { dispose: () => void };
      material?: unknown;
    };
    if (!mesh.isMesh) return;
    mesh.geometry?.dispose();
    const list = Array.isArray(mesh.material) ? mesh.material : [mesh.material];
    for (const mat of list) {
      if (!mat || typeof mat !== "object") continue;
      for (const val of Object.values(mat as Record<string, unknown>)) {
        const tex = val as { isTexture?: boolean; dispose?: () => void };
        if (tex?.isTexture && tex.dispose) tex.dispose();
      }
      const disposable = mat as { dispose?: () => void };
      disposable.dispose?.();
    }
  });
}

/**
 * Mounts an interactive Three.js glTF PCB viewer that renders only while something moves on screen.
 */
export async function mountPcbScene(
  host: HTMLDivElement,
  initialUrl: string,
  initialRotate: boolean,
  hooks: SceneHooks,
): Promise<SceneControl> {
  const [THREE, { OrbitControls }, { GLTFLoader }] = await Promise.all([
    import("three"),
    import("three/examples/jsm/controls/OrbitControls.js"),
    import("three/examples/jsm/loaders/GLTFLoader.js"),
  ]);

  const { renderer, camera, controls } = createViewport(THREE, OrbitControls, host, hooks.onUserInteract);
  const scene = new THREE.Scene();
  addLights(THREE, scene);

  const directions: Record<Preset, Vec3> = {
    iso: new THREE.Vector3(1, 0.92, 1).normalize(),
    top: new THREE.Vector3(0, 1, 0.0015).normalize(),
    side: new THREE.Vector3(1, 0.34, 0.08).normalize(),
  };

  let currentModel: import("three").Object3D | null = null;
  let center = new THREE.Vector3();
  let distance = 60;
  let transition: Transition | null = null;
  const loader = new GLTFLoader();
  let loadSeq = 0;

  function setInteractive(on: boolean) {
    controls.enabled = on;
    renderer.domElement.style.touchAction = on ? "none" : "pan-y";
    renderer.domElement.style.cursor = on ? "grab" : "pointer";
  }

  const loop = demandLoop(
    renderer,
    host,
    () => {
      if (transition && !stepTransition(transition, camera, controls.target)) transition = null;
      const moved = controls.update();
      renderer.render(scene, camera);
      return moved || controls.autoRotate || transition !== null;
    },
    () => {
      setInteractive(false);
      hooks.onHidden();
    },
  );
  controls.addEventListener("change", loop.wake);
  setInteractive(false);

  function frameModel(model: import("three").Object3D) {
    const sphere = new THREE.Box3().setFromObject(model).getBoundingSphere(new THREE.Sphere());
    center = sphere.center.clone();
    const radius = Math.max(sphere.radius, 1);
    camera.near = radius / 100;
    camera.far = radius * 100;
    camera.updateProjectionMatrix();
    distance = (radius / Math.sin(((camera.fov / 2) * Math.PI) / 180)) * 0.88;
    controls.minDistance = distance * 0.35;
    controls.maxDistance = distance * 3;
    controls.target.copy(center);
    camera.position.copy(center).add(directions.iso.clone().multiplyScalar(distance));
    controls.update();
  }

  async function loadUrl(url: string) {
    const seq = ++loadSeq;
    const gltf = await loader.loadAsync(url);
    if (seq !== loadSeq) {
      disposeObject(gltf.scene);
      return;
    }
    if (currentModel) {
      scene.remove(currentModel);
      disposeObject(currentModel);
    }
    currentModel = gltf.scene;
    scene.add(currentModel);
    transition = null;
    frameModel(currentModel);
    loop.wake();
  }

  function applyPreset(next: Preset) {
    transition = {
      start: performance.now(),
      fromPos: camera.position.clone(),
      toPos: center.clone().add(directions[next].clone().multiplyScalar(distance)),
      fromTarget: controls.target.clone(),
      toTarget: center.clone(),
    };
    loop.wake();
  }

  try {
    await loadUrl(initialUrl);
  } catch (err) {
    loop.dispose();
    controls.dispose();
    renderer.dispose();
    renderer.domElement.remove();
    throw err;
  }
  controls.autoRotate = initialRotate;
  loop.wake();

  return {
    applyPreset,
    setRotating: (val: boolean) => {
      controls.autoRotate = val;
      loop.wake();
    },
    setInteractive,
    loadUrl,
    dispose: () => {
      loop.dispose();
      controls.dispose();
      if (currentModel) disposeObject(currentModel);
      renderer.dispose();
      renderer.domElement.remove();
    },
  };
}
