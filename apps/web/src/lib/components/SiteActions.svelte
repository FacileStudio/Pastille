<script lang="ts">
  import { Button, Icon, IconButton } from "@facile/muse";
  import { MOON_ICON, SUN_ICON } from "$lib/icons";
  import { t } from "$lib/i18n/index.svelte";
  import { scrollToSection } from "$lib/motion";
  import { theme, toggleTheme } from "$lib/theme.svelte";

  const ROUND = "size-(--fc-ctl) rounded-fc-inner bg-fc-surface/70 text-fc-fg hover:bg-fc-surface";
</script>

<IconButton
  variant="ghost"
  class={ROUND}
  aria-label={theme.value === "dark" ? t("theme.light") : t("theme.dark")}
  onclick={toggleTheme}
>
  <Icon icon={theme.value === "dark" ? SUN_ICON : MOON_ICON} size={16} />
</IconButton>
<Button
  variant="primary"
  size="md"
  class="h-(--fc-ctl) rounded-fc-inner px-4"
  href="#preorder"
  icon="solar:cart-large-2-bold-duotone"
  onclick={(e) => scrollToSection("preorder", e)}>{t("hero.preorder")}</Button
>
