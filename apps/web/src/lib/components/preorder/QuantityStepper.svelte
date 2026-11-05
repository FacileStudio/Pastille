<script lang="ts">
  import { Field, Icon, IconButton, Input } from "@facile/muse";
  import { PREORDER } from "$lib/data";
  import { clampQuantity, formatPrice } from "./order";

  interface Props {
    value: number;
    error?: string;
    onblur?: () => void;
  }

  let { value = $bindable(), error, onblur }: Props = $props();

  const STEP = "rounded-fc-md bg-fc-surface text-fc-fg aria-disabled:opacity-50 [&_svg]:size-5.5";

  const total = $derived(Number.isInteger(value) ? formatPrice(clampQuantity(value) * PREORDER.unitPrice) : "");

  /**
   * Moves the quantity by one step, staying inside the allowed range.
   */
  function step(delta: number) {
    value = clampQuantity((Number.isFinite(value) ? value : 0) + delta);
    onblur?.();
  }
</script>

<Field label="Quantity" helper="Up to {PREORDER.maxQuantity} per order." {error}>
  {#snippet children({ id })}
    <div class="flex items-center gap-2">
      <IconButton
        aria-label="Remove one"
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
        class="w-20 border-0 bg-fc-surface text-center font-mono"
        type="number"
        inputmode="numeric"
        min="1"
        max={PREORDER.maxQuantity}
        step="1"
        bind:value
        {onblur}
      />
      <IconButton
        aria-label="Add one"
        aria-controls={id}
        aria-disabled={value >= PREORDER.maxQuantity}
        variant="ghost"
        class={STEP}
        onclick={() => step(1)}
      >
        <Icon icon="solar:add-square-bold-duotone" size={22} />
      </IconButton>
      <p class="ml-auto text-right text-fc-sm text-fc-fg-muted" aria-live="polite">
        Total <span class="font-mono font-semibold text-fc-fg">{total}</span>
      </p>
    </div>
  {/snippet}
</Field>
