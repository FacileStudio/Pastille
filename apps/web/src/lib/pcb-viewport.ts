type Vec3 = import("three").Vector3;

export interface Transition {
  start: number;
  fromPos: Vec3;
  toPos: Vec3;
  fromTarget: Vec3;
  toTarget: Vec3;
}

/**
 * Creates a WebGL renderer and OrbitControls instance inside the given container, with orbit off until enabled.
 */
export function createViewport(
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
 * Runs `frame` on every animation frame while the host is on screen and `frame` reports more motion.
 */
export function demandLoop(
  renderer: import("three").WebGLRenderer,
  host: HTMLElement,
  frame: () => boolean,
  onHidden: () => void,
) {
  let visible = false;
  let running = false;
  const stop = () => {
    running = false;
    renderer.setAnimationLoop(null);
  };
  const wake = () => {
    if (!visible || running) return;
    running = true;
    renderer.setAnimationLoop(() => {
      if (!frame()) stop();
    });
  };
  const observer = new IntersectionObserver(([entry]) => {
    visible = entry?.isIntersecting ?? false;
    if (visible) wake();
    else {
      stop();
      onHidden();
    }
  });
  observer.observe(host);
  const resize = new ResizeObserver(wake);
  resize.observe(host);
  return {
    wake,
    dispose: () => {
      observer.disconnect();
      resize.disconnect();
      stop();
    },
  };
}

/**
 * Moves the camera along a running preset transition and clears it once it lands.
 */
export function stepTransition(t: Transition, camera: import("three").Camera, target: Vec3) {
  const p = Math.min(1, (performance.now() - t.start) / 650);
  const eased = 1 - Math.pow(1 - p, 3);
  camera.position.lerpVectors(t.fromPos, t.toPos, eased);
  target.lerpVectors(t.fromTarget, t.toTarget, eased);
  return p < 1;
}
