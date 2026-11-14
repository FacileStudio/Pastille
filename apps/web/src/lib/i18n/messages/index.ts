import type { Catalogs, Message } from "@facile/muse";
import { demo } from "./demo";
import { inside } from "./inside";
import { landing } from "./landing";
import { makers } from "./makers";
import { preorder } from "./preorder";
import { topology } from "./topology";
import type { Row } from "../row";

const rows = { ...landing, ...demo, ...makers, ...inside, ...topology, ...preorder };

export type AppKey = keyof typeof rows;
type AppCatalog = Record<AppKey, Message>;

/**
 * Picks one locale's column out of the four-locale rows.
 */
function column(index: number): AppCatalog {
  const entries = Object.entries(rows as Record<string, Row>).map(([key, row]) => [key, row[index]]);
  return Object.fromEntries(entries) as AppCatalog;
}

/** The page's catalogs: English is the typed base, and every row carries all four locales. */
export const catalogs: Catalogs<AppCatalog> = {
  en: column(0),
  fr: column(1),
  es: column(2),
  de: column(3),
};
