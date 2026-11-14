<script module lang="ts">
  export type Step = "hidden" | "running" | "ok";
</script>

<script lang="ts">
  import { fade } from "svelte/transition";
  import { Icon } from "@facile/muse";
  import { Stream, grow } from "@facile/mise-en-scene";
  import { Blob } from "@facile/blob";
  import { t } from "$lib/i18n/index.svelte";

  let { heard, tools, words }: { heard: string; tools: Step[]; words: number } = $props();

  const STEPS = [
    { icon: "solar:slider-vertical-minimalistic-bold-duotone", key: "demo.toolList" },
    { icon: "solar:smartphone-vibration-bold-duotone", key: "demo.toolBuzz" },
  ] as const;
</script>

<div class="hd-thread">
  <div class="hd-q"><p>{t("demo.pastQuestion")}</p></div>
  <div class="hd-reply">
    <div class="hd-who"><Blob name="Bulle" size={22} /><b>Bulle</b></div>
    <ul class="hd-tools">
      <li data-state="ok">
        <span class="hd-mark"></span><Icon icon="solar:battery-charge-bold-duotone" size={14} />
        <span>{t("demo.pastTool")}</span>
      </li>
    </ul>
    <p class="hd-answer">{t("demo.pastAnswer")}</p>
  </div>
  {#if heard}
    <div class="hd-q" in:grow out:fade={{ duration: 200, delay: 160 }}><p>{heard}</p></div>
  {/if}
  {#if tools[0] !== "hidden"}
    <div class="hd-reply" in:grow out:fade={{ duration: 200, delay: 100 }}>
      <div class="hd-who"><Blob name="Bulle" size={22} /><b>Bulle</b></div>
      <ul class="hd-tools">
        {#each STEPS as step, i (step.key)}
          {#if tools[i] !== "hidden"}
            <li in:grow={{ duration: 300 }} data-state={tools[i]}>
              <span class="hd-mark"></span><Icon icon={step.icon} size={14} /><span>{t(step.key)}</span>
            </li>
          {/if}
        {/each}
      </ul>
      {#if words > 0}
        <Stream class="hd-answer" text={t("demo.answer")} count={words} />
      {/if}
    </div>
  {/if}
</div>

<style>
  .hd-thread {
    display: flex;
    flex: 1;
    flex-direction: column;
    justify-content: flex-end;
    min-height: 0;
    overflow: hidden;
    padding: 0 18px 18px;
    font-family: var(--font-fc-body);
    font-size: 14px;
    color: var(--color-fc-fg);
    mask-image: linear-gradient(to bottom, transparent, black 48px);
  }
  .hd-q {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 6px;
    padding-top: 16px;
  }
  .hd-q p {
    max-width: 85%;
    margin: 0;
    padding: 8px 16px;
    border-radius: 12px;
    background: var(--color-fc-surface);
  }
  .hd-reply {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding-top: 16px;
  }
  .hd-who {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
  }
  .hd-tools {
    margin: 0;
    padding: 4px 0;
    list-style: none;
    border-radius: 12px;
    background: var(--color-fc-component);
  }
  .hd-tools li {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 7px 12px;
    font-size: 12px;
    color: var(--color-fc-fg-muted);
  }
  .hd-mark {
    width: 8px;
    height: 8px;
    border-radius: 999px;
    background: var(--color-fc-fg-muted);
    transition: background 0.2s ease;
  }
  [data-state="ok"] .hd-mark {
    background: var(--color-fc-success);
  }
  :global(.hd-answer) {
    margin: 0;
    line-height: 1.6;
  }
</style>
