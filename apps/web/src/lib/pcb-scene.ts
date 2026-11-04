export type Preset = "iso" | "top" | "side";

export interface SceneControl {
  applyPreset: (preset: Preset) => void;
  setRotating: (value: boolean) => void;
  loadUrl: (url: string) => Promise<void>;
  dispose: () => void;
}

/**
 * Configures three-point studio lighting on a Three.js scene.
 */
function addLights(THREE: typeof import("three"), scene: import("three").Scene) {
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
function disposeObject(root: import("three").Object3D) {
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
 * Creates a WebGL renderer and OrbitControls instance inside the given container.
 */
function createViewport(
  THREE: typeof import("three"),
  OrbitControls: typeof import("three/examples/jsm/controls/OrbitControls.js").OrbitControls,
  host: HTMLDivElement,
  onUserInteract: () => void,
) {
  const width = host.clientWidth || 800;
  const height = host.clientHeight || 520;
  const renderer = new THREE.WebGLRenderer({
    antialias: true,
    alpha: true,
    powerPreference: "high-performance",
  });
  renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, 2));
  renderer.setSize(width, height, false);
  renderer.toneMapping = THREE.ACESFilmicToneMapping;
  renderer.toneMappingExposure = 1.08;
  renderer.domElement.style.display = "block";
  renderer.domElement.style.width = "100%";
  renderer.domElement.style.height = "100%";
  renderer.domElement.style.cursor = "grab";
  host.appendChild(renderer.domElement);

  const camera = new THREE.PerspectiveCamera(32, width / height, 0.1, 5000);
  const controls = new OrbitControls(camera, renderer.domElement);
  controls.enableDamping = true;
  controls.dampingFactor = 0.075;
  controls.rotateSpeed = 0.8;
  controls.zoomSpeed = 0.7;
  controls.enablePan = false;
  controls.autoRotateSpeed = 1.4;
  controls.minPolarAngle = 0.02;
  controls.maxPolarAngle = Math.PI - 0.02;
  controls.addEventListener("start", () => {
    controls.autoRotate = false;
    onUserInteract();
  });
  return { renderer, camera, controls };
}

/**
 * Mounts an interactive Three.js glTF PCB viewer inside a host element.
 */
export async function mountPcbScene(
  host: HTMLDivElement,
  initialUrl: string,
  initialRotate: boolean,
  onUserInteract: () => void,
): Promise<SceneControl> {
  const [THREE, { OrbitControls }, { GLTFLoader }] = await Promise.all([
    import("three"),
    import("three/examples/jsm/controls/OrbitControls.js"),
    import("three/examples/jsm/loaders/GLTFLoader.js"),
  ]);

  const { renderer, camera, controls } = createViewport(
    THREE,
    OrbitControls,
    host,
    onUserInteract,
  );
  const scene = new THREE.Scene();
  addLights(THREE, scene);

  const directions: Record<Preset, import("three").Vector3> = {
    iso: new THREE.Vector3(1, 0.92, 1).normalize(),
    top: new THREE.Vector3(0, 1, 0.0015).normalize(),
    side: new THREE.Vector3(1, 0.34, 0.08).normalize(),
  };

  let currentModel: import("three").Object3D | null = null;
  let center = new THREE.Vector3();
  let distance = 60;
  let transition: {
    start: number;
    fromPos: import("three").Vector3;
    toPos: import("three").Vector3;
    fromTarget: import("three").Vector3;
    toTarget: import("three").Vector3;
  } | null = null;

  const loader = new GLTFLoader();

  async function loadUrl(url: string) {
    const gltf = await loader.loadAsync(url);
    if (currentModel) {
      scene.remove(currentModel);
      disposeObject(currentModel);
    }
    currentModel = gltf.scene;
    scene.add(currentModel);

    const box = new THREE.Box3().setFromObject(currentModel);
    const sphere = box.getBoundingSphere(new THREE.Sphere());
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

  await loadUrl(initialUrl);
  controls.autoRotate = initialRotate;
  controls.update();

  function applyPreset(next: Preset) {
    const toPos = center.clone().add(directions[next].clone().multiplyScalar(distance));
    transition = {
      start: performance.now(),
      fromPos: camera.position.clone(),
      toPos,
      fromTarget: controls.target.clone(),
      toTarget: center.clone(),
    };
  }

  renderer.setAnimationLoop(() => {
    if (transition) {
      const p = Math.min(1, (performance.now() - transition.start) / 650);
      const eased = 1 - Math.pow(1 - p, 3);
      camera.position.lerpVectors(transition.fromPos, transition.toPos, eased);
      controls.target.lerpVectors(transition.fromTarget, transition.toTarget, eased);
      if (p >= 1) transition = null;
    }
    controls.update();
    renderer.render(scene, camera);
  });

  const observer = new ResizeObserver(() => {
    const w = host.clientWidth;
    const h = host.clientHeight;
    if (!w || !h) return;
    camera.aspect = w / h;
    camera.updateProjectionMatrix();
    renderer.setSize(w, h, false);
  });
  observer.observe(host);

  return {
    applyPreset,
    setRotating: (val: boolean) => {
      controls.autoRotate = val;
    },
    loadUrl,
    dispose: () => {
      observer.disconnect();
      renderer.setAnimationLoop(null);
      controls.dispose();
      if (currentModel) disposeObject(currentModel);
      renderer.dispose();
      renderer.domElement.remove();
    },
  };
}
