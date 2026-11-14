<script lang="ts">
  import { fade } from "svelte/transition";
  import { Icon } from "@facile/muse";
  import { Bar, Stage, Stream, Window, grow, type Act } from "@facile/mise-en-scene";
  import { Blob } from "@facile/blob";
  import CharmFace, { type Mood } from "./CharmFace.svelte";

  type Step = "hidden" | "running" | "ok";

  const QUESTION = "What's left on my list for today?";
  const ANSWER = "Two things: call the plumber, and send Léa the slides before six.";
  const TOOLS = [
    { name: "charm.display_avatar", args: "thinking" },
    { name: "charm.haptic_pulse", args: "short" },
  ];
  const WORDS = ANSWER.split(" ").length;

  let mood = $state<Mood>("idle");
  let tapped = $state(false);
  let heard = $state("");
  let tools = $state<Step[]>(["hidden", "hidden"]);
  let words = $state(0);

  function reset() {
    mood = "idle";
    tapped = false;
    heard = "";
    tools = ["hidden", "hidden"];
    words = 0;
  }

  function poster() {
    mood = "speaking";
    heard = QUESTION;
    tools = ["ok", "ok"];
    words = WORDS;
  }

  async function runTool(act: Act, i: number) {
    tools[i] = "running";
    await act.wait(700);
    tools[i] = "ok";
  }

  async function script(act: Act) {
    await act.wait(900);
    tapped = true;
    await act.wait(180);
    tapped = false;
    mood = "listening";
    await act.wait(300);
    await act.type(QUESTION, (v) => (heard = v), 42);
    await act.wait(400);
    mood = "thinking";
    await runTool(act, 0);
    await act.wait(500);
    mood = "speaking";
    await act.count(WORDS, (v) => (words = v), 120);
    await runTool(act, 1);
    await act.wait(2200);
    reset();
    await act.wait(700);
  }
</script>

<Stage
  width={1120}
  height={480}
  compact={{ below: 640, width: 358, height: 600 }}
  {script}
  {reset}
  {poster}
  label="Pastille hears a question, Bulle answers out loud and the charm shows its face."
>
  {#snippet children({ compact })}
    <div class="hd-charm" class:hd-compact={compact}>
      <CharmFace {mood} {tapped} size={compact ? 150 : 300} />
    </div>
    <Window title="Bulle" class={compact ? "hd-win hd-win-compact" : "hd-win"}>
      <div class="hd-thread">
        <div class="hd-past">
          <span class="hd-bubble-bar hd-short"></span>
          <Bar w="64%" />
          <span class="hd-bubble-bar"></span>
          <Bar w="78%" />
          <Bar w="54%" />
        </div>
        {#if heard}
          <div class="hd-q" in:grow out:fade={{ duration: 200, delay: 160 }}>
            <span class="hd-via"><Icon icon="solar:microphone-3-bold-duotone" size={13} />Pastille</span>
            <p>{heard}</p>
          </div>
        {/if}
        {#if tools[0] !== "hidden"}
          <div class="hd-reply" in:grow out:fade={{ duration: 200, delay: 100 }}>
            <div class="hd-who"><Blob name="Bulle" size={22} /><b>Bulle</b></div>
            <ul class="hd-tools">
              {#each TOOLS as tool, i (tool.name)}
                {#if tools[i] !== "hidden"}
                  <li in:grow={{ duration: 300 }} data-state={tools[i]}>
                    <span class="hd-mark"></span><code>{tool.name}</code><span>{tool.args}</span>
                  </li>
                {/if}
              {/each}
            </ul>
            {#if words > 0}
              <Stream class="hd-answer" text={ANSWER} count={words} />
            {/if}
          </div>
        {/if}
      </div>
    </Window>
  {/snippet}
</Stage>

<style>
  .hd-charm {
    position: absolute;
    top: 48px;
    left: 40px;
  }
  .hd-charm.hd-compact {
    top: 12px;
    left: 104px;
  }
  :global(.hd-win) {
    top: 16px;
    left: 440px;
    width: 640px;
    height: 432px;
  }
  :global(.hd-win.hd-win-compact) {
    top: 232px;
    left: 16px;
    width: 326px;
    height: 352px;
  }
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
  .hd-past {
    display: grid;
    gap: 8px;
    padding-bottom: 8px;
  }
  .hd-bubble-bar {
    justify-self: end;
    width: 40%;
    height: 32px;
    border-radius: 12px;
    background: var(--color-fc-surface);
  }
  .hd-short {
    width: 28%;
  }
  .hd-q {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 6px;
    padding-top: 16px;
  }
  .hd-via {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font-size: 12px;
    color: var(--color-fc-fg-muted);
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
  .hd-tools code {
    font-family: var(--font-fc-mono);
    color: var(--color-fc-fg);
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
