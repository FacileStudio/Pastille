<script lang="ts">
  import "../app.css";
  import { onMount, type Snippet } from "svelte";
  import { LandingFooter, LandingHeader, Page, provideLocale, registerIcons } from "@facile/muse";
  import { ICONS, PASTILLE_LOGO } from "$lib/icons";
  import { NAV_SECTIONS, REPO_URL } from "$lib/data";
  import { bindLocale, rememberLocale, t, visitorLocale } from "$lib/i18n/index.svelte";
  import { scrollToSection, watchActiveSection } from "$lib/motion";
  import IntroLoader from "$lib/components/IntroLoader.svelte";
  import SiteActions from "$lib/components/SiteActions.svelte";

  let { children }: { children: Snippet } = $props();
  registerIcons(ICONS);
  let source = $state<string | null>(null);
  const i18n = provideLocale(() => source);
  bindLocale(() => i18n.locale);

  let activeSection = $state<string>(NAV_SECTIONS[0].id);
  const SECTION_IDS = NAV_SECTIONS.map((s) => s.id);
  const links = $derived(
    NAV_SECTIONS.map((s) => ({ href: `#${s.id}`, label: t(s.label), active: s.id === activeSection })),
  );

  const columns = $derived([
    {
      title: t("footer.product"),
      links: [
        { label: t("nav.preorder"), href: "#preorder" },
        { label: "Bulle", href: "https://bulle.facile.studio", external: true },
      ],
    },
    {
      title: t("footer.makers"),
      links: [
        { label: t("footer.spec"), href: `${REPO_URL}/tree/main/docs/spec`, external: true },
        { label: t("footer.releases"), href: `${REPO_URL}/releases`, external: true },
        { label: "GitHub", href: REPO_URL, external: true },
      ],
    },
    {
      title: t("footer.studio"),
      links: [{ label: t("footer.madeBy"), href: "https://facile.studio", external: true }],
    },
  ]);

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

  onMount(() => {
    source = visitorLocale();
    return watchActiveSection(SECTION_IDS, (id) => (activeSection = id));
  });
</script>

<svelte:document {onclick} />

<svelte:head>
  <title>{t("meta.title")}</title>
  <meta name="description" content={t("meta.description")} />
  <meta property="og:title" content={t("meta.title")} />
  <meta property="og:description" content={t("meta.description")} />
  <meta property="og:image:alt" content={t("meta.title")} />
  <meta name="twitter:title" content={t("meta.title")} />
  <meta name="twitter:description" content={t("meta.description")} />
</svelte:head>

<IntroLoader />
<div class="flex min-h-dvh w-full flex-col bg-fc-page text-fc-fg">
  <LandingHeader appName="Pastille" icon={PASTILLE_LOGO} {links}>
    {#snippet actions()}<SiteActions />{/snippet}
  </LandingHeader>

  <main class="flex-1"><Page width="xl">{@render children()}</Page></main>

  <LandingFooter
    appName="Pastille"
    icon={PASTILLE_LOGO}
    tagline={t("footer.tagline")}
    {columns}
    copyright="CERN-OHL-S v2 · Apache 2.0"
    legal={{ onLocaleChange: rememberLocale }}
  />
</div>
