<script lang="ts">
  import type { TopologyLink, TopologyMode } from "$lib/topology";

  let { mode }: { mode: TopologyMode } = $props();

  const isOn = (link: TopologyLink) => mode.links.includes(link);
</script>

<svg
  viewBox="0 0 880 320"
  role="img"
  aria-labelledby="topology-title"
  class="topology h-auto w-full min-w-[640px]"
  font-family="var(--font-fc-mono)"
>
  <title id="topology-title">{mode.title}: {mode.summary}</title>

  <path d="M180 248 Q300 210 360 150" class="link idle" />
  <path d="M154 104 L360 104" class="link" class:flow={isOn("relay")} class:idle={!isOn("relay")} />
  <path d="M520 104 L700 104" class="link" class:flow={isOn("relay")} class:idle={!isOn("relay")} />
  <path d="M148 140 Q440 330 760 160" class="link" class:flow={isOn("direct")} class:idle={!isOn("direct")} />
  {#if isOn("provision")}
    <path d="M700 128 L520 128" class="link flow token" />
    <path d="M360 128 L152 128" class="link flow token" />
    <rect x="374" y="176" width="132" height="26" rx="13" fill="var(--color-fc-surface)" />
    <text x="440" y="193" text-anchor="middle" font-size="12" fill="var(--color-fc-warning)">bohs_tok_* · B004</text>
  {/if}

  {#if isOn("relay")}
    <text x="257" y="94" text-anchor="middle" font-size="11" fill="var(--color-fc-fg-muted)">PCM 0x01 · MCP 0x02</text>
    <text x="610" y="94" text-anchor="middle" font-size="11" fill="var(--color-fc-fg-muted)">WSS tunnel</text>
  {/if}
  {#if isOn("direct")}
    <text x="440" y="226" text-anchor="middle" font-size="11" fill="var(--color-fc-fg-muted)">
      bohs.v1 TLS WebSocket · no phone
    </text>
  {/if}

  <circle cx="110" cy="110" r="54" class="halo" fill="none" stroke="var(--color-fc-accent)" />
  <circle cx="110" cy="110" r="44" fill="var(--color-fc-surface)" stroke="var(--color-fc-accent)" stroke-width="2" />
  <text x="110" y="106" text-anchor="middle" font-size="14" font-weight="600" fill="var(--color-fc-fg)">Pastille</text>
  <text x="110" y="124" text-anchor="middle" font-size="10" fill="var(--color-fc-fg-muted)">38 mm · 18 uA</text>

  <rect x="40" y="222" width="140" height="52" rx="12" fill="var(--color-fc-surface)" />
  <text x="110" y="244" text-anchor="middle" font-size="12" fill="var(--color-fc-fg)">Other BOHS-01</text>
  <text x="110" y="262" text-anchor="middle" font-size="12" fill="var(--color-fc-fg)">devices</text>

  <rect x="360" y="52" width="160" height="104" rx="14" fill="var(--color-fc-surface)" />
  <text x="440" y="80" text-anchor="middle" font-size="14" font-weight="600" fill="var(--color-fc-fg)">Bulle app</text>
  <text x="440" y="98" text-anchor="middle" font-size="10" fill="var(--color-fc-fg-muted)">mobile & desktop</text>
  <text x="440" y="122" text-anchor="middle" font-size="10" fill="var(--color-fc-info)">B002 audio · B003 MCP</text>
  <text x="440" y="140" text-anchor="middle" font-size="10" fill="var(--color-fc-info)">B004 provisioning</text>

  <rect x="700" y="52" width="160" height="104" rx="14" fill="var(--color-fc-surface)" />
  <text x="780" y="80" text-anchor="middle" font-size="14" font-weight="600" fill="var(--color-fc-fg)">
    Bulle Gateway
  </text>
  <text x="780" y="98" text-anchor="middle" font-size="10" fill="var(--color-fc-fg-muted)">+ Facile Registre</text>
  <text x="780" y="122" text-anchor="middle" font-size="10" fill="var(--color-fc-fg-muted)">STT · LLM · TTS</text>
  <text x="780" y="140" text-anchor="middle" font-size="10" fill="var(--color-fc-fg-muted)">self-hosted</text>
</svg>

<style>
  .link {
    fill: none;
    stroke-width: 2.5;
    stroke-linecap: round;
    transition: opacity 200ms ease;
  }

  .idle {
    stroke: var(--color-fc-fg-muted);
    stroke-dasharray: 2 8;
    opacity: 0.35;
  }

  .flow {
    stroke: var(--color-fc-accent);
    stroke-dasharray: 8 10;
    animation: packet 1.1s linear infinite;
  }

  .token {
    stroke: var(--color-fc-warning);
  }

  .halo {
    stroke-width: 1.5;
    opacity: 0.4;
    transform-origin: 110px 110px;
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
      transform: scale(1.15);
      opacity: 0;
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .link,
    .flow,
    .halo {
      animation: none;
      transition: none;
    }
  }
</style>
