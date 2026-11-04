<script lang="ts">
  import { onMount } from "svelte";
  import { Icon } from "@facile/muse";
  import { PASTILLE_LOGO } from "$lib/icons";
  import { runIntroSequence } from "$lib/motion";

  let visible = $state(true);
  let overlay: HTMLDivElement | null = $state(null);
  let ringOuter: HTMLDivElement | null = $state(null);
  let ringInner: HTMLDivElement | null = $state(null);
  let mark: HTMLDivElement | null = $state(null);
  let bar: HTMLDivElement | null = $state(null);

  onMount(() => {
    if (!overlay || !ringOuter || !ringInner || !mark || !bar) {
      visible = false;
      return;
    }
    return runIntroSequence(
      { overlay, ringOuter, ringInner, mark, bar },
      () => {
        visible = false;
      },
    );
  });
</script>

{#if visible}
  <div
    bind:this={overlay}
    class="fixed inset-0 z-[100] flex flex-col items-center justify-center bg-fc-page pointer-events-none select-none"
    aria-hidden="true"
  >
    <div class="relative flex h-36 w-36 items-center justify-center">
      <div
        bind:this={ringOuter}
        class="absolute h-28 w-28 rounded-fc-full bg-fc-surface opacity-0"
      ></div>
      <div
        bind:this={ringInner}
        class="absolute h-20 w-20 rounded-fc-full bg-fc-component opacity-0"
      ></div>
      <div
        bind:this={mark}
        class="relative flex h-14 w-14 items-center justify-center rounded-fc-full bg-fc-surface text-fc-fg opacity-0"
      >
        <Icon icon={PASTILLE_LOGO} size={32} />
      </div>
    </div>

    <div class="mt-4 h-1 w-28 overflow-hidden rounded-fc-pill bg-fc-surface">
      <div
        bind:this={bar}
        class="h-full w-full origin-left scale-x-0 rounded-fc-pill bg-fc-fg"
      ></div>
    </div>
  </div>
{/if}
