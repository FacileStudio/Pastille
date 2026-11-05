<script lang="ts">
  import { ICONS } from "$lib/icons";

  let { name, x, y, size = 24, color = "currentColor" }: {
    name: string;
    x: number;
    y: number;
    size?: number;
    color?: string;
  } = $props();

  const body = $derived(ICONS[name]?.body ?? "");
</script>

<g transform="translate({x - size / 2} {y - size / 2}) scale({size / 24})" style:color aria-hidden="true">
  {@html body}
</g>
