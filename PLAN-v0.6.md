# Pastille v0.6.0: review Gemini's releases, fix icons, redesign the pre-order form

## Goal

Make the landing page and release pipeline clean and robust after Gemini's v0.4.0 to v0.5.2 releases, then ship v0.6.0.

## Why

1. **Icons fail silently.** Icons are registered by hand in `icons.ts`, `icons-solar.ts`, `icons-explode.ts` and `topology-icons.ts`. A name that is used but never registered renders an empty `<svg>` with no error. On the live site the nav items Board (`solar:cpu-linear`), Connectivity (`solar:bluetooth-linear`) and Build (`solar:box-linear`) show no icon. This is direct observation, from a headless check of https://pastille.facile.studio/ on 2026-11-05.
2. **The nav uses linear icons.** The user asked for solar bold duotone icons on the page.
3. **The pre-order form needs a redesign.** `PreOrderForm.svelte` builds a `mailto:` link. It has no validation feedback and no success state, and it offers nothing if the visitor has no mail client. The layout is also plain.
4. **Gemini's changes were never reviewed.** That covers the range `329780b..HEAD`: 4 releases plus 1 commit after v0.5.2. Known issues:
   - `.github/workflows/ci.yml` now runs `git diff -I "CARTESIAN_POINT"`, so the stale-bodies check ignores every geometry line.
   - `hardware/bodies/build.ts` rewrites one specific float literal by hand.

## Tracks (parallel)

### Track A: review (fresh context, read-only)

Run `/review` over `git diff 329780b..HEAD` and the current state of every file it touches. Write the findings to `scratchpad/review-gemini.md`. This track changes no files.

### Track B: icons (`apps/web/src/lib/icons*.ts`, `routes/+layout.svelte`, `scripts/`)

1. Build one icon registry from `@iconify-json/solar`, added as a dev dependency, with names pinned to bold duotone wherever a duotone variant exists. Bundle only the icons the page uses, generated at build time or with a script, not pasted by hand.
2. Give every nav item, the header buttons and the mobile drawer a bold duotone icon.
3. Add a check to `bun run check` that scans `src/` for every `solar:` name and fails the build if any name is missing from the registry. It also fails if a name is not in the solar set at all.
4. Delete the hand-pasted icon files the registry replaces.

### Track C: pre-order form (`components/PreOrderForm.svelte`, the pre-order block in `data.ts`)

1. **Layout:** a clean two-column layout. On the left, a product summary with the price, what is included, the ship date and a muse-style trust note. On the right, the form. Use muse components and tokens only. Hit targets are at least 44 px.
2. **Validation:** inline errors after a field is touched, `aria-invalid` and `aria-describedby`, and a quantity stepper that stays within 1 to 20.
3. **After submit:** a success panel that confirms what was prepared. It includes a "copy order details" button and the plain email address, as a fallback when no mail client opens.
4. **No new service:** the form keeps the `mailto:` flow, because the site is static. Every claim (price, ship date) comes from one constant in `data.ts`.

### Track D: fix review findings

Fixers act on Track A's output, split by area so that no two agents edit the same file.

## Exit criteria

1. The headless check finds 0 empty icons on desktop and mobile, and the drawer is open during the mobile check.
2. `mise run check` passes. The icon check is part of it.
3. CI's stale-bodies check catches real geometry changes again, and its output is still deterministic.
4. `/ship` v0.6.0. CI and the release workflow are green, and the live site shows the fixes.
