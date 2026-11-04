<script lang="ts">
  import { Button, Card, Icon, Section, icons } from "@facile/muse";
  import PcbViewer from "$lib/components/PcbViewer.svelte";
  import {
    COMPARISON,
    FEATURES,
    HERO_METRICS,
    PROTOCOL_PLANES,
    QUICKSTART_COMMANDS,
    SUBSYSTEMS,
  } from "$lib/data";
  import { PASTILLE_LOGO } from "$lib/icons";
  import { scrollReveal } from "$lib/motion";
</script>

<svelte:head>
  <title>Pastille — Open-source wearable AI charm (BOHS-01)</title>
  <meta
    name="description"
    content="Pastille is an open-source 38 mm wearable AI charm and the reference implementation of BOHS-01."
  />
</svelte:head>

<section use:scrollReveal={{ delay: 0.15, stagger: 0.08 }} class="flex flex-col gap-6 pt-4 md:pt-8">
  <div
    data-reveal-item
    class="inline-flex w-fit items-center gap-2 rounded-fc-pill bg-fc-surface
      px-3.5 py-1.5 text-fc-xs font-medium text-fc-fg-muted"
  >
    <Icon icon={PASTILLE_LOGO} size={16} class="text-fc-fg" />
    <span>Bulle Open Hardware Standard · Reference Implementation</span>
  </div>

  <div data-reveal-item class="flex flex-col gap-4 max-w-3xl">
    <h1 class="text-fc-2xl sm:text-fc-3xl font-semibold tracking-tight text-fc-fg">
      A 38 mm wearable voice and expression charm for your private Bulle gateway.
    </h1>
    <p class="text-fc-md sm:text-fc-lg text-fc-fg-muted">
      Pastille clips to clothing or hangs as a pendant. It wakes from deep sleep in under
      50 milliseconds, streams bidirectional Opus audio over WebSocket, renders 60 FPS companion
      expressions on a round IPS display, and runs for over two weeks on a single charge.
    </p>
  </div>

  <div data-reveal-item class="flex flex-wrap items-center gap-3 pt-1">
    <Button
      variant="primary"
      size="lg"
      href="https://github.com/FacileStudio/Pastille"
      target="_blank"
      rel="noreferrer"
      icon={icons.code}
    >
      Explore the repository
    </Button>
    <Button
      variant="ghost"
      size="lg"
      class="bg-fc-surface text-fc-fg hover:bg-fc-component"
      href="https://github.com/FacileStudio/Pastille/releases/tag/v0.2.0"
      target="_blank"
      rel="noreferrer"
      icon={icons.download}
    >
      Download Gerbers & STEP (v0.2.0)
    </Button>
  </div>

  <div class="grid grid-cols-1 gap-4 pt-4 sm:grid-cols-2 lg:grid-cols-4">
    {#each HERO_METRICS as metric (metric.label)}
      <div data-reveal-item>
        <Card class="flex h-full flex-col gap-1.5 p-5">
          <span class="text-fc-xs font-mono uppercase text-fc-fg-muted">{metric.label}</span>
          <span class="text-fc-2xl font-semibold text-fc-fg">{metric.value}</span>
          <span class="text-fc-xs text-fc-fg-muted">{metric.detail}</span>
        </Card>
      </div>
    {/each}
  </div>
</section>

<div use:scrollReveal={{ y: 24 }}>
  <Section
    id="viewer"
    title="Interactive 3D PCB & enclosure"
    description="Baked directly from hardware/index.tsx via Fusible after passing electrical and DRC checks."
  >
    <PcbViewer />
  </Section>
</div>

<div use:scrollReveal={{ stagger: 0.06 }}>
  <Section
    id="architecture"
    title="Engineered for skin-contact wearables"
    description="Dedicated wearable peripherals outperform miniature smartphones on standby power, thermals, and BOM."
  >
    <div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
      {#each FEATURES as item (item.title)}
        <div data-reveal-item>
          <Card class="flex h-full flex-col justify-between gap-4 p-6">
            <div class="flex flex-col gap-3">
              <div class="inline-flex h-10 w-10 items-center justify-center rounded-fc-md bg-fc-surface text-fc-fg">
                <Icon icon={item.icon} size={22} />
              </div>
              <h3 class="text-fc-md font-semibold text-fc-fg">{item.title}</h3>
              <p class="text-fc-sm text-fc-fg-muted">{item.description}</p>
            </div>
            <span class="w-fit rounded-fc-pill bg-fc-surface px-3 py-1 text-fc-xs font-mono text-fc-fg-muted">
              {item.tag}
            </span>
          </Card>
        </div>
      {/each}
    </div>

    <div class="grid grid-cols-1 gap-4 pt-2 lg:grid-cols-3">
      {#each SUBSYSTEMS as group (group.title)}
        <div data-reveal-item>
          <Card class="flex h-full flex-col gap-3 p-6">
            <div class="flex flex-col gap-0.5">
              <h3 class="text-fc-md font-semibold text-fc-fg">{group.title}</h3>
              <span class="text-fc-xs text-fc-fg-muted">{group.subtitle}</span>
            </div>
            <ul class="flex flex-col gap-2 pt-1 text-fc-sm text-fc-fg-muted">
              {#each group.items as line (line)}
                <li class="flex items-start gap-2.5">
                  <span class="mt-1.5 h-1.5 w-1.5 shrink-0 rounded-fc-full bg-fc-fg"></span>
                  <span>{line}</span>
                </li>
              {/each}
            </ul>
          </Card>
        </div>
      {/each}
    </div>
  </Section>
</div>

<div use:scrollReveal={{ stagger: 0.08 }}>
  <Section
    id="protocol"
    title="BOHS-01 multiplexed wire protocol"
    description="One WebSocket connection (bohs.v1) carries Opus voice frames and Reverse MCP JSON-RPC 2.0 tool calls."
  >
    <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
      {#each PROTOCOL_PLANES as plane (plane.opcode)}
        <div data-reveal-item>
          <Card class="flex h-full flex-col justify-between gap-4 p-6">
            <div class="flex flex-col gap-2">
              <span class="w-fit rounded-fc-pill bg-fc-surface px-3 py-1 text-fc-xs font-mono text-fc-fg-muted">
                {plane.opcode}
              </span>
              <h3 class="text-fc-md font-semibold text-fc-fg">{plane.name}</h3>
              <p class="text-fc-sm text-fc-fg-muted">{plane.summary}</p>
            </div>
            <div class="rounded-fc-md bg-fc-surface p-3.5 font-mono text-fc-xs text-fc-fg overflow-x-auto">
              <code>{plane.payload}</code>
            </div>
          </Card>
        </div>
      {/each}
    </div>
  </Section>
</div>

<div use:scrollReveal={{ stagger: 0.05 }}>
  <Section
    id="comparison"
    title="Architectural comparison"
    description="How Pastille (BOHS-01) compares against commercial wearable AI pins and pendants."
  >
    <div class="grid grid-cols-1 gap-2.5">
      <div class="hidden md:grid md:grid-cols-4 gap-4 px-5 py-2 text-fc-xs font-mono uppercase text-fc-fg-muted">
        <span>Characteristic</span>
        <span>Pastille (BOHS-01)</span>
        <span>OMI (Friend)</span>
        <span>Rabbit R1</span>
      </div>
      {#each COMPARISON as row (row.metric)}
        <div data-reveal-item>
          <Card class="grid grid-cols-1 gap-2 p-5 md:grid-cols-4 md:items-center md:gap-4">
            <span class="text-fc-sm font-semibold text-fc-fg">{row.metric}</span>
            <span class="rounded-fc-sm bg-fc-surface px-3 py-1.5 text-fc-sm font-medium text-fc-fg w-fit">
              {row.pastille}
            </span>
            <span class="text-fc-sm text-fc-fg-muted">{row.omi}</span>
            <span class="text-fc-sm text-fc-fg-muted">{row.rabbit}</span>
          </Card>
        </div>
      {/each}
    </div>
  </Section>
</div>

<div use:scrollReveal={{ y: 18 }}>
  <Section
    id="quickstart"
    title="Build & verify from source"
    description="Fusible compiles the circuit, validates electrical rules, and exports Gerbers and STEP models."
  >
    <Card class="flex flex-col gap-2.5 p-6 font-mono text-fc-xs sm:text-fc-sm overflow-x-auto">
      {#each QUICKSTART_COMMANDS as cmd (cmd)}
        <div class="flex items-center gap-3 whitespace-nowrap">
          <span class="text-fc-fg-muted select-none">$</span>
          <span class="text-fc-fg">{cmd}</span>
        </div>
      {/each}
    </Card>
  </Section>
</div>
