<script module lang="ts">
  export type Mood = "idle" | "thinking" | "buzz" | "showing";
</script>

<script lang="ts">
  import { Blob, idle, surprised, type Expression } from "@facile/blob";
  import { Roll } from "@facile/mise-en-scene";
  import { t } from "$lib/i18n/index.svelte";

  let { mood, size }: { mood: Mood; size: number } = $props();

  const pose = (p: Partial<Expression["p"]>): Expression => ({ ...idle, p: { ...idle.p, ...p } });
  const EXPRESSIONS: Record<Mood, Expression> = {
    idle,
    thinking: pose({ esx: 1.06, esy: 1.06, bdy: -0.6 }),
    buzz: surprised,
    showing: pose({ esx: 1.16, esy: 1.16, bdy: -1 }),
  };
</script>

<div class="cf" style="--cf-size:{size}px">
  <div class="cf-shell" class:cf-buzz={mood === "buzz"}>
    <div class="cf-screen" class:cf-still={mood === "thinking"}>
      <Blob name="Bulle" size={Math.round(size * 0.42)} animate="always" expression={EXPRESSIONS[mood]} />
    </div>
  </div>
  <div class="cf-status">
    <span class="cf-wave" class:cf-live={mood === "buzz" || mood === "thinking"}>
      <i></i><i></i><i></i><i></i>
    </span>
    <Roll value={t(`demo.mood.${mood}`)} class="cf-label" />
  </div>
</div>

<style>
  .cf {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: calc(var(--cf-size) * 0.09);
  }
  .cf-shell {
    display: grid;
    place-items: center;
    width: var(--cf-size);
    height: var(--cf-size);
    border-radius: 999px;
    background: var(--color-fc-component);
    box-shadow: var(--mes-shadow-hero, 0 30px 70px -20px rgb(0 0 0 / 0.22), 0 8px 24px -8px rgb(0 0 0 / 0.1));
  }
  .cf-buzz {
    animation: cf-shake 0.12s linear 5;
  }
  @keyframes cf-shake {
    25% {
      transform: translateX(-3px) rotate(-1.5deg);
    }
    75% {
      transform: translateX(3px) rotate(1.5deg);
    }
  }
  .cf-screen {
    display: grid;
    place-items: center;
    width: 76%;
    height: 76%;
    border-radius: 999px;
    background: var(--color-fc-inverse);
  }
  .cf-still :global(.mo-eyes) {
    animation: none;
  }
  .cf-status {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 6px 14px;
    border-radius: 999px;
    background: var(--color-fc-surface);
    font-family: var(--font-fc-body);
    font-size: 13px;
    color: var(--color-fc-fg);
  }
  :global(.cf-label) {
    width: 84px;
    justify-items: center;
  }
  .cf-wave {
    display: flex;
    align-items: center;
    gap: 2px;
    height: 14px;
  }
  .cf-wave i {
    width: 3px;
    height: 14px;
    transform: scaleY(0.3);
    border-radius: 2px;
    background: var(--color-fc-fg-muted);
  }
  .cf-live i {
    background: var(--color-fc-accent);
    animation: cf-bar 0.9s ease-in-out infinite;
  }
  .cf-live i:nth-child(2) {
    animation-delay: 0.15s;
  }
  .cf-live i:nth-child(3) {
    animation-delay: 0.3s;
  }
  .cf-live i:nth-child(4) {
    animation-delay: 0.45s;
  }
  @keyframes cf-bar {
    50% {
      transform: scaleY(1);
    }
  }
  @media (prefers-reduced-motion: reduce) {
    .cf-buzz {
      animation: none;
    }
    .cf-live i {
      animation: none;
      transform: scaleY(0.7);
    }
  }
</style>
