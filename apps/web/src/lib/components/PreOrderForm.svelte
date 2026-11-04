<script lang="ts">
  import { Button, Field, Input, Section, Textarea } from "@facile/muse";
  import { PREORDER_PRICE, RELEASE_TAG } from "$lib/data";

  const CONTACT = "contact@facile.studio";
  const DESCRIPTION = `${PREORDER_PRICE} per pre-built Pastille. The full release ships in March 2027. Reply by email.`;

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

<Section
  id="preorder"
  title="Pre-order Pastille"
  description={DESCRIPTION}
>
  <form onsubmit={submit} class="grid grid-cols-1 gap-4 max-w-2xl">
    <Field label="Name">
      {#snippet children({ id })}
        <Input {id} bind:value={name} required autocomplete="name" placeholder="Ada Lovelace" />
      {/snippet}
    </Field>
    <Field label="Email">
      {#snippet children({ id })}
        <Input {id} type="email" bind:value={email} required autocomplete="email" placeholder="you@example.com" />
      {/snippet}
    </Field>
    <Field label="Quantity" helper="One Pastille per unit. Shipping starts in March 2027.">
      {#snippet children({ id })}
        <Input {id} type="number" min="1" max="20" bind:value={quantity} required />
      {/snippet}
    </Field>
    <Field label="Message" helper="Optional. Tell us how you plan to use it.">
      {#snippet children({ id })}
        <Textarea {id} bind:value={message} rows={4} />
      {/snippet}
    </Field>
    <div>
      <Button type="submit" variant="primary" size="lg">Send pre-order</Button>
    </div>
  </form>
</Section>
