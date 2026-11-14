<script lang="ts">
  import { Button, Card, Divider, Field, Icon, Input, Textarea } from "@facile/muse";
  import Block from "./Block.svelte";
  import { tick } from "svelte";
  import OrderSuccess from "./preorder/OrderSuccess.svelte";
  import OrderSummary from "./preorder/OrderSummary.svelte";
  import QuantityStepper from "./preorder/QuantityStepper.svelte";
  import {
    ORDER_FIELDS, PREORDER, clampQuantity, formatPrice, orderMailto, validateOrder,
    type Order, type OrderField,
  } from "./preorder/order";
  import { t } from "$lib/i18n/index.svelte";

  const FIELD = "bg-fc-surface";
  const GROUP_LABEL = "text-fc-xs font-mono uppercase text-fc-fg-muted";

  const STEPS = [
    { icon: "solar:letter-bold-duotone", title: "po.step1.title", body: "po.step1.body" },
    { icon: "solar:chat-round-check-bold-duotone", title: "po.step2.title", body: "po.step2.body" },
    { icon: "solar:wallet-money-bold-duotone", title: "po.step3.title", body: "po.step3.body" },
  ] as const;

  let order: Order = $state({ name: "", email: "", quantity: 1, message: "" });
  let touched: Partial<Record<OrderField, boolean>> = $state({});
  let submitted = $state(false);
  let sent = $state(false);
  let form: HTMLFormElement | undefined = $state();

  const errors = $derived(validateOrder(order));
  const shown = $derived(
    Object.fromEntries(ORDER_FIELDS.map((f) => [f, submitted || touched[f] ? message(f) : undefined])),
  );
  const units = $derived(clampQuantity(order.quantity));
  const total = $derived(formatPrice(units * PREORDER.unitPrice));

  /**
   * The visitor's language version of a field's validation error, or nothing when the field is valid.
   */
  function message(field: OrderField): string | undefined {
    if (!errors[field]) return undefined;
    if (field === "name") return t("po.err.name");
    if (field === "quantity") return t("po.err.quantity", { max: PREORDER.maxQuantity });
    return order.email.trim() ? t("po.err.emailShape") : t("po.err.email");
  }

  /**
   * Validates the order, focuses the first invalid field, or opens the mail app and shows the success panel.
   */
  async function submit(event: SubmitEvent) {
    event.preventDefault();
    submitted = true;
    if (Object.keys(errors).length > 0) {
      await tick();
      form?.querySelector<HTMLElement>('[aria-invalid="true"]')?.focus();
      return;
    }
    window.location.href = orderMailto(order);
    sent = true;
  }

  /**
   * Returns to the filled-in form so the visitor can change the order.
   */
  async function edit() {
    sent = false;
    await tick();
    form?.querySelector<HTMLElement>("input")?.focus();
  }
</script>

<Block id="preorder" title={t("po.title")} description={t("po.line")}>
  <div class="grid gap-6 lg:grid-cols-[5fr_7fr] lg:items-stretch">
    <OrderSummary />

    {#if sent}
      <OrderSuccess {order} onedit={edit} />
    {:else}
      <Card class="flex flex-col gap-6 p-6 sm:p-8">
        <form bind:this={form} onsubmit={submit} novalidate class="flex flex-col gap-6">
          <div class="flex flex-col gap-5">
            <span class={GROUP_LABEL}>{t("po.details")}</span>
            <div class="grid grid-cols-1 gap-5 sm:grid-cols-2">
              <Field label={t("po.name")} error={shown.name}>
                <Input
                  class={FIELD}
                  bind:value={order.name}
                  required
                  autocomplete="name"
                  onblur={() => (touched.name = true)}
                />
              </Field>
              <Field label={t("po.email")} error={shown.email}>
                <Input
                  class={FIELD}
                  type="email"
                  bind:value={order.email}
                  required
                  autocomplete="email"
                  onblur={() => (touched.email = true)}
                />
              </Field>
            </div>
          </div>

          <Divider />

          <div class="flex flex-col gap-5">
            <span class={GROUP_LABEL}>{t("po.order")}</span>
            <QuantityStepper
              bind:value={order.quantity}
              error={shown.quantity}
              onblur={() => (touched.quantity = true)}
            />
            <Field label={t("po.message")} helper={t("po.messageHelp")}>
              <Textarea class={FIELD} bind:value={order.message} rows={4} />
            </Field>
          </div>

          <Divider />

          <dl class="grid grid-cols-2 gap-x-4 gap-y-2.5 rounded-fc-md bg-fc-surface p-4 text-fc-sm">
            <dt class="text-fc-fg-muted">{t("po.quantity")}</dt>
            <dd class="text-right font-mono text-fc-fg">{units} × {formatPrice(PREORDER.unitPrice)}</dd>
            <dt class="text-fc-fg-muted">{t("po.subtotal")}</dt>
            <dd class="text-right font-mono font-bold text-fc-fg">{total}</dd>
            <dd class="col-span-2 text-fc-xs text-fc-fg-muted">{t("po.payment")}</dd>
          </dl>

          <div class="flex flex-col gap-3 sm:flex-row sm:items-center">
            <Button
              type="submit"
              variant="primary"
              size="lg"
              icon="solar:cart-large-2-bold-duotone"
              class="w-full sm:w-auto"
            >
              {t("po.submit")}
            </Button>
            <p class="text-fc-xs text-fc-fg-muted">{t("po.opensMail")} {t("po.privacy")}</p>
          </div>
        </form>
      </Card>
    {/if}
  </div>

  <div class="grid grid-cols-1 gap-4 sm:grid-cols-3">
    {#each STEPS as step, i (step.title)}
      <Card class="flex items-start gap-3 p-5">
        <span
          class="inline-flex size-9 shrink-0 items-center justify-center rounded-fc-md bg-fc-surface text-fc-fg"
        >
          <Icon icon={step.icon} size={18} />
        </span>
        <div class="flex min-w-0 flex-col gap-1">
          <span class="text-fc-xs font-mono uppercase text-fc-fg-muted">{t("po.step", { n: i + 1 })}</span>
          <span class="text-fc-sm font-bold text-fc-fg">{t(step.title)}</span>
          <span class="text-fc-xs leading-relaxed text-fc-fg-muted">{t(step.body)}</span>
        </div>
      </Card>
    {/each}
  </div>
</Block>