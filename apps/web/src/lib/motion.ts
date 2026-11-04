import { gsap } from "gsap";
import { prefersReducedMotion } from "@facile/muse";

export interface IntroElements {
  overlay: HTMLDivElement;
  ringOuter: HTMLDivElement;
  ringInner: HTMLDivElement;
  mark: HTMLDivElement;
  caption: HTMLDivElement;
  bar: HTMLDivElement;
}

/**
 * Populates the GSAP intro timeline with concentric pulse and curtain steps.
 */
function buildIntroTimeline(tl: gsap.core.Timeline, els: IntroElements) {
  const { overlay, ringOuter, ringInner, mark, caption, bar } = els;
  const from = { ease: "power3.out" };
  tl.fromTo(mark, { scale: 0.72, opacity: 0 }, { scale: 1, opacity: 1, duration: 0.45, ...from }, 0);
  tl.fromTo(ringInner, { scale: 0.65, opacity: 0 }, { scale: 1.18, opacity: 0.4, duration: 0.65, ...from }, 0.08);
  tl.fromTo(ringOuter, { scale: 0.5, opacity: 0 }, { scale: 1.45, opacity: 0.18, duration: 0.85, ...from }, 0.14);
  tl.fromTo(caption, { y: 10, opacity: 0 }, { y: 0, opacity: 1, duration: 0.4, ...from }, 0.18);
  tl.fromTo(bar, { scaleX: 0 }, { scaleX: 1, duration: 0.65, ease: "power3.inOut" }, 0.12);
  tl.to(
    [mark, ringInner, ringOuter, caption, bar.parentElement],
    { y: -14, opacity: 0, duration: 0.32, ease: "power2.in" },
    0.88,
  );
  tl.to(overlay, { yPercent: -100, duration: 0.65, ease: "power3.inOut" }, 1.02);
}

/**
 * Runs the entrance curtain and concentric pulse choreography.
 */
export function runIntroSequence(els: IntroElements, onDone: () => void) {
  if (prefersReducedMotion()) {
    onDone();
    return () => {};
  }
  const ctx = gsap.context(() => {
    const tl = gsap.timeline({ onComplete: onDone });
    buildIntroTimeline(tl, els);
  }, els.overlay);
  return () => ctx.revert();
}

/**
 * Tweens reveal targets into place and clears inline transforms when finished.
 */
function animateRevealTargets(
  targets: HTMLElement[],
  delay: number,
  stagger: number,
) {
  gsap.to(targets, {
    opacity: 1,
    y: 0,
    duration: 0.65,
    delay,
    stagger,
    ease: "power3.out",
    onComplete: () => {
      gsap.set(targets, { clearProps: "transform,opacity" });
    },
  });
}

/**
 * Svelte action that reveals an element and its `[data-reveal-item]` children when scrolled into view.
 */
export function scrollReveal(
  node: HTMLElement,
  opts: { delay?: number; stagger?: number; y?: number } = {},
) {
  if (typeof window === "undefined" || prefersReducedMotion()) return {};
  const delay = opts.delay ?? 0;
  const stagger = opts.stagger ?? 0.07;
  const items = Array.from(node.querySelectorAll<HTMLElement>("[data-reveal-item]"));
  const targets = items.length > 0 ? items : [node];
  gsap.set(targets, { opacity: 0, y: opts.y ?? 20 });

  const observer = new IntersectionObserver(
    (entries) => {
      if (!entries.some((entry) => entry.isIntersecting)) return;
      observer.disconnect();
      animateRevealTargets(targets, delay, stagger);
    },
    { threshold: 0.1, rootMargin: "0px 0px -40px 0px" },
  );
  observer.observe(node);

  return {
    destroy() {
      observer.disconnect();
      gsap.killTweensOf(targets);
    },
  };
}
