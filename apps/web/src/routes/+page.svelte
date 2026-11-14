<script lang="ts">
  import { FeatureSplit, PricingPlans, WorksWith, type FeatureItem, type WorksWithApp } from "@facile/muse";
  import Block from "$lib/components/Block.svelte";
  import Disassembly from "$lib/components/Disassembly.svelte";
  import HeroSection from "$lib/components/HeroSection.svelte";
  import MakersBlock from "$lib/components/MakersBlock.svelte";
  import PcbViewer from "$lib/components/PcbViewer.svelte";
  import PreOrderForm from "$lib/components/PreOrderForm.svelte";
  import SyncTopology from "$lib/components/SyncTopology.svelte";
  import { t } from "$lib/i18n/index.svelte";
  import { scrollReveal } from "$lib/motion";

  const features: FeatureItem[] = $derived([
    { icon: "solar:eye-bold-duotone", title: t("features.face.title"), text: t("features.face.text") },
    {
      icon: "solar:battery-charge-bold-duotone",
      title: t("features.battery.title"),
      text: t("features.battery.text"),
    },
    {
      icon: "solar:smile-circle-bold-duotone",
      title: t("features.account.title"),
      text: t("features.account.text"),
    },
    { icon: "solar:microphone-3-bold-duotone", title: t("features.voice.title"), text: t("features.voice.text") },
  ]);

  const apps: WorksWithApp[] = $derived([
    {
      name: "Bulle",
      icon: "solar:smile-circle-bold-duotone",
      line: t("with.bulle"),
      href: "https://bulle.facile.studio",
    },
    {
      name: "Fusible",
      icon: "solar:cpu-bolt-bold-duotone",
      line: t("with.fusible"),
      href: "https://fusible.facile.studio",
    },
  ]);
</script>

<div class="flex flex-col gap-24 pb-12 sm:gap-32">
  <HeroSection />

  <FeatureSplit id="features" class="scroll-mt-20" title={t("features.title")} description={t("features.line")}
    {features} />

  <WorksWith {apps} />

  <Block id="pricing" title={t("pricing.title")} description={t("pricing.note")}>
    <PricingPlans
      headingLevel={3}
      highlight={["ai.access"]}
      highlightOnly
      notes={{ solo: t("pricing.solo"), "*": t("pricing.paid") }}
      recommended="pro"
      suiteHref="https://suite.facile.studio/#tarifs"
    />
  </Block>

  <Block id="disassembly" title={t("inside.title")} description={t("inside.line")}>
    <Disassembly />
  </Block>

  <div use:scrollReveal={{ y: 20 }}>
    <Block id="connectivity" title={t("topo.title")} description={t("topo.line")}>
      <SyncTopology />
    </Block>
  </div>

  <div use:scrollReveal={{ y: 20 }}>
    <Block id="viewer" title={t("board.title")} description={t("board.line")}>
      <PcbViewer />
    </Block>
  </div>

  <MakersBlock />

  <PreOrderForm />
</div>
