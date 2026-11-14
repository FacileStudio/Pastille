<script lang="ts">
  import type { Snippet } from "svelte";
  import { LANDING_HEADING } from "@facile/muse";

  let {
    id,
    title,
    description,
    children,
  }: { id: string; title: string; description?: string; children: Snippet } = $props();
</script>

<section {id} class="flex scroll-mt-20 flex-col gap-8">
  <div class="flex max-w-2xl flex-col gap-2">
    <h2 class={LANDING_HEADING}>{title}</h2>
    {#if description}<p class="text-fc-md text-fc-fg-muted">{description}</p>{/if}
  </div>
  {@render children()}
</section>
