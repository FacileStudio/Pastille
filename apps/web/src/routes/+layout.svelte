<script lang="ts">
  import "../app.css";
  import { onMount, type Snippet } from "svelte";
  import { LandingFooter, LandingHeader, Page, provideLocale, registerIcons } from "@facile/muse";
  import { ICONS, PASTILLE_LOGO } from "$lib/icons";
  import { NAV_SECTIONS } from "$lib/data";
  import { scrollToSection, watchActiveSection } from "$lib/motion";
  import IntroLoader from "$lib/components/IntroLoader.svelte";
  import SiteActions from "$lib/components/SiteActions.svelte";

  let { children }: { children: Snippet } = $props();
  registerIcons(ICONS);
  provideLocale("en");

  let activeSection = $state<string>(NAV_SECTIONS[0].id);
  const BAR_IDS = ["disassembly", "viewer", "architecture", "protocol", "quickstart", "preorder"];
  const SECTION_IDS = NAV_SECTIONS.map((s) => s.id);
  const barActive = $derived.by(() => {
    const at = SECTION_IDS.indexOf(activeSection as (typeof SECTION_IDS)[number]);
    const owners = BAR_IDS.filter((id) => SECTION_IDS.indexOf(id as (typeof SECTION_IDS)[number]) <= at);
    return owners[owners.length - 1];
  });
  const links = $derived(
    NAV_SECTIONS.filter((s) => BAR_IDS.includes(s.id)).map((s) => ({
      href: `#${s.id}`,
      label: s.label,
      active: s.id === barActive,
    })),
  );

  const FOOTER_COLUMNS = [
    {
      title: "Project",
      links: [
        { label: "BOHS-01 Spec", href: "https://github.com/FacileStudio/Pastille/tree/main/docs/spec", external: true },
        { label: "Releases", href: "https://github.com/FacileStudio/Pastille/releases", external: true },
        { label: "GitHub", href: "https://github.com/FacileStudio/Pastille", external: true },
      ],
    },
    {
      title: "Studio",
      links: [{ label: "Made by Facile Studio", href: "https://facile.studio", external: true }],
    },
  ];

  /**
   * Smooth-scrolls in-page nav links to their section, offset under the sticky bar.
   */
  function onclick(event: MouseEvent) {
    const anchor = (event.target as Element | null)?.closest<HTMLAnchorElement>('a[href^="#"]');
    const id = anchor?.getAttribute("href")?.slice(1);
    if (!id || !NAV_SECTIONS.some((s) => s.id === id)) return;
    activeSection = id;
    scrollToSection(id, event);
  }

  onMount(() =>
    watchActiveSection(SECTION_IDS, (id) => (activeSection = id)),
  );
</script>

<svelte:document {onclick} />

<IntroLoader />
<div class="flex min-h-dvh w-full flex-col bg-fc-page text-fc-fg">
  <LandingHeader appName="Pastille" icon={PASTILLE_LOGO} {links}>
    {#snippet actions()}<SiteActions />{/snippet}
  </LandingHeader>

  <main class="flex-1"><Page width="xl">{@render children()}</Page></main>

  <LandingFooter
    appName="Pastille"
    icon={PASTILLE_LOGO}
    tagline="Wear your assistant."
    columns={FOOTER_COLUMNS}
    copyright="CERN-OHL-S v2 · Apache 2.0"
    legal={{ hideLocaleSwitch: true }}
  />
</div>
