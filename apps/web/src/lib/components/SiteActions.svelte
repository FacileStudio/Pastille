<script lang="ts">
  import { Button, Icon, IconButton } from "@facile/muse";
  import { GITHUB_ICON, MOON_ICON, SUN_ICON } from "$lib/icons";
  import { theme, toggleTheme } from "$lib/theme.svelte";

  const ROUND = "size-(--fc-ctl) rounded-fc-inner bg-fc-surface/70 text-fc-fg hover:bg-fc-surface";
</script>

<IconButton
  variant="ghost"
  class={ROUND}
  aria-label={theme.value === "dark" ? "Switch to light theme" : "Switch to dark theme"}
  onclick={toggleTheme}
>
  <Icon icon={theme.value === "dark" ? SUN_ICON : MOON_ICON} size={16} />
</IconButton>
<Button
  variant="primary"
  size="md"
  class="h-(--fc-ctl) rounded-fc-inner px-4"
  href="https://github.com/FacileStudio/Pastille"
  target="_blank"
  rel="noreferrer"
  icon={GITHUB_ICON}>GitHub</Button
>
