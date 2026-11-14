import { browser } from "$app/environment";
import { createTranslator, LOCALE_COOKIE, resolveLocale, type Locale } from "@facile/muse";
import { catalogs } from "./messages";

let active: () => Locale = () => "en";

/**
 * The visitor's locale: the cookie a language switch left, then the browser language.
 */
export function visitorLocale(): string | null {
  if (!browser) return null;
  const prefix = `${LOCALE_COOKIE}=`;
  const found = document.cookie.split("; ").find((part) => part.startsWith(prefix));
  return found ? decodeURIComponent(found.slice(prefix.length)) : navigator.language;
}

/**
 * Keeps a locale picked in the footer switch for the next visit.
 */
export function rememberLocale(locale: Locale) {
  document.cookie = `${LOCALE_COOKIE}=${locale}; path=/; max-age=31536000; samesite=lax`;
}

/**
 * Points the translator at the locale `provideLocale` resolved.
 */
export function bindLocale(source: () => Locale) {
  active = source;
}

/**
 * The locale currently in force.
 */
export function locale(): Locale {
  return resolveLocale(active());
}

/** Translates a page message in the current locale. */
export const t = createTranslator(catalogs, () => active());
