<script lang="ts">
  import { onMount } from "svelte";
  import { Button, Spinner } from "@facile/muse";
  import { mountPcbScene, type Preset, type SceneControl } from "$lib/pcb-scene";

  type ModelMode = "pcb" | "case";

  const MODELS: Record<ModelMode, string> = {
    pcb: "/models/pastille.glb?v=0.4.0",
    case: "/models/pastille_assembly.glb?v=0.4.0",
  };

  const PRESETS: Preset[] = ["iso", "top", "side"];

  let host: HTMLDivElement;
  let status = $state<"idle" | "loading" | "ready" | "error">("idle");
  let preset = $state<Preset>("iso");
  let mode = $state<ModelMode>("pcb");
  let rotating = $state(true);
  let viewer: SceneControl | null = null;
  let loadSeq = 0;

  /**
   * Moves the camera to a preset angle and pauses auto-rotation.
   */
  function choosePreset(next: Preset) {
    preset = next;
    rotating = false;
    viewer?.setRotating(false);
    viewer?.applyPreset(next);
  }

  /**
   * Switches between the 3D PCB model and the generated enclosure body.
   */
  async function chooseMode(next: ModelMode) {
    if (mode === next || !viewer) return;
    mode = next;
    status = "loading";
    const seq = ++loadSeq;
    try {
      await viewer.loadUrl(MODELS[next]);
      if (seq !== loadSeq) return;
      preset = "iso";
      status = "ready";
    } catch {
      if (seq === loadSeq) status = "error";
    }
  }

  /**
   * Toggles turntable auto-rotation.
   */
  function toggleRotate() {
    rotating = !rotating;
    viewer?.setRotating(rotating);
  }

  onMount(() => {
    const reduced = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
    rotating = !reduced;
    let disposed = false;

    async function boot() {
      if (disposed || status !== "idle") return;
      status = "loading";
      try {
        const ctrl = await mountPcbScene(host, MODELS[mode], rotating, () => {
          rotating = false;
        });
        if (disposed) {
          ctrl.dispose();
          return;
        }
        viewer = ctrl;
        status = "ready";
      } catch {
        status = "error";
      }
    }

    boot();

    return () => {
      disposed = true;
      viewer?.dispose();
      viewer = null;
    };
  });
</script>

<div class="relative w-full h-[380px] sm:h-[480px] lg:h-[560px] rounded-fc-lg bg-fc-component overflow-hidden">
  <div bind:this={host} class="absolute inset-0" aria-hidden="true"></div>

  <div class="absolute top-0 left-0 right-0 flex flex-wrap items-center justify-between gap-2 p-4 pointer-events-none">
    <div
      class="flex flex-wrap items-center gap-1 rounded-fc-pill bg-fc-surface/90 p-1
        backdrop-blur pointer-events-auto"
    >
      <Button
        size="sm"
        variant={mode === "pcb" ? "primary" : "ghost"}
        class={mode === "pcb" ? "" : "text-fc-fg hover:bg-fc-component"}
        aria-pressed={mode === "pcb"}
        onclick={() => chooseMode("pcb")}
      >
        3D PCB
      </Button>
      <Button
        size="sm"
        variant={mode === "case" ? "primary" : "ghost"}
        class={mode === "case" ? "" : "text-fc-fg hover:bg-fc-component"}
        aria-pressed={mode === "case"}
        onclick={() => chooseMode("case")}
      >
        Enclosure
      </Button>
    </div>

    {#if status === "ready"}
      <div
        class="flex flex-wrap items-center gap-1 rounded-fc-pill bg-fc-surface/90 p-1
          backdrop-blur pointer-events-auto"
      >
        {#each PRESETS as option (option)}
          <Button
            size="sm"
            variant={preset === option ? "primary" : "ghost"}
            class={preset === option ? "uppercase" : "uppercase text-fc-fg hover:bg-fc-component"}
            aria-pressed={preset === option}
            onclick={() => choosePreset(option)}
          >
            {option}
          </Button>
        {/each}
        <Button
          size="sm"
          variant={rotating ? "primary" : "ghost"}
          class={rotating ? "uppercase" : "uppercase text-fc-fg hover:bg-fc-component"}
          aria-pressed={rotating}
          onclick={toggleRotate}
        >
          Auto
        </Button>
      </div>
    {/if}
  </div>

  {#if status === "idle" || status === "loading"}
    <div
      class="absolute inset-0 flex flex-col items-center justify-center gap-3
        bg-fc-component/80 pointer-events-none"
    >
      <Spinner />
      <span class="text-fc-xs font-mono uppercase text-fc-fg-muted">Loading 3D geometry</span>
    </div>
  {:else if status === "error"}
    <div class="absolute inset-0 flex flex-col items-center justify-center gap-2 p-6 text-center">
      <span class="text-fc-sm font-semibold text-fc-fg">3D preview unavailable</span>
      <span class="text-fc-xs text-fc-fg-muted">WebGL could not initialize in this browser.</span>
    </div>
  {:else}
    <div class="absolute bottom-0 left-0 right-0 flex items-center justify-between p-4 pointer-events-none">
      <span
        class="rounded-fc-pill bg-fc-surface/85 backdrop-blur px-3 py-1
          text-fc-xs font-mono text-fc-fg-muted"
      >
        Drag to orbit · Scroll to zoom
      </span>
      <span
        class="hidden sm:inline-flex rounded-fc-pill bg-fc-surface/85
          backdrop-blur px-3 py-1 text-fc-xs font-mono text-fc-success"
      >
        Fuse intact · 0 DRC violations
      </span>
    </div>
  {/if}
</div>
