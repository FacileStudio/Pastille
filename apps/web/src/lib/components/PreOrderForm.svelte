<script lang="ts">
  import { Alert, Button, Card, Checkbox, Field, Icon, Input } from "@facile/muse";
  import { onMount } from "svelte";
  import Block from "./Block.svelte";
  import OrderSummary from "./preorder/OrderSummary.svelte";
  import QuantityStepper from "./preorder/QuantityStepper.svelte";
  import { orderMailto } from "./preorder/order";
  import { joinWaitlist, quantityOk, type JoinResult } from "./preorder/waitlist";
  import { locale, t } from "$lib/i18n/index.svelte";

  const FIELD = "bg-fc-surface";
  const EMAIL = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  const MAILTO = orderMailto({ name: "", email: "", quantity: 1, message: "" });

  const STEPS = [
    { icon: "solar:letter-bold-duotone", title: "po.step1.title", body: "po.step1.body" },
    { icon: "solar:chat-round-check-bold-duotone", title: "po.step2.title", body: "po.step2.body" },
    { icon: "solar:wallet-money-bold-duotone", title: "po.step3.title", body: "po.step3.body" },
  ] as const;

  let ready = $state(false);
  let email = $state("");
  let quantity: number | undefined = $state(undefined);
  let consent = $state(false);
  let checked = $state(false);
  let busy = $state(false);
  let result: JoinResult | null = $state(null);

  const emailError = $derived(emailMessage(checked, email, result));
  const quantityError = $derived(
    !quantityOk(quantity) || result === "invalid_quantity" ? t("po.err.quantity") : undefined,
  );
  const consentError = $derived(
    (checked && !consent) || result === "consent_required" ? t("po.err.consent") : undefined,
  );
  const failure = $derived(
    result === "rate_limited" ? t("po.err.rate") : result === "failed" ? t("po.err.failed") : undefined,
  );

  onMount(() => (ready = true));

  /**
   * The email field's error once the form was submitted, or the server's refusal of the address.
   */
  function emailMessage(shown: boolean, value: string, outcome: JoinResult | null): string | undefined {
    if (shown && !value.trim()) return t("po.err.email");
    if ((shown && !EMAIL.test(value.trim())) || outcome === "invalid_email") return t("po.err.emailShape");
    return undefined;
  }

  /**
   * Checks the address and the consent box, then puts the address on the pre-order list.
   */
  async function submit(event: SubmitEvent) {
    event.preventDefault();
    checked = true;
    if (busy || !EMAIL.test(email.trim()) || !consent || !quantityOk(quantity)) return;
    busy = true;
    result = await joinWaitlist(email, locale(), quantity);
    busy = false;
  }
</script>

<Block id="preorder" title={t("po.title")} description={t("po.line")}>
  <div class="grid gap-6 lg:grid-cols-[5fr_7fr] lg:items-stretch">
    <OrderSummary />

    <Card class="flex flex-col justify-center gap-6 p-6 sm:p-8">
      {#if !ready}
        <Button variant="primary" size="lg" href={MAILTO} icon="solar:letter-bold-duotone" class="w-full sm:w-auto">
          {t("po.mailLink")}
        </Button>
      {:else if result === "ok"}
        <Alert tone="success" title={t("po.done.title")}>{t("po.done.body")}</Alert>
      {:else}
        <form onsubmit={submit} novalidate class="flex flex-col gap-5">
          <Field label={t("po.email")} error={emailError}>
            <Input class={FIELD} type="email" bind:value={email} required autocomplete="email" />
          </Field>
          <QuantityStepper bind:value={quantity} error={quantityError} />
          <Field error={consentError}>
            <Checkbox label={t("po.consent")} bind:checked={consent} />
          </Field>
          {#if failure}
            <Alert tone="danger" title={failure} />
          {/if}
          <div class="flex flex-col gap-3 sm:flex-row sm:items-center">
            <Button
              type="submit"
              variant="primary"
              size="lg"
              icon="solar:cart-large-2-bold-duotone"
              class="w-full sm:w-auto"
              disabled={busy}
            >
              {busy ? t("po.sending") : t("po.submit")}
            </Button>
            <a class="text-fc-xs text-fc-fg-muted underline underline-offset-4" href={MAILTO}>{t("po.mailLink")}</a>
          </div>
          <p class="text-fc-xs text-fc-fg-muted">{t("po.payment")}</p>
        </form>
      {/if}
      <p class="text-fc-xs text-fc-fg-muted">{t("po.privacy")}</p>
    </Card>
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
