<script lang="ts">
  import { onMount } from "svelte";
  import { browser } from "$app/environment";
  import { Button, Icon, IconButton } from "@facile/muse";
  import { NAV_SECTIONS } from "$lib/data";
  import { GITHUB_ICON, MENU_ICON, MOON_ICON, PASTILLE_LOGO, SUN_ICON } from "$lib/icons";
  import NavDrawer from "$lib/components/NavDrawer.svelte";
  import { moveNavPill, scrollToSection, watchActiveSection } from "$lib/motion";

  const ROUND = "size-11 border-0 bg-fc-surface/70 text-fc-fg hover:bg-fc-surface rounded-fc-pill";

  let theme = $state<"light" | "dark">("dark");
  let activeSection = $state<string>(NAV_SECTIONS[0].id);
  let scrolled = $state(false);
  let mobileOpen = $state(false);
  let navPill: HTMLSpanElement | null = $state(null);
  let navEls: Record<string, HTMLAnchorElement> = $state({});

  if (browser && document.documentElement.classList.contains("light")) theme = "light";

  $effect(() => {
    if (!browser) return;
    document.documentElement.classList.toggle("dark", theme === "dark");
    document.documentElement.classList.toggle("light", theme === "light");
    try {
      localStorage.setItem("pastille-theme", theme);
    } catch {}
  });
  $effect(() => moveNavPill(navPill, navEls[activeSection], true));

  /**
   * Switches between the light and dark themes.
   */
  function toggleTheme() {
    theme = theme === "dark" ? "light" : "dark";
  }

  /**
   * Marks a section as active and scrolls to it.
   */
  function selectSection(id: string, event?: MouseEvent) {
    activeSection = id;
    scrollToSection(id, event);
  }

  onMount(() => {
    moveNavPill(navPill, navEls[activeSection], false);
    return watchActiveSection(
      NAV_SECTIONS.map((s) => s.id),
      (id) => (activeSection = id),
      (isScrolled) => (scrolled = isScrolled),
    );
  });
</script>

<div class="sticky top-0 z-30 flex w-full justify-center pointer-events-none px-3 sm:px-6">
  <header
    class="pointer-events-auto flex h-14 w-full min-w-0 items-center justify-between gap-3 transition-all
      duration-500 ease-fc {scrolled
      ? 'mt-3 max-w-6xl rounded-fc-pill bg-fc-surface/65 px-1.5 backdrop-blur-xl'
      : 'mt-0 max-w-fc-xl rounded-none bg-fc-page/80 px-2 sm:px-4 backdrop-blur-md'}"
  >
    <a
      href="/"
      class="inline-flex h-11 shrink-0 items-center gap-2 rounded-fc-pill px-3 text-fc-md font-semibold text-fc-fg
        transition-colors hover:bg-fc-component/70 focus-visible:outline-2
        focus-visible:outline-offset-2 focus-visible:outline-fc-ring"
    >
      <Icon icon={PASTILLE_LOGO} size={22} class="text-fc-fg" />
      <span>Pastille</span>
    </a>

    <nav aria-label="Primary" class="relative hidden min-w-0 items-center gap-0.5 p-1 lg:flex">
      <span
        bind:this={navPill}
        aria-hidden="true"
        class="pointer-events-none absolute inset-y-1 left-0 rounded-fc-pill bg-fc-accent opacity-0"
      ></span>
      {#each NAV_SECTIONS as item (item.id)}
        {@const active = activeSection === item.id}
        <a
          bind:this={navEls[item.id]}
          href="#{item.id}"
          aria-current={active ? "location" : undefined}
          onclick={(e) => selectSection(item.id, e)}
          class="relative z-10 inline-flex h-9 items-center gap-1.5 whitespace-nowrap rounded-fc-pill px-2.5 text-fc-xs
            font-medium transition-colors duration-200 focus-visible:outline-2 focus-visible:outline-offset-2
            focus-visible:outline-fc-ring pointer-coarse:h-11 xl:px-3
            {active ? 'text-fc-accent-fg' : 'text-fc-fg-muted hover:text-fc-fg'}"
        >
          <Icon icon={item.icon} size={14} class="hidden xl:block" /><span>{item.label}</span>
        </a>
      {/each}
    </nav>

    <div class="flex shrink-0 items-center gap-1.5">
      <IconButton
        variant="ghost"
        class={ROUND}
        aria-label={theme === "dark" ? "Switch to light theme" : "Switch to dark theme"}
        onclick={toggleTheme}
      >
        <Icon icon={theme === "dark" ? SUN_ICON : MOON_ICON} size={16} />
      </IconButton>
      <Button
        variant="primary"
        size="md"
        class="hidden h-11 px-4 rounded-fc-pill sm:inline-flex"
        href="https://github.com/FacileStudio/Pastille"
        target="_blank"
        rel="noreferrer"
        icon={GITHUB_ICON}>GitHub</Button
      >
      <IconButton
        variant="ghost"
        class="{ROUND} lg:hidden"
        aria-label="Open navigation menu"
        aria-expanded={mobileOpen}
        onclick={() => (mobileOpen = true)}
      >
        <Icon icon={MENU_ICON} size={18} />
      </IconButton>
    </div>
  </header>
</div>

<NavDrawer bind:open={mobileOpen} active={activeSection} onselect={(id) => selectSection(id)} />
