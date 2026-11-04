<script lang="ts">
  import { Button, Card, Icon, prefersReducedMotion, registerIcons } from "@facile/muse";
  import TopologyDiagram from "$lib/components/TopologyDiagram.svelte";
  import { TOPOLOGY_ICONS } from "$lib/topology-icons";
  import { EXAMPLE_NOTE, TOPOLOGY_MODES, type TopologyModeId } from "$lib/topology";

  const CYCLE_MS = 5000;
  const TOUCH = "pointer-coarse:h-11";

  registerIcons(TOPOLOGY_ICONS);

  let activeId = $state<TopologyModeId>("ble");
  let pinned = $state(false);
  let root: HTMLDivElement | null = $state(null);
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

  /**
   * Cycles the modes while the card is on screen, and holds still while it is hovered or focused.
   */
  function autoCycle(node: HTMLElement) {
    const hold = { hover: false, focus: false, seen: false };
    const timer = setInterval(() => {
      if (hold.seen && !hold.hover && !hold.focus) advance();
    }, CYCLE_MS);
    const observer = new IntersectionObserver(([entry]) => (hold.seen = entry?.isIntersecting ?? false));
    observer.observe(node);
    const events: [string, () => void][] = [
      ["pointerenter", () => (hold.hover = true)],
      ["pointerleave", () => (hold.hover = false)],
      ["focusin", () => (hold.focus = true)],
      ["focusout", () => (hold.focus = false)],
    ];
    for (const [type, fn] of events) node.addEventListener(type, fn);
    return () => {
      clearInterval(timer);
      observer.disconnect();
      for (const [type, fn] of events) node.removeEventListener(type, fn);
    };
  }

  $effect(() => {
    if (pinned || !root || prefersReducedMotion()) return;
    return autoCycle(root);
  });
</script>

<div bind:this={root}>
  <Card class="flex flex-col items-center gap-5 p-4 sm:p-6">
    <div role="group" aria-label="Connection mode" class="flex flex-wrap gap-1 rounded-fc-pill bg-fc-surface p-1">
      {#each TOPOLOGY_MODES as option (option.id)}
        <Button
          size="sm"
          icon={option.icon}
          variant={activeId === option.id ? "primary" : "ghost"}
          class={activeId === option.id ? TOUCH : `${TOUCH} text-fc-fg hover:bg-fc-component`}
          aria-pressed={activeId === option.id}
          onclick={() => choose(option.id)}
        >
          {option.button}
        </Button>
      {/each}
    </div>

    <p class="text-center text-fc-sm text-fc-fg-muted" aria-live={pinned ? "polite" : "off"}>{active.summary}</p>

    <div class="w-full rounded-fc-md bg-fc-component">
      <div class="sm:hidden"><TopologyDiagram mode={active} id="topology-vertical" vertical /></div>
      <div class="hidden sm:block"><TopologyDiagram mode={active} id="topology-horizontal" /></div>
    </div>

    <ul class="flex flex-wrap justify-center gap-2">
      {#each active.pills as pill (pill.value)}
        <li class="flex items-center gap-2 rounded-fc-pill bg-fc-surface px-3 py-1.5 text-fc-sm text-fc-fg">
          <Icon icon={pill.icon} size={18} class="text-fc-accent" />
          {pill.value}
        </li>
      {/each}
    </ul>

    <p class="text-center text-fc-xs text-fc-fg-muted">{EXAMPLE_NOTE}</p>
  </Card>
</div>
