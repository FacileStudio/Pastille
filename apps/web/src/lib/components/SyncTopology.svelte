<script lang="ts">
  import { Button, Card, prefersReducedMotion } from "@facile/muse";
  import TopologyDiagram from "$lib/components/TopologyDiagram.svelte";
  import { TOPOLOGY_MODES, type TopologyModeId } from "$lib/topology";

  const CYCLE_MS = 6000;

  let activeId = $state<TopologyModeId>("ble");
  let pinned = $state(false);
  const active = $derived(TOPOLOGY_MODES.find((m) => m.id === activeId) ?? TOPOLOGY_MODES[0]);

  /**
   * Selects a transport mode and stops the automatic cycle.
   */
  function choose(id: TopologyModeId) {
    pinned = true;
    activeId = id;
  }

  /**
   * Advances to the next transport mode in display order.
   */
  function advance() {
    const index = TOPOLOGY_MODES.findIndex((m) => m.id === activeId);
    activeId = TOPOLOGY_MODES[(index + 1) % TOPOLOGY_MODES.length].id;
  }

  $effect(() => {
    if (pinned || prefersReducedMotion()) return;
    const timer = setInterval(advance, CYCLE_MS);
    return () => clearInterval(timer);
  });
</script>

<Card class="flex flex-col gap-5 p-4 sm:p-6">
  <div role="group" aria-label="Transport mode" class="flex flex-wrap gap-1 rounded-fc-pill bg-fc-surface p-1 w-fit">
    {#each TOPOLOGY_MODES as option (option.id)}
      <Button
        size="sm"
        variant={activeId === option.id ? "primary" : "ghost"}
        class={activeId === option.id ? "" : "text-fc-fg hover:bg-fc-component"}
        aria-pressed={activeId === option.id}
        onclick={() => choose(option.id)}
      >
        {option.button}
      </Button>
    {/each}
  </div>

  <div class="flex flex-col gap-1.5" aria-live={pinned ? "polite" : "off"}>
    <h3 class="text-fc-md font-semibold text-fc-fg">{active.title}</h3>
    <p class="max-w-3xl text-fc-sm text-fc-fg-muted">{active.summary}</p>
  </div>

  <div class="overflow-x-auto rounded-fc-md bg-fc-component">
    <TopologyDiagram mode={active} />
  </div>

  <dl class="grid grid-cols-1 gap-2 sm:grid-cols-2 lg:grid-cols-4">
    {#each active.telemetry as pill (pill.label)}
      <div class="flex flex-col gap-0.5 rounded-fc-md bg-fc-surface px-4 py-3">
        <dt class="text-fc-xs font-mono uppercase text-fc-fg-muted">{pill.label}</dt>
        <dd class="text-fc-sm font-medium text-fc-fg">{pill.value}</dd>
      </div>
    {/each}
  </dl>
</Card>
