<script lang="ts">
  import { ForDevelopers, type DeveloperRow } from "@facile/muse";
  import { REPO_URL } from "$lib/data";
  import { t } from "$lib/i18n/index.svelte";

  const rows: DeveloperRow[] = $derived([
    { kind: "source", text: t("makers.source"), href: REPO_URL },
    {
      icon: "solar:code-square-bold-duotone",
      title: t("makers.firmware.title"),
      text: t("makers.firmware"),
      href: `${REPO_URL}/tree/main/apps/firmware`,
    },
    {
      icon: "solar:document-text-bold-duotone",
      title: t("makers.spec.title"),
      text: t("makers.spec"),
      href: `${REPO_URL}/tree/main/docs/spec`,
    },
    {
      icon: "solar:cpu-bolt-bold-duotone",
      title: t("makers.fusible.title"),
      text: t("makers.fusible"),
      href: "https://fusible.facile.studio",
    },
  ]);
</script>

<ForDevelopers
  id="makers"
  class="scroll-mt-20"
  title={t("makers.title")}
  description={t("makers.line")}
  {rows}
  selfHostHref="https://suite.facile.studio/auto-hebergement"
/>
