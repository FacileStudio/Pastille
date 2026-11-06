<script lang="ts">
  import { gsap } from "gsap";
  import { Button, Icon, IconButton, prefersReducedMotion } from "@facile/muse";
  import { NAV_SECTIONS } from "$lib/data";
  import { CLOSE_ICON, GITHUB_ICON, PASTILLE_LOGO } from "$lib/icons";

  interface Props {
    open: boolean;
    active: string;
    onselect: (id: string) => void;
  }

  let { open = $bindable(), active, onselect }: Props = $props();

  let scrimEl: HTMLDivElement | null = $state(null);
  let drawerEl: HTMLElement | null = $state(null);

  $effect(() => {
    if (!open || !scrimEl || !drawerEl) return;
    drawerEl.querySelector<HTMLElement>("button")?.focus();
    if (prefersReducedMotion()) return;
    gsap.fromTo(scrimEl, { opacity: 0 }, { opacity: 1, duration: 0.25, ease: "power2.out" });
    gsap.fromTo(drawerEl, { xPercent: 100 }, { xPercent: 0, duration: 0.34, ease: "power3.out" });
  });

  /**
   * Slides the drawer out, closes it, then runs `afterClose` once it is gone.
   */
  function close(afterClose?: () => void) {
    if (!scrimEl || !drawerEl || prefersReducedMotion()) {
      open = false;
      afterClose?.();
      return;
    }
    gsap.to(scrimEl, { opacity: 0, duration: 0.22, ease: "power2.in" });
    gsap.to(drawerEl, {
      xPercent: 100,
      duration: 0.26,
      ease: "power3.in",
      onComplete: () => {
        open = false;
        afterClose?.();
      },
    });
  }

  /**
   * Closes the drawer when the visitor presses Escape.
   */
  function onkeydown(event: KeyboardEvent) {
    if (open && event.key === "Escape") close();
  }
</script>

<svelte:window {onkeydown} />

{#if open}
  <div class="fixed inset-0 z-50 lg:hidden" role="dialog" aria-modal="true" aria-label="Navigation">
    <div
      bind:this={scrimEl}
      role="presentation"
      class="fixed inset-0 bg-fc-scrim backdrop-blur-xs"
      onclick={() => close()}
    ></div>
    <aside
      bind:this={drawerEl}
      class="fixed top-3 right-3 bottom-3 flex w-72 max-w-[calc(100vw-1.5rem)] flex-col justify-between
        overflow-y-auto rounded-fc-lg bg-fc-component/95 p-5 backdrop-blur-xl"
    >
      <div class="flex flex-col gap-6">
        <div class="flex items-center justify-between">
          <span class="inline-flex items-center gap-2 text-fc-md font-semibold text-fc-fg">
            <Icon icon={PASTILLE_LOGO} size={20} />
            <span>Pastille</span>
          </span>
          <IconButton
            variant="ghost"
            class="size-11 border-0 bg-fc-surface text-fc-fg"
            aria-label="Close navigation menu"
            onclick={() => close()}
          >
            <Icon icon={CLOSE_ICON} size={16} />
          </IconButton>
        </div>
        <nav aria-label="Mobile" class="flex flex-col gap-1.5">
          {#each NAV_SECTIONS as item (item.id)}
            <a
              href="#{item.id}"
              aria-current={active === item.id ? "location" : undefined}
              onclick={(e) => {
                e.preventDefault();
                close(() => onselect(item.id));
              }}
              class="inline-flex h-11 items-center gap-2.5 rounded-fc-pill px-4 text-fc-sm font-medium
                transition-colors {active === item.id
                ? 'bg-fc-accent text-fc-accent-fg'
                : 'text-fc-fg-muted hover:bg-fc-surface hover:text-fc-fg'}"
            >
              <Icon icon={item.icon} size={18} /><span>{item.label}</span>
            </a>
          {/each}
        </nav>
      </div>
      <Button
        variant="primary"
        size="md"
        class="mt-6 w-full"
        href="https://github.com/FacileStudio/Pastille"
        target="_blank"
        rel="noreferrer"
        icon={GITHUB_ICON}>GitHub</Button
      >
    </aside>
  </div>
{/if}
