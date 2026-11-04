import { gsap } from "gsap";
import type { ScrollTrigger as Trigger } from "gsap/ScrollTrigger";

let plugin: Promise<typeof Trigger> | null = null;

export interface StepWatch {
  count: number;
  scrub: boolean;
  onProgress: (t: number) => void;
  onActive: (index: number) => void;
}

/**
 * Imports ScrollTrigger on first use and registers it with GSAP exactly once.
 */
function scrollTrigger() {
  plugin ??= import("gsap/ScrollTrigger").then(({ ScrollTrigger }) => {
    gsap.registerPlugin(ScrollTrigger);
    return ScrollTrigger;
  });
  return plugin;
}

/**
 * Viewport line a step block must cross to become active: the centre on desktop, and the middle of the
 * area left below the sticky canvas on mobile.
 */
const readingLine = () => (window.matchMedia("(min-width: 1024px)").matches ? "center" : "75%");

/**
 * Calls `onNear` once when `node` comes within one viewport of the visible area.
 */
export function whenNear(node: Element, onNear: () => void) {
  const observer = new IntersectionObserver(
    (entries) => {
      if (!entries.some((entry) => entry.isIntersecting)) return;
      observer.disconnect();
      onNear();
    },
    { rootMargin: "100% 0px 100% 0px" },
  );
  observer.observe(node);
  return () => observer.disconnect();
}

/**
 * Maps the scroll position over a step list to a 0..1 progress, where step `i` sits at the moment
 * its block's centre crosses the reading line. Returns a teardown that reverts every trigger.
 */
export async function watchSteps(list: HTMLElement, opts: StepWatch) {
  const ScrollTrigger = await scrollTrigger();
  const half = (el: Element | null | undefined) => ((el as HTMLElement | null)?.offsetHeight ?? 0) / 2;
  const last = Math.max(1, opts.count - 1);
  const trigger = {
    trigger: list,
    start: () => `top+=${half(list.firstElementChild)} ${readingLine()}`,
    end: () => `bottom-=${half(list.lastElementChild)} ${readingLine()}`,
    invalidateOnRefresh: true,
    onUpdate: (self: Trigger) => opts.onActive(Math.round(self.progress * last)),
  };
  const proxy = { t: 0 };
  const ctx = gsap.context(() => {
    if (!opts.scrub) {
      ScrollTrigger.create(trigger);
      return;
    }
    gsap.to(proxy, {
      t: 1,
      ease: "none",
      onUpdate: () => opts.onProgress(proxy.t),
      scrollTrigger: { ...trigger, scrub: true },
    });
  });
  return () => ctx.revert();
}
