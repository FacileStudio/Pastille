<script lang="ts">
  import "../app.css";
  import type { Snippet } from "svelte";
  import { browser } from "$app/environment";
  import {
    Button,
    Icon,
    IconButton,
    LegalFooter,
    Page,
    Topbar,
    icons,
    provideLocale,
    registerIcons,
  } from "@facile/muse";
  import { PASTILLE_ICONS, PASTILLE_LOGO } from "$lib/icons";
  import IntroLoader from "$lib/components/IntroLoader.svelte";

  let { children }: { children: Snippet } = $props();

  registerIcons(PASTILLE_ICONS);
  provideLocale("en");

  let theme = $state<"light" | "dark" | "system">("dark");

  $effect(() => {
    if (!browser) return;
    const root = document.documentElement;
    root.classList.toggle("dark", theme === "dark");
    root.classList.toggle("light", theme === "light");
    localStorage.setItem("pastille-theme", theme);
  });

  if (browser) {
    const saved = localStorage.getItem("pastille-theme");
    if (saved === "light" || saved === "dark" || saved === "system") {
      theme = saved;
    } else {
      document.documentElement.classList.add("dark");
    }
  }
</script>

<IntroLoader />

<div class="flex min-h-dvh w-full flex-col bg-fc-page text-fc-fg">
  <Topbar class="px-4 sm:px-6 md:px-10">
    <a
      href="/"
      class="inline-flex items-center gap-2.5 text-fc-md font-semibold text-fc-fg
        focus-visible:outline-2 focus-visible:outline-offset-2
        focus-visible:outline-fc-ring rounded-fc-xs"
    >
      <Icon icon={PASTILLE_LOGO} size={22} class="text-fc-fg" />
      <span>Pastille</span>
      <span class="rounded-fc-pill bg-fc-surface px-2.5 py-0.5 text-fc-xs font-mono text-fc-fg-muted">
        BOHS-01 · v0.2.0
      </span>
    </a>

    <nav aria-label="Primary" class="hidden md:flex items-center gap-1">
      <Button variant="ghost" size="sm" href="#viewer">3D Board</Button>
      <Button variant="ghost" size="sm" href="#architecture">Architecture</Button>
      <Button variant="ghost" size="sm" href="#protocol">Wire Protocol</Button>
      <Button variant="ghost" size="sm" href="#comparison">Comparison</Button>
    </nav>

    <div class="flex items-center gap-1.5">
      {#each [["light", icons.sun, "Light theme"], ["dark", icons.moon, "Dark theme"]] as [mode, icon, label] (mode)}
        <IconButton
          variant={theme === mode ? "default" : "ghost"}
          aria-label={label}
          aria-pressed={theme === mode}
          onclick={() => (theme = mode as typeof theme)}
        >
          <Icon {icon} size={16} />
        </IconButton>
      {/each}
      <Button
        variant="primary"
        size="sm"
        href="https://github.com/FacileStudio/Pastille"
        target="_blank"
        rel="noreferrer"
        icon={icons.code}
      >
        GitHub
      </Button>
    </div>
  </Topbar>

  <main class="flex-1">
    <Page width="xl">
      {@render children()}
    </Page>
  </main>

  <div class="mx-auto w-full max-w-fc-xl px-4 pb-10 pt-6 sm:px-6 md:px-10">
    <LegalFooter hideLocaleSwitch={true} />
  </div>
</div>
