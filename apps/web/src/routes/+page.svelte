<script lang="ts">
  import { Button, Card, Icon, Section } from "@facile/muse";
  import Disassembly from "$lib/components/Disassembly.svelte";
  import HeroDemo from "$lib/components/demo/HeroDemo.svelte";
  import PcbViewer from "$lib/components/PcbViewer.svelte";
  import PreOrderForm from "$lib/components/PreOrderForm.svelte";
  import { FEATURES } from "$lib/data";
  import { GITHUB_ICON } from "$lib/icons";
  import { scrollReveal, scrollToSection } from "$lib/motion";
</script>


<section use:scrollReveal={{ delay: 0.12, stagger: 0.07 }} class="flex flex-col gap-6 pt-4 md:pt-8">
  <div data-reveal-item class="flex max-w-2xl flex-col gap-3">
    <h1 class="text-fc-2xl font-bold tracking-tight text-fc-fg sm:text-fc-3xl">Open hardware. Sovereign AI.</h1>
    <p class="text-fc-md text-fc-fg-muted sm:text-fc-lg">
      A 42 mm charm you tap to talk to. Bulle answers from a server you run.
    </p>
  </div>

  <div data-reveal-item class="flex flex-wrap items-center gap-3">
    <Button variant="primary" size="lg" href="#preorder" icon="solar:cart-large-2-bold-duotone"
      onclick={(e) => scrollToSection("preorder", e)}>Pre-order</Button>
    <Button variant="secondary" size="lg" href="https://github.com/FacileStudio/Pastille" target="_blank"
      rel="noreferrer" icon={GITHUB_ICON}>View on GitHub</Button>
  </div>

  <div data-reveal-item class="pt-2">
    <HeroDemo />
  </div>
</section>

<Section id="disassembly" title="What's inside" description="Scroll to take Pastille apart, one layer at a time.">
  <Disassembly />
</Section>

<div use:scrollReveal={{ stagger: 0.05 }}>
  <Section id="features" title="Made to wear" description="A 42 mm disc, 14 mm thick, built around a 38 mm board.">
    <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
      {#each FEATURES as item (item.title)}
        <div data-reveal-item>
          <Card class="flex h-full flex-col gap-2.5 p-6">
            <div class="inline-flex size-10 items-center justify-center rounded-fc-md bg-fc-surface text-fc-fg">
              <Icon icon={item.icon} size={22} />
            </div>
            <h3 class="text-fc-md font-bold text-fc-fg">{item.title}</h3>
            <p class="text-fc-sm text-fc-fg-muted">{item.description}</p>
          </Card>
        </div>
      {/each}
    </div>
  </Section>
</div>

<div use:scrollReveal={{ y: 20 }} class="pb-12">
  <Section id="viewer" title="The board in 3D" description="Tap Explore in 3D, then drag to turn it.">
    <PcbViewer />
  </Section>
</div>

<PreOrderForm />
