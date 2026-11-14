<script lang="ts">
  import { Button, Card, Icon } from "@facile/muse";
  import { onMount } from "svelte";
  import { copyText } from "./clipboard";
  import { PREORDER, formatPrice, orderMailto, orderText, type Order } from "./order";

  interface Props {
    order: Order;
    onedit: () => void;
  }

  let { order, onedit }: Props = $props();

  let heading: HTMLHeadingElement | undefined = $state();
  let copyStatus = $state("");

  const rows = $derived([
    { label: "Name", value: order.name.trim() },
    { label: "Email", value: order.email.trim() },
    { label: "Quantity", value: String(order.quantity) },
    { label: "Total", value: `${formatPrice(order.quantity * PREORDER.unitPrice)} before shipping` },
  ]);

  onMount(() => heading?.focus());

  /**
   * Copies the order summary and tells the visitor whether it worked.
   */
  async function copy() {
    const copied = await copyText(orderText(order));
    copyStatus = copied ? "Order details copied." : "Copy did not work. Select the details above instead.";
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
        Your reservation email is ready
      </h3>
      <p class="text-fc-sm leading-relaxed text-fc-fg-muted">
        Your mail app should open with your details filled in. Send it, and we reply by email to confirm your
        reservation.
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
    No mail app opened? Copy the details and send them to
    <a href="mailto:{PREORDER.contact}" class="font-medium text-fc-fg underline">{PREORDER.contact}</a>.
  </p>

  <div class="flex flex-col gap-2">
    <div class="flex flex-col gap-3 sm:flex-row sm:flex-wrap sm:items-center">
      <Button variant="primary" size="lg" icon="solar:copy-bold-duotone" onclick={copy}>Copy order details</Button>
      <Button variant="secondary" size="lg" icon="solar:letter-bold-duotone" href={orderMailto(order)}>
        Open in mail app
      </Button>
      <Button variant="ghost" size="lg" icon="solar:pen-bold-duotone" onclick={onedit}>Edit order</Button>
    </div>
    <p role="status" class="min-h-5 text-fc-sm text-fc-fg-muted">{copyStatus}</p>
  </div>
</Card>
