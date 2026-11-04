import { gsap } from "gsap";
import { prefersReducedMotion } from "@facile/muse";

export interface IntroElements {
  overlay: HTMLDivElement;
  ringOuter: HTMLDivElement;
  ringInner: HTMLDivElement;
  mark: HTMLDivElement;
  bar: HTMLDivElement;
}

let scrollLockUntil = 0;

/**
 * Populates the GSAP intro timeline with concentric pulse and curtain steps.
 */
function buildIntroTimeline(tl: gsap.core.Timeline, els: IntroElements) {
  const { overlay, ringOuter, ringInner, mark, bar } = els;
  const from = { ease: "power3.out" };
  tl.fromTo(mark, { scale: 0.72, opacity: 0 }, { scale: 1, opacity: 1, duration: 0.45, ...from }, 0);
  tl.fromTo(ringInner, { scale: 0.65, opacity: 0 }, { scale: 1.18, opacity: 0.4, duration: 0.65, ...from }, 0.08);
  tl.fromTo(ringOuter, { scale: 0.5, opacity: 0 }, { scale: 1.45, opacity: 0.18, duration: 0.85, ...from }, 0.14);
  tl.fromTo(bar, { scaleX: 0 }, { scaleX: 1, duration: 0.6, ease: "power3.inOut" }, 0.1);
  tl.to(
    [mark, ringInner, ringOuter, bar.parentElement],
    { y: -12, opacity: 0, duration: 0.3, ease: "power2.in" },
    0.8,
  );
  tl.to(overlay, { yPercent: -100, duration: 0.6, ease: "power3.inOut" }, 0.92);
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
      gsap.to(targets, {
        opacity: 1,
        y: 0,
        duration: 0.65,
        delay,
        stagger,
        ease: "power3.out",
        clearProps: "transform,opacity",
      });
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

/**
 * Slides the active navigation pill to match the target link's offset and width.
 */
export function moveNavPill(
  pill: HTMLElement | null,
  target: HTMLElement | undefined,
  animate = true,
) {
  if (!pill || !target) return;
  const to = { x: target.offsetLeft, width: target.offsetWidth, autoAlpha: 1 };
  if (!animate || prefersReducedMotion()) {
    gsap.set(pill, to);
    return;
  }
  gsap.to(pill, { ...to, duration: 0.32, ease: "power3.inOut", overwrite: "auto" });
}

/**
 * Smoothly scrolls the viewport to a section ID while locking scroll-spy updates.
 */
export function scrollToSection(id: string, event?: MouseEvent) {
  if (typeof window === "undefined") return;
  const target = document.getElementById(id);
  if (!target) return;
  event?.preventDefault();
  scrollLockUntil = performance.now() + 750;
  history.replaceState(null, "", `#${id}`);
  const top = target.getBoundingClientRect().top + window.scrollY - 76;
  window.scrollTo({
    top: Math.max(0, top),
    behavior: prefersReducedMotion() ? "auto" : "smooth",
  });
}

/**
 * Computes which section ID is currently active in the viewport.
 */
function detectActiveSection(ids: string[]): string {
  const maxScroll = Math.max(1, document.documentElement.scrollHeight - window.innerHeight);
  const progress = Math.min(1, Math.max(0, window.scrollY / maxScroll));
  const triggerLine = window.innerHeight * (0.24 + progress * 0.56);
  let current = ids[0] ?? "";
  for (const id of ids) {
    const el = document.getElementById(id);
    if (el && el.getBoundingClientRect().top <= triggerLine) {
      current = id;
    }
  }
  return current;
}

/**
 * Subscribes to window scroll and resize events to sync active section and scroll state.
 */
export function watchActiveSection(
  ids: string[],
  onActive: (id: string) => void,
  onScrollState?: (scrolled: boolean) => void,
) {
  if (typeof window === "undefined") return () => {};
  const onScroll = () => {
    onScrollState?.(window.scrollY > 16);
    if (performance.now() < scrollLockUntil) return;
    onActive(detectActiveSection(ids));
  };
  onScroll();
  window.addEventListener("scroll", onScroll, { passive: true });
  window.addEventListener("resize", onScroll, { passive: true });
  return () => {
    window.removeEventListener("scroll", onScroll);
    window.removeEventListener("resize", onScroll);
  };
}
