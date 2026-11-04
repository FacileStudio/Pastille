<script lang="ts">
  import SvgIcon from "$lib/components/SvgIcon.svelte";
  import {
    LAYOUTS,
    TOPOLOGY_DEVICES,
    TOPOLOGY_HUB,
    TOPOLOGY_SERVER,
    type TopologyMode,
    type TopologyNode,
  } from "$lib/topology";

  let { mode, vertical = false }: { mode: TopologyMode; vertical?: boolean } = $props();

  const L = $derived(vertical ? LAYOUTS.vertical : LAYOUTS.horizontal);
  const isActive = (node: TopologyNode) => mode.devices.includes(node.id);
  const relayOn = (node: TopologyNode) => !mode.direct && isActive(node);
  const directOn = (node: TopologyNode) => mode.direct && isActive(node);
  const pairing = $derived(mode.id === "pair");
  const linkIcon = $derived(pairing ? "solar:key-bold-duotone" : "solar:bluetooth-wave-bold-duotone");
</script>

<svg
  viewBox={L.viewBox}
  role="img"
  aria-labelledby="topology-title"
  class="topology mx-auto h-auto w-full"
  class:max-w-sm={vertical}
>
  <title id="topology-title">{mode.button}: {mode.summary}</title>

  {#each TOPOLOGY_DEVICES as node, i (node.id)}
    <path
      d="M{L.devices[i].x} {L.devices[i].y} L{L.hub.x} {L.hub.y}"
      class="link"
      class:flow={relayOn(node)}
      class:token={pairing && relayOn(node)}
      class:idle={!relayOn(node)}
    />
    <path
      d="M{L.devices[i].x} {L.devices[i].y} Q{L.direct.x} {L.direct.y} {L.server.x} {L.server.y}"
      class="link"
      class:flow={directOn(node)}
      class:hidden={!directOn(node)}
    />
  {/each}
  <path
    d="M{L.hub.x} {L.hub.y} Q{L.uplink.x} {L.uplink.y} {L.server.x} {L.server.y}"
    class="link"
    class:flow={!mode.direct}
    class:token={pairing}
    class:idle={mode.direct}
  />

  {#each TOPOLOGY_DEVICES as node, i (node.id)}
    {#if relayOn(node)}
      {@const mid = { x: (L.devices[i].x + L.hub.x) / 2, y: (L.devices[i].y + L.hub.y) / 2 }}
      <circle cx={mid.x} cy={mid.y} r="17" class="badge" />
      <SvgIcon
        name={linkIcon}
        x={mid.x}
        y={mid.y}
        size={20}
        color={pairing ? "var(--color-fc-warning)" : "var(--color-fc-info)"}
      />
    {/if}
  {/each}
  {#if mode.direct}
    <circle cx={L.wifi.x} cy={L.wifi.y} r="19" class="badge" />
    <SvgIcon name="solar:wi-fi-router-bold-duotone" x={L.wifi.x} y={L.wifi.y} size={22} color="var(--color-fc-info)" />
  {/if}

  {#each TOPOLOGY_DEVICES as node, i (node.id)}
    {@const at = L.devices[i]}
    <g class="node" class:dim={!isActive(node)}>
      {#if isActive(node)}
        <circle cx={at.x} cy={at.y} r="40" class="halo" />
      {/if}
      <circle cx={at.x} cy={at.y} r="34" class="disc" class:primary={node.id === "pastille"} />
      <SvgIcon name={node.icon} x={at.x} y={at.y} size={34} color="var(--color-fc-accent)" />
      <text x={at.x} y={at.y + L.labelDy} text-anchor="middle" class="label">{node.label}</text>
    </g>
  {/each}

  <g class="node" class:dim={mode.direct}>
    <circle cx={L.hub.x} cy={L.hub.y} r="44" class="disc" />
    <SvgIcon name={TOPOLOGY_HUB.icon} x={L.hub.x} y={L.hub.y} size={40} color="var(--color-fc-accent)" />
    <text x={L.hub.x} y={L.hub.y + 64} text-anchor="middle" class="label">{TOPOLOGY_HUB.label}</text>
  </g>

  <g class="node">
    <circle cx={L.server.x} cy={L.server.y} r="48" class="disc" />
    <SvgIcon name={TOPOLOGY_SERVER.icon} x={L.server.x} y={L.server.y} size={44} color="var(--color-fc-accent)" />
    <text x={L.server.x} y={L.server.y + 70} text-anchor="middle" class="label">{TOPOLOGY_SERVER.label}</text>
  </g>
</svg>

<style>
  .link {
    fill: none;
    stroke-width: 2.5;
    stroke-linecap: round;
    transition: opacity 300ms ease;
  }

  .idle {
    stroke: var(--color-fc-fg-muted);
    stroke-dasharray: 2 8;
    opacity: 0.3;
  }

  .hidden {
    opacity: 0;
  }

  .flow {
    stroke: var(--color-fc-accent);
    stroke-dasharray: 8 10;
    animation: packet 1.1s linear infinite;
  }

  .token {
    stroke: var(--color-fc-warning);
    animation-direction: reverse;
  }

  .badge {
    fill: var(--color-fc-surface);
  }

  .disc {
    fill: var(--color-fc-surface);
  }

  .disc.primary {
    stroke: var(--color-fc-accent);
    stroke-width: 2;
  }

  .label {
    fill: var(--color-fc-fg);
    font-size: 15px;
    font-weight: 600;
  }

  .node {
    transition: opacity 300ms ease;
  }

  .dim {
    opacity: 0.35;
  }

  .halo {
    fill: none;
    stroke: var(--color-fc-accent);
    stroke-width: 1.5;
    transform-box: fill-box;
    transform-origin: center;
    animation: halo 2.4s ease-out infinite;
  }

  @keyframes packet {
    to {
      stroke-dashoffset: -36;
    }
  }

  @keyframes halo {
    0% {
      transform: scale(0.9);
      opacity: 0.5;
    }
    100% {
      transform: scale(1.2);
      opacity: 0;
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .link,
    .flow,
    .halo,
    .node {
      animation: none;
      transition: none;
    }
  }
</style>
