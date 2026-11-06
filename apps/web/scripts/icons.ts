import { Glob } from "bun";
import { join, relative } from "node:path";
import solar from "@iconify-json/solar/icons.json" with { type: "json" };
import { ICON_DATA } from "../node_modules/@facile/muse/src/lib/icons-data.ts";
import { ICONS } from "../src/lib/icons.ts";

type IconData = { body: string; width: number; height: number };
type SolarAlias = { parent: string; [key: string]: unknown };

const ROOT = join(import.meta.dir, "..");
const SRC = join(ROOT, "src");
const OUT = join(SRC, "lib", "icons.generated.json");
const NAME = /solar:[a-z0-9]+(?:-[a-z0-9]+)*/g;
const QUOTED = /["'`]([a-z][a-z0-9]*:[a-z0-9]+(?:-[a-z0-9]+)*)["'`]/g;
const ICON_PROP = /\bicon\s*[=:]\s*["'`]([a-z][a-z0-9-]*:[a-z0-9-]+)["'`]/g;
const KNOWN = new Set([...Object.keys(ICON_DATA), ...Object.keys(ICONS)].filter((n) => !n.startsWith("solar:")));
const PREFIXES = new Set([...KNOWN].map((n) => n.split(":")[0]));

/** Collects every `solar:<name>` string, and every other icon name, used under src. */
async function scanNames(): Promise<{ solarNames: string[]; others: string[] }> {
  const names = new Set<string>();
  const others = new Set<string>();
  for await (const file of new Glob("**/*.{ts,js,svelte}").scan(SRC)) {
    const text = await Bun.file(join(SRC, file)).text();
    for (const match of text.matchAll(NAME)) names.add(match[0].slice("solar:".length));
    for (const [, name] of text.matchAll(QUOTED)) if (PREFIXES.has(name.split(":")[0])) others.add(name);
    for (const [, name] of text.matchAll(ICON_PROP)) if (!name.startsWith("solar:")) others.add(name);
  }
  return { solarNames: [...names].sort(), others: [...others].sort() };
}

/** Resolves a solar icon or plain alias to its data, or null when the set lacks it. */
function lookup(name: string): IconData | null {
  const icons = solar.icons as Record<string, Partial<IconData>>;
  const alias = (solar.aliases as Record<string, SolarAlias>)[name];
  const isPlainAlias = alias && Object.keys(alias).length === 1;
  const icon = icons[name] ?? (isPlainAlias ? icons[alias.parent] : undefined);
  if (!icon?.body) return null;
  return { body: icon.body, width: icon.width ?? solar.width, height: icon.height ?? solar.height };
}

/** Builds the registry text for the given names and lists any name the solar set lacks. */
function render(names: string[]): { text: string; missing: string[] } {
  const registry: Record<string, IconData> = {};
  const missing: string[] = [];
  for (const name of names) {
    const data = lookup(name);
    if (data) registry[`solar:${name}`] = data;
    else missing.push(`solar:${name}`);
  }
  return { text: `${JSON.stringify(registry, null, 2)}\n`, missing };
}

/** Regenerates the registry, or with --check fails when a name is unknown or the file is stale. */
async function main(): Promise<void> {
  const { solarNames: names, others } = await scanNames();
  const { text, missing } = render(names);
  const target = relative(process.cwd(), OUT);
  const unregistered = others.filter((name) => !KNOWN.has(name));
  if (unregistered.length > 0) {
    console.error(`icons: not in muse ICON_DATA or src/lib/icons.ts: ${unregistered.join(", ")}`);
    process.exit(1);
  }
  if (missing.length > 0) {
    console.error(`icons: not in @iconify-json/solar: ${missing.join(", ")}`);
    process.exit(1);
  }
  if (!process.argv.includes("--check")) {
    await Bun.write(OUT, text);
    console.log(`icons: wrote ${names.length} solar icons to ${target}`);
    return;
  }
  const current = (await Bun.file(OUT).exists()) ? await Bun.file(OUT).text() : "";
  if (current !== text) {
    console.error(`icons: ${target} is stale, run bun scripts/icons.ts`);
    process.exit(1);
  }
  console.log(`icons: ${names.length} solar icons up to date`);
}

await main();
