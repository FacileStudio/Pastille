<script lang="ts">
  import { Button, Card, Icon } from "@facile/muse";
  import { onMount } from "svelte";
  import { t } from "$lib/i18n/index.svelte";
  import { copyText } from "./clipboard";
  import { PREORDER, formatPrice, orderMailto, orderText, type Order } from "./order";

  interface Props {
    order: Order;
    onedit: () => void;
  }

  let { order, onedit }: Props = $props();

  let heading: HTMLHeadingElement | undefined = $state();
  let copyStatus = $state("");

  const total = $derived(formatPrice(order.quantity * PREORDER.unitPrice));
  const rows = $derived([
    { label: t("po.name"), value: order.name.trim() },
    { label: t("po.email"), value: order.email.trim() },
    { label: t("po.quantity"), value: String(order.quantity) },
    { label: t("po.ok.total"), value: t("po.ok.totalValue", { total }) },
  ]);

  onMount(() => heading?.focus());

  /**
   * Copies the order summary and tells the visitor whether it worked.
   */
  async function copy() {
    const copied = await copyText(orderText(order));
    copyStatus = copied ? t("po.ok.copied") : t("po.ok.copyFailed");
  }
</script>

<Card class="flex flex-col gap-6 p-6 sm:p-8">
  <div class="flex items-start gap-3">
    <span
      class="inline-flex size-10 shrink-0 items-center justify-center rounded-fc-md bg-fc-success/10 text-fc-success"
    >
      <Icon icon="solar:check-circle-bold-duotone" size={22} />
    </span>
    <div class="flex flex-col gap-1">
      <h3 bind:this={heading} tabindex="-1" class="text-fc-md font-bold text-fc-fg focus:outline-none">
        {t("po.ok.title")}
      </h3>
      <p class="text-fc-sm leading-relaxed text-fc-fg-muted">
        {t("po.ok.body")}
      </p>
    </div>
  </div>

  <dl class="grid grid-cols-[auto_1fr] gap-x-6 gap-y-2 rounded-fc-md bg-fc-surface p-4 text-fc-sm">
    {#each rows as row (row.label)}
      <dt class="text-fc-fg-muted">{row.label}</dt>
      <dd class="min-w-0 break-words font-medium text-fc-fg">{row.value}</dd>
    {/each}
  </dl>

  <p class="text-fc-sm leading-relaxed text-fc-fg-muted">
    {t("po.ok.fallback")}
    <a href="mailto:{PREORDER.contact}" class="font-medium text-fc-fg underline">{PREORDER.contact}</a>.
  </p>

  <div class="flex flex-col gap-2">
    <div class="flex flex-col gap-3 sm:flex-row sm:flex-wrap sm:items-center">
      <Button variant="primary" size="lg" icon="solar:copy-bold-duotone" onclick={copy}>{t("po.ok.copy")}</Button>
      <Button variant="secondary" size="lg" icon="solar:letter-bold-duotone" href={orderMailto(order)}>
        {t("po.ok.open")}
      </Button>
      <Button variant="ghost" size="lg" icon="solar:pen-bold-duotone" onclick={onedit}>{t("po.ok.edit")}</Button>
    </div>
    <p role="status" class="min-h-5 text-fc-sm text-fc-fg-muted">{copyStatus}</p>
  </div>
</Card>
