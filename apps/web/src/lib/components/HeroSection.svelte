<script lang="ts">
  import { Button } from "@facile/muse";
  import HeroDemo from "$lib/components/demo/HeroDemo.svelte";
  import { t } from "$lib/i18n/index.svelte";
  import { scrollReveal, scrollToSection } from "$lib/motion";
</script>

<section use:scrollReveal={{ delay: 0.12, stagger: 0.07 }} class="flex flex-col gap-6 pt-4 md:pt-8">
  <div data-reveal-item class="flex max-w-2xl flex-col gap-3">
    <span class="w-fit rounded-fc-pill bg-fc-surface px-3 py-1 text-fc-xs font-medium text-fc-fg-muted">
      {t("hero.badge")}
    </span>
    <h1 class="text-fc-2xl font-bold tracking-tight text-fc-fg sm:text-fc-3xl">{t("hero.title")}</h1>
    <p class="text-fc-md text-fc-fg-muted sm:text-fc-lg">{t("hero.subtitle")}</p>
  </div>

  <div data-reveal-item class="flex flex-col gap-3 sm:flex-row sm:flex-wrap sm:items-center">
    <Button variant="primary" size="lg" href="#preorder" icon="solar:cart-large-2-bold-duotone"
      class="w-full sm:w-auto" onclick={(e) => scrollToSection("preorder", e)}>{t("hero.preorder")}</Button>
    <Button variant="secondary" size="lg" href="https://bulle.facile.studio" icon="solar:smile-circle-bold-duotone"
      class="w-full sm:w-auto">{t("hero.bulle")}</Button>
  </div>

  <div data-reveal-item class="pt-2">
    <HeroDemo />
  </div>
</section>
