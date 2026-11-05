<script lang="ts">
  import { Button, Card, Field, Input, Section, Textarea } from "@facile/muse";
  import { PREORDER_PRICE, RELEASE_TAG } from "$lib/data";

  const CONTACT = "contact@facile.studio";
  const DESCRIPTION = "Reserve a pre-built Pastille. The full release ships in March 2027.";

  let name = $state("");
  let email = $state("");
  let quantity = $state("1");
  let message = $state("");

  function submit(event: SubmitEvent) {
    event.preventDefault();
    const subject = `Pastille pre-order (${RELEASE_TAG})`;
    const body = [`Name: ${name}`, `Email: ${email}`, `Quantity: ${quantity}`, "", message].join("\n");
    window.location.href = `mailto:${CONTACT}?subject=${encodeURIComponent(subject)}&body=${encodeURIComponent(body)}`;
  }
</script>

<Section id="preorder" title="Pre-order Pastille" description={DESCRIPTION}>
  <div class="grid gap-8 lg:grid-cols-[5fr_7fr] lg:items-start">
    <div class="flex flex-col gap-4">
      <span class="w-fit rounded-fc-pill bg-fc-accent px-3 py-1 font-mono text-fc-xs text-fc-accent-fg">
        Pre-built
      </span>
      <p class="font-mono text-fc-3xl font-semibold text-fc-fg">{PREORDER_PRICE}</p>
      <p class="text-fc-sm text-fc-fg-muted leading-relaxed">
        One assembled Pastille per unit. Send the form and we reply by email to confirm your order.
      </p>
      <p class="font-mono text-fc-xs uppercase text-fc-fg-muted">Ships March 2027</p>
    </div>

    <Card class="flex flex-col gap-5 p-6 sm:p-8">
      <form onsubmit={submit} class="grid grid-cols-1 gap-5 sm:grid-cols-2">
        <div class="sm:col-span-1">
          <Field label="Name">
            {#snippet children({ id })}
              <Input {id} bind:value={name} required autocomplete="name" placeholder="Ada Lovelace" />
            {/snippet}
          </Field>
        </div>
        <div class="sm:col-span-1">
          <Field label="Email">
            {#snippet children({ id })}
              <Input {id} type="email" bind:value={email} required autocomplete="email" placeholder="you@example.com" />
            {/snippet}
          </Field>
        </div>
        <div class="sm:col-span-1">
          <Field label="Quantity" helper="Up to 20 per order.">
            {#snippet children({ id })}
              <Input {id} type="number" min="1" max="20" bind:value={quantity} required />
            {/snippet}
          </Field>
        </div>
        <div class="sm:col-span-2">
          <Field label="Message" helper="Optional. Tell us how you plan to use it.">
            {#snippet children({ id })}
              <Textarea {id} bind:value={message} rows={4} />
            {/snippet}
          </Field>
        </div>
        <div class="sm:col-span-2">
          <Button type="submit" variant="primary" size="lg" class="w-full sm:w-auto">Send pre-order</Button>
        </div>
      </form>
    </Card>
  </div>
</Section>
