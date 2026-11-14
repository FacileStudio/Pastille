<script lang="ts">
  import { Stage, Window, type Act } from "@facile/mise-en-scene";
  import { t } from "$lib/i18n/index.svelte";
  import CharmFace, { type Mood } from "./CharmFace.svelte";
  import DemoThread, { type Step } from "./DemoThread.svelte";

  let mood = $state<Mood>("idle");
  let heard = $state("");
  let tools = $state<Step[]>(["hidden", "hidden"]);
  let words = $state(0);

  const answerWords = () => t("demo.answer").split(" ").length;

  /**
   * Puts the scene back on its resting frame: the earlier exchange only, the charm idle.
   */
  function reset() {
    mood = "idle";
    heard = "";
    tools = ["hidden", "hidden"];
    words = 0;
  }

  /**
   * Shows the finished exchange, for reduced motion and the first paint before the loop starts.
   */
  function poster() {
    mood = "showing";
    heard = t("demo.question");
    tools = ["ok", "ok"];
    words = answerWords();
  }

  /**
   * Runs one tool row from pending to done.
   */
  async function runTool(act: Act, i: number) {
    tools[i] = "running";
    await act.wait(700);
    tools[i] = "ok";
  }

  /**
   * One loop: the question is typed in Bulle, Bulle checks Opus, buzzes the charm and answers.
   */
  async function script(act: Act) {
    await act.wait(900);
    await act.type(t("demo.question"), (v) => (heard = v), 42);
    await act.wait(400);
    mood = "thinking";
    await runTool(act, 0);
    await act.wait(300);
    mood = "buzz";
    await runTool(act, 1);
    mood = "showing";
    await act.count(answerWords(), (v) => (words = v), 120);
    await act.wait(2600);
    reset();
    await act.wait(900);
  }
</script>

<Stage
  width={1120}
  height={480}
  compact={{ below: 640, width: 358, height: 600 }}
  {script}
  {reset}
  {poster}
  label={t("demo.label")}
>
  {#snippet children({ compact })}
    <div class="hd-charm" class:hd-compact={compact}>
      <CharmFace {mood} size={compact ? 150 : 300} />
    </div>
    <Window title="Bulle" class={compact ? "hd-win hd-win-compact" : "hd-win"}>
      <DemoThread {heard} {tools} {words} />
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
</style>
