<script lang="ts">
  import { Card, Icon } from "@facile/muse";
  import { PREORDER, formatPrice } from "./order";

  const PLATE = "inline-flex size-10 shrink-0 items-center justify-center rounded-fc-md bg-fc-surface text-fc-fg";
</script>

<Card class="flex h-full flex-col gap-6 p-6 sm:p-8">
  <div class="flex flex-col gap-3">
    <span class="w-fit rounded-fc-pill bg-fc-accent px-3 py-1 font-mono text-fc-xs text-fc-accent-fg">
      Pre-built
    </span>
    <div class="flex flex-col gap-1">
      <span class="text-fc-xs font-mono uppercase text-fc-fg-muted">{PREORDER.priceLabel}</span>
      <p class="flex items-baseline gap-2">
        <span class="font-mono text-fc-3xl font-bold text-fc-fg">{formatPrice(PREORDER.unitPrice)}</span>
        <span class="text-fc-sm text-fc-fg-muted">per unit</span>
      </p>
    </div>
  </div>

  <div class="flex flex-col gap-3">
    <h3 class="text-fc-sm font-bold text-fc-fg">What you get</h3>
    <ul class="flex flex-col gap-3">
      {#each PREORDER.includes as item (item.label)}
        <li class="flex items-center gap-3 text-fc-sm text-fc-fg">
          <span class={PLATE}><Icon icon={item.icon} size={20} /></span>
          {item.label}
        </li>
      {/each}
      <li class="flex items-center gap-3 text-fc-sm text-fc-fg">
        <span class={PLATE}><Icon icon="solar:calendar-bold-duotone" size={20} /></span>
        {PREORDER.shipLabel}: {PREORDER.shipDate}
      </li>
      <li class="flex items-center gap-3 text-fc-sm text-fc-fg">
        <span class={PLATE}><Icon icon="solar:delivery-bold-duotone" size={20} /></span>
        {PREORDER.shippingNote}
      </li>
    </ul>
  </div>

  <div class="mt-auto flex gap-3 rounded-fc-md bg-fc-surface p-4">
    <Icon icon="solar:shield-check-bold-duotone" size={20} class="mt-0.5 text-fc-fg" />
    <p class="text-fc-sm leading-relaxed text-fc-fg-muted">
      A pre-order is a reservation. We email you to confirm it. {PREORDER.paymentNote}
    </p>
  </div>
</Card>
