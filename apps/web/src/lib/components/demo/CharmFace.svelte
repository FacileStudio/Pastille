<script module lang="ts">
  export type Mood = "idle" | "listening" | "thinking" | "speaking";
</script>

<script lang="ts">
  import { Blob, happy, idle, surprised, thinking } from "@facile/blob";
  import { Roll } from "@facile/mise-en-scene";

  let { mood, tapped, size }: { mood: Mood; tapped: boolean; size: number } = $props();

  const EXPRESSIONS = { idle, listening: surprised, thinking, speaking: happy };
  const LABELS: Record<Mood, string> = {
    idle: "Tap to talk",
    listening: "Listening",
    thinking: "Thinking",
    speaking: "Speaking",
  };
</script>

<div class="cf" style="--cf-size:{size}px">
  <div class="cf-shell" class:cf-tapped={tapped}>
    <div class="cf-screen">
      <Blob name="Bulle" size={Math.round(size * 0.42)} animate="always" expression={EXPRESSIONS[mood]} />
    </div>
  </div>
  <div class="cf-status">
    <span class="cf-wave" class:cf-live={mood === "listening" || mood === "speaking"}>
      <i></i><i></i><i></i><i></i>
    </span>
    <Roll value={LABELS[mood]} class="cf-label" />
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
    transition: scale 0.18s ease;
  }
  .cf-tapped {
    scale: 0.96;
  }
  .cf-screen {
    display: grid;
    place-items: center;
    width: 76%;
    height: 76%;
    border-radius: 999px;
    background: var(--color-fc-inverse);
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
  }
  .cf-wave {
    display: flex;
    align-items: center;
    gap: 2px;
    height: 14px;
  }
  .cf-wave i {
    width: 3px;
    height: 4px;
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
      height: 14px;
    }
  }
  @media (prefers-reduced-motion: reduce) {
    .cf-live i {
      animation: none;
      height: 10px;
    }
  }
</style>
