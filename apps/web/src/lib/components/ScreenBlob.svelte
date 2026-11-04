<script lang="ts">
  import { Blob, gaze } from "@facile/blob";

  let { still = false }: { still?: boolean } = $props();

  const eyes = gaze({ travel: 6, target: "pointer" });
</script>

<div class="flex size-full items-center justify-center overflow-hidden rounded-full bg-black" aria-hidden="true">
  {#if still}
    <Blob name="Bulle" class="block size-4/5" />
  {:else}
    <Blob name="Bulle" animate="always" class="block size-4/5" {@attach eyes} />
  {/if}
</div>
