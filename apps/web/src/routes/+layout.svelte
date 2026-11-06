<script lang="ts">
  import "../app.css";
  import type { Snippet } from "svelte";
  import { Icon, Page, provideLocale, registerIcons } from "@facile/muse";
  import { GITHUB_ICON, ICONS, PASTILLE_LOGO, RELEASES_ICON, SPEC_ICON } from "$lib/icons";
  import IntroLoader from "$lib/components/IntroLoader.svelte";
  import SiteHeader from "$lib/components/SiteHeader.svelte";

  let { children }: { children: Snippet } = $props();
  registerIcons(ICONS);
  provideLocale("en");

  const FOOTER_LINKS = [
    { label: "BOHS-01 Spec", icon: SPEC_ICON, href: "https://github.com/FacileStudio/Pastille/tree/main/docs/spec" },
    { label: "Releases", icon: RELEASES_ICON, href: "https://github.com/FacileStudio/Pastille/releases" },
    { label: "GitHub", icon: GITHUB_ICON, href: "https://github.com/FacileStudio/Pastille" },
  ];
</script>
<IntroLoader />
<div class="flex min-h-dvh w-full flex-col bg-fc-page text-fc-fg">
  <SiteHeader />

  <main class="flex-1"><Page width="xl">{@render children()}</Page></main>

  <footer class="mx-auto w-full max-w-fc-xl px-4 pb-10 pt-8 sm:px-6 md:px-10">
    <div
      class="flex flex-col items-center justify-between gap-4 rounded-fc-pill bg-fc-component px-8 py-4
        text-fc-xs text-fc-fg-muted sm:flex-row"
    >
      <div class="flex flex-wrap items-center gap-2.5">
        <span class="inline-flex items-center gap-1.5 font-semibold text-fc-fg">
          <Icon icon={PASTILLE_LOGO} size={16} />
          <span>Pastille</span>
        </span>
        <span>·</span>
        <span>
          Made by
          <a href="https://facile.studio" target="_blank" rel="noreferrer" class="font-medium text-fc-fg"
            >Facile Studio</a
          >
        </span>
      </div>
      <div class="flex flex-wrap items-center gap-4">
        <span class="font-mono">CERN-OHL-S v2 · Apache 2.0</span>
        {#each FOOTER_LINKS as link (link.label)}
          <a href={link.href} target="_blank" rel="noreferrer" class="inline-flex items-center gap-1.5 hover:text-fc-fg"
            ><Icon icon={link.icon} size={14} />{link.label}</a>
        {/each}
      </div>
    </div>
  </footer>
</div>
