<script lang="ts">
  import { Field, Icon, IconButton, Input } from "@facile/muse";
  import { t } from "$lib/i18n/index.svelte";
  import { MAX_QUANTITY, stepQuantity } from "./waitlist";

  interface Props {
    value: number | undefined;
    error?: string;
  }

  let { value = $bindable(), error }: Props = $props();

  const STEP = "rounded-fc-md bg-fc-surface text-fc-fg aria-disabled:opacity-50 [&_svg]:size-5.5";
</script>

<Field label={t("po.quantity")} helper={t("po.quantityHelp", { max: MAX_QUANTITY })} {error}>
  {#snippet children({ id })}
    <div class="flex items-center gap-2">
      <IconButton
        aria-label={t("po.remove")}
        aria-controls={id}
        aria-disabled={value == null || value <= 1}
        variant="ghost"
        class={STEP}
        onclick={() => (value = stepQuantity(value, -1) ?? undefined)}
      >
        <Icon icon="solar:minus-square-bold-duotone" size={22} />
      </IconButton>
      <Input
        {id}
        class="w-20 bg-fc-surface text-center font-mono"
        type="number"
        inputmode="numeric"
        min="1"
        max={MAX_QUANTITY}
        step="1"
        bind:value
      />
      <IconButton
        aria-label={t("po.add")}
        aria-controls={id}
        aria-disabled={value != null && value >= MAX_QUANTITY}
        variant="ghost"
        class={STEP}
        onclick={() => (value = stepQuantity(value, 1) ?? undefined)}
      >
        <Icon icon="solar:add-square-bold-duotone" size={22} />
      </IconButton>
    </div>
  {/snippet}
</Field>
