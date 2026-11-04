<script lang="ts">
  import { Button, Card, Icon, prefersReducedMotion, registerIcons } from "@facile/muse";
  import TopologyDiagram from "$lib/components/TopologyDiagram.svelte";
  import { TOPOLOGY_ICONS } from "$lib/topology-icons";
  import { TOPOLOGY_MODES, type TopologyModeId } from "$lib/topology";

  const CYCLE_MS = 5000;

  registerIcons(TOPOLOGY_ICONS);

  let activeId = $state<TopologyModeId>("ble");
  let pinned = $state(false);
  let vertical = $state(false);
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
    const query = window.matchMedia("(max-width: 639px)");
    const sync = () => (vertical = query.matches);
    sync();
    query.addEventListener("change", sync);
    return () => query.removeEventListener("change", sync);
  });

  $effect(() => {
    if (pinned || prefersReducedMotion()) return;
    const timer = setInterval(advance, CYCLE_MS);
    return () => clearInterval(timer);
  });
</script>

<Card class="flex flex-col items-center gap-5 p-4 sm:p-6">
  <div role="group" aria-label="Connection mode" class="flex flex-wrap gap-1 rounded-fc-pill bg-fc-surface p-1">
    {#each TOPOLOGY_MODES as option (option.id)}
      <Button
        size="sm"
        icon={option.icon}
        variant={activeId === option.id ? "primary" : "ghost"}
        class={activeId === option.id ? "" : "text-fc-fg hover:bg-fc-component"}
        aria-pressed={activeId === option.id}
        onclick={() => choose(option.id)}
      >
        {option.button}
      </Button>
    {/each}
  </div>

  <p class="text-center text-fc-sm text-fc-fg-muted" aria-live={pinned ? "polite" : "off"}>{active.summary}</p>

  <div class="w-full rounded-fc-md bg-fc-component">
    <TopologyDiagram mode={active} {vertical} />
  </div>

  <ul class="flex flex-wrap justify-center gap-2">
    {#each active.pills as pill (pill.value)}
      <li class="flex items-center gap-2 rounded-fc-pill bg-fc-surface px-3 py-1.5 text-fc-sm text-fc-fg">
        <Icon icon={pill.icon} size={18} class="text-fc-accent" />
        {pill.value}
      </li>
    {/each}
  </ul>
</Card>
