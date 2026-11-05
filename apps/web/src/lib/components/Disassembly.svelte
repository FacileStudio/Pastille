<script lang="ts">
  import { onMount } from "svelte";
  import { gsap } from "gsap";
  import { Card, Icon, Spinner, prefersReducedMotion } from "@facile/muse";
  import { STEPS } from "$lib/explode";
  import { whenNear, watchSteps } from "$lib/explode-motion";
  import { mountExplodeScene, type ExplodeControl } from "$lib/explode-scene";

  const LAST = STEPS.length - 1;
  const pad = (n: number) => String(n).padStart(2, "0");

  let root: HTMLDivElement;
  let list: HTMLOListElement;
  let stage: HTMLDivElement;
  let host: HTMLDivElement;
  let status = $state<"idle" | "loading" | "ready" | "error">("idle");
  let active = $state(0);
  let reduced = $state(false);
  let viewer: ExplodeControl | null = null;
  let progress = 0;
  const label = $derived(`Pastille disassembly, step ${active + 1} of ${STEPS.length}: ${STEPS[active].title}`);

  /**
   * Stores the scroll progress and forwards it to the scene, which renders on the next frame.
   */
  function onProgress(t: number) {
    progress = t;
    viewer?.setProgress(t);
  }

  /**
   * Fades the stage out, jumps the scene to a step's static exploded frame, and fades it back in.
   */
  function crossfade(index: number) {
    if (!viewer) return;
    gsap.to(stage, {
      opacity: 0,
      duration: 0.18,
      ease: "power2.in",
      overwrite: true,
      onComplete: () => {
        viewer?.setProgress(index / LAST);
        gsap.to(stage, { opacity: 1, duration: 0.24, ease: "power2.out" });
      },
    });
  }

  /**
   * Sinks upcoming steps into a pile under the active card, so they rise into place when reached.
   */
  function pile(index: number): string {
    if (reduced || index <= active) return "none";
    const depth = index - active;
    return `translateY(${depth * 14}px) scale(${1 - depth * 0.02})`;
  }

  /**
   * Highlights the step whose block crosses the viewport centre.
   */
  function onActive(index: number) {
    if (index === active) return;
    active = index;
    if (reduced) crossfade(index);
  }

  /**
   * Loads three.js and the assembly model, then shows the frame for the current scroll position.
   */
  async function boot(isDisposed: () => boolean) {
    status = "loading";
    try {
      const ctrl = await mountExplodeScene(host);
      if (isDisposed()) {
        ctrl.dispose();
        return;
      }
      viewer = ctrl;
      ctrl.setProgress(reduced ? active / LAST : progress);
      status = "ready";
    } catch {
      if (!isDisposed()) status = "error";
    }
  }

  onMount(() => {
    reduced = prefersReducedMotion();
    let disposed = false;
    let stopSteps: (() => void) | null = null;
    const stopNear = whenNear(root, () => boot(() => disposed));
    watchSteps(list, { count: STEPS.length, scrub: !reduced, onProgress, onActive })
      .then((stop) => {
        if (disposed) stop();
        else stopSteps = stop;
      })
      .catch(() => {});
    return () => {
      disposed = true;
      stopNear();
      stopSteps?.();
      gsap.killTweensOf(stage);
      viewer?.dispose();
      viewer = null;
    };
  });
</script>

<div bind:this={root} class="flex flex-col gap-6 lg:grid lg:grid-cols-[11fr_9fr] lg:gap-10">
  <div
    class="sticky top-0 z-20 h-[calc(50dvh+5rem)] bg-fc-page pt-20 lg:top-20 lg:z-10
      lg:h-[calc(100dvh-7rem)] lg:self-start lg:bg-transparent lg:pt-0"
  >
    <div bind:this={stage} class="relative size-full overflow-hidden rounded-fc-lg bg-fc-component">
      <div bind:this={host} role="img" aria-label={label} class="absolute inset-0"></div>

      {#if status === "idle" || status === "loading"}
        <div class="absolute inset-0 flex flex-col items-center justify-center gap-3 pointer-events-none">
          <Spinner />
          <span class="text-fc-xs font-mono uppercase text-fc-fg-muted">Loading 3D assembly</span>
        </div>
      {:else if status === "error"}
        <div class="absolute inset-0 flex flex-col items-center justify-center gap-2 p-6 text-center">
          <span class="text-fc-sm font-semibold text-fc-fg">3D preview unavailable</span>
          <span class="text-fc-xs text-fc-fg-muted">The steps below still describe every part.</span>
        </div>
      {/if}

      <span
        class="absolute bottom-4 left-4 rounded-fc-pill bg-fc-surface/85 px-3 py-1 font-mono text-fc-xs
          text-fc-fg-muted backdrop-blur"
        aria-hidden="true"
      >
        {pad(active + 1)} / {pad(STEPS.length)} · {STEPS[active].title}
      </span>
    </div>
  </div>

  <ol
    bind:this={list}
    class="relative flex flex-col [--step-offset:8px] [--sticky-top:calc(50dvh+5.5rem)] lg:[--step-offset:14px]
      lg:[--sticky-top:5.5rem]"
    aria-label="Disassembly steps"
  >
    {#each STEPS as step, i (step.id)}
      {@const current = i === active}
      <li
        aria-current={current ? "step" : undefined}
        class="sticky transition-all duration-300 ease-out"
        style:top="calc(var(--sticky-top) + {i} * var(--step-offset))"
        style:z-index={i + 1}
        style:transform={pile(i)}
        style:margin-bottom={i === LAST ? "0" : "36vh"}
      >
        <Card
          class="flex w-full flex-col gap-3 p-6 !bg-fc-component transition-all duration-300
            {current
            ? 'shadow-[var(--step-card-shadow-active)] scale-[1.01]'
            : 'shadow-[var(--step-card-shadow)]'}"
        >
          <div class="flex items-center gap-3">
            <span
              class="rounded-fc-pill px-3 py-1 font-mono text-fc-xs transition-colors duration-300
                {current ? 'bg-fc-accent text-fc-accent-fg font-semibold' : 'bg-fc-component text-fc-fg-muted'}"
            >
              {pad(i + 1)}
            </span>
            {#each step.icons as icon (icon)}
              <span class="inline-flex size-10 items-center justify-center rounded-fc-md bg-fc-component text-fc-fg">
                <Icon {icon} size={22} />
              </span>
            {/each}
          </div>
          <h3 class="text-fc-md font-semibold text-fc-fg">{step.title}</h3>
          <p class="text-fc-sm text-fc-fg-muted leading-relaxed">
            {step.description}{#if step.motionNote && !reduced}{" "}{step.motionNote}{/if}
          </p>
        </Card>
      </li>
    {/each}
  </ol>
</div>

<style>
  :global(html) {
    --step-card-shadow: 0 -1px 2px -1px rgb(0 0 0 / 0.05), 0 -4px 10px -7px rgb(0 0 0 / 0.1);
    --step-card-shadow-active: 0 -1px 2px -1px rgb(0 0 0 / 0.07), 0 -7px 18px -9px rgb(0 0 0 / 0.16);
  }

  :global(html.dark) {
    --step-card-shadow: 0 -1px 2px -1px rgb(0 0 0 / 0.35), 0 -6px 14px -9px rgb(0 0 0 / 0.4);
    --step-card-shadow-active: 0 -1px 2px -1px rgb(0 0 0 / 0.45), 0 -9px 22px -10px rgb(0 0 0 / 0.5);
  }
</style>
