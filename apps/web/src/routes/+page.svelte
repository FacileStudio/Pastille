<script lang="ts">
  import { Button, Card, Icon, Section, icons } from "@facile/muse";
  import Disassembly from "$lib/components/Disassembly.svelte";
  import PcbViewer from "$lib/components/PcbViewer.svelte";
  import SyncTopology from "$lib/components/SyncTopology.svelte";
  import {
    COMPARISON,
    FEATURES,
    HERO_METRICS,
    PROTOCOL_PLANES,
    QUICKSTART_STEPS,
    type TokenTone,
  } from "$lib/data";
  import { GITHUB_ICON } from "$lib/icons";
  import { scrollReveal } from "$lib/motion";

  const TONE_CLASS: Record<TokenTone, string> = {
    fg: "text-fc-fg",
    muted: "text-fc-fg-muted",
    key: "text-fc-info",
    str: "text-fc-success",
    num: "text-fc-warning",
    kw: "text-fc-admin",
  };
</script>

<svelte:head>
  <title>Pastille | Wear your assistant</title>
  <meta
    name="description"
    content="Pastille is an open hardware charm you wear. Tap it, talk, and your own Bulle assistant answers."
  />
</svelte:head>

<section use:scrollReveal={{ delay: 0.12, stagger: 0.07 }} class="flex flex-col gap-6 pt-4 md:pt-8">
  <div data-reveal-item class="flex flex-col gap-3 max-w-2xl">
    <h1 class="text-fc-2xl sm:text-fc-3xl font-semibold tracking-tight text-fc-fg">
      Wear your assistant.
    </h1>
    <p class="text-fc-md sm:text-fc-lg text-fc-fg-muted">
      Pastille is a 42 mm charm with Bulle's face on a round screen. Tap it and talk. Bulle answers out
      loud from a server you run, and every part of Pastille is open source.
    </p>
  </div>

  <div data-reveal-item class="flex flex-wrap items-center gap-3 pt-1">
    <Button
      variant="primary"
      size="lg"
      href="https://github.com/FacileStudio/Pastille"
      target="_blank"
      rel="noreferrer"
      icon={GITHUB_ICON}
    >
      View on GitHub
    </Button>
    <Button
      variant="ghost"
      size="lg"
      class="bg-fc-surface text-fc-fg hover:bg-fc-component"
      href="https://github.com/FacileStudio/Pastille/releases/tag/v0.4.0"
      target="_blank"
      rel="noreferrer"
      icon={icons.download}
    >
      Build files (v0.4.0)
    </Button>
  </div>

  <div class="grid grid-cols-1 gap-4 pt-3 sm:grid-cols-2 lg:grid-cols-4">
    {#each HERO_METRICS as metric (metric.label)}
      <div data-reveal-item>
        <Card class="flex h-full flex-col gap-1 p-5">
          <span class="text-fc-xs font-mono uppercase text-fc-fg-muted">{metric.label}</span>
          <span class="text-fc-2xl font-semibold text-fc-fg">{metric.value}</span>
          <span class="text-fc-xs text-fc-fg-muted">{metric.detail}</span>
        </Card>
      </div>
    {/each}
  </div>
</section>

<Section
  id="disassembly"
  title="What's inside"
  description="Scroll to take Pastille apart, one layer at a time."
>
  <Disassembly />
</Section>

<div use:scrollReveal={{ y: 20 }}>
  <Section
    id="viewer"
    title="The board in 3D"
    description="Drag to turn it. Fusible builds this model from the circuit source, so it matches the real board."
  >
    <PcbViewer />
  </Section>
</div>

<div use:scrollReveal={{ y: 20 }}>
  <Section
    id="connectivity"
    title="One app, many devices"
    description="Bluetooth on the go, Wi-Fi at home, one tap to pair."
  >
    <SyncTopology />
  </Section>
</div>

<div use:scrollReveal={{ stagger: 0.05 }}>
  <Section
    id="architecture"
    title="What it's made of"
    description="Six blocks on one 38 mm, 4-layer board."
  >
    <div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
      {#each FEATURES as item (item.title)}
        <div data-reveal-item>
          <Card class="flex h-full flex-col justify-between gap-4 p-6">
            <div class="flex flex-col gap-2.5">
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
  </Section>
</div>

<div use:scrollReveal={{ stagger: 0.07 }}>
  <Section
    id="protocol"
    title="How it talks"
    description="Voice, tool calls and pairing share one bohs.v1 stream, over Bluetooth or Wi-Fi."
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
            <div
              class="flex flex-col gap-0.5 rounded-fc-md bg-fc-surface p-4 font-mono text-fc-xs
                leading-relaxed overflow-x-auto"
            >
              {#each plane.lines as line, idx (idx)}
                <div class="flex">
                  {#each line as token, tIdx (tIdx)}
                    <span class="whitespace-pre {TONE_CLASS[token.tone ?? 'fg']}">{token.text}</span>
                  {/each}
                </div>
              {/each}
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
    title="How it compares"
    description="Pastille next to two AI gadgets you can buy today."
  >
    <div class="grid grid-cols-1 gap-2.5">
      <div class="hidden md:grid md:grid-cols-4 gap-4 px-5 py-2 text-fc-xs font-mono uppercase text-fc-fg-muted">
        <span>Metric</span>
        <span>Pastille (BOHS-01)</span>
        <span>OMI (Friend)</span>
        <span>Rabbit R1</span>
      </div>
      {#each COMPARISON as row (row.metric)}
        <div data-reveal-item>
          <Card class="grid grid-cols-1 gap-2 p-4 sm:p-5 md:grid-cols-4 md:items-center md:gap-4">
            <span class="text-fc-sm font-semibold text-fc-fg">{row.metric}</span>
            <span class="rounded-fc-sm bg-fc-surface px-3 py-1 text-fc-sm font-medium text-fc-fg w-fit">
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

<div use:scrollReveal={{ stagger: 0.07 }} class="pb-12">
  <Section
    id="quickstart"
    title="Build your own"
    description="Check the circuit, export the case and board files, then test and flash the firmware."
  >
    <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
      {#each QUICKSTART_STEPS as step (step.badge)}
        <div data-reveal-item>
          <Card class="flex h-full flex-col justify-between gap-4 p-6">
            <div class="flex flex-col gap-2">
              <span class="w-fit rounded-fc-pill bg-fc-surface px-3 py-1 text-fc-xs font-mono text-fc-fg-muted">
                {step.badge}
              </span>
              <h3 class="text-fc-md font-semibold text-fc-fg">{step.title}</h3>
              <p class="text-fc-sm text-fc-fg-muted">{step.summary}</p>
            </div>
            <div
              class="flex flex-col gap-1 rounded-fc-md bg-fc-surface p-4 font-mono text-fc-xs
                leading-relaxed overflow-x-auto"
            >
              {#each step.lines as line, idx (idx)}
                <div class="flex">
                  {#each line as token, tIdx (tIdx)}
                    <span class="whitespace-pre {TONE_CLASS[token.tone ?? 'fg']}">{token.text}</span>
                  {/each}
                </div>
              {/each}
            </div>
          </Card>
        </div>
      {/each}
    </div>
  </Section>
</div>
