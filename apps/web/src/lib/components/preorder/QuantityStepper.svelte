<script lang="ts">
  import { Field, Icon, IconButton, Input } from "@facile/muse";
  import { t } from "$lib/i18n/index.svelte";
  import { PREORDER, clampQuantity } from "./order";

  interface Props {
    value: number;
    error?: string;
    onblur?: () => void;
  }

  let { value = $bindable(), error, onblur }: Props = $props();

  const STEP = "rounded-fc-md bg-fc-surface text-fc-fg aria-disabled:opacity-50 [&_svg]:size-5.5";

  /**
   * Moves the quantity by one step, staying inside the allowed range.
   */
  function step(delta: number) {
    value = clampQuantity((Number.isFinite(value) ? value : 0) + delta);
    onblur?.();
  }
</script>

<Field label={t("po.quantity")} helper={t("po.quantityHelp", { max: PREORDER.maxQuantity })} {error}>
  {#snippet children({ id })}
    <div class="flex items-center gap-2">
      <IconButton
        aria-label={t("po.remove")}
        aria-controls={id}
        aria-disabled={value <= 1}
        variant="ghost"
        class={STEP}
        onclick={() => step(-1)}
      >
        <Icon icon="solar:minus-square-bold-duotone" size={22} />
      </IconButton>
      <Input
        {id}
        class="w-20 bg-fc-surface text-center font-mono"
        type="number"
        inputmode="numeric"
        min="1"
        max={PREORDER.maxQuantity}
        step="1"
        bind:value
        {onblur}
      />
      <IconButton
        aria-label={t("po.add")}
        aria-controls={id}
        aria-disabled={value >= PREORDER.maxQuantity}
        variant="ghost"
        class={STEP}
        onclick={() => step(1)}
      >
        <Icon icon="solar:add-square-bold-duotone" size={22} />
      </IconButton>
    </div>
  {/snippet}
</Field>
