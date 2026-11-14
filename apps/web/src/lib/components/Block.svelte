<script lang="ts">
  import type { Snippet } from "svelte";
  import { LANDING_HEADING } from "@facile/muse";

  let {
    id,
    kicker,
    title,
    description,
    children,
  }: { id: string; kicker?: string; title: string; description?: string; children: Snippet } = $props();
</script>

<section {id} class="flex scroll-mt-20 flex-col gap-8">
  <div class="flex max-w-2xl flex-col gap-2">
    {#if kicker}<p class="text-fc-xs font-mono uppercase text-fc-fg-muted">{kicker}</p>{/if}
    <h2 class={LANDING_HEADING}>{title}</h2>
    {#if description}<p class="text-fc-md text-fc-fg-muted">{description}</p>{/if}
  </div>
  {@render children()}
</section>
