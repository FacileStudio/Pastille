/**
 * Joining the Pastille pre-order list kept by Registre at sso.facile.studio.
 *
 * The form fetches an ALTCHA proof-of-work challenge from Registre, solves it in the browser with Web
 * Crypto, then posts the address. The solver is a copy of Registre's apps/client/src/lib/altcha.ts, on
 * purpose: the landing pulls in no package for twenty lines.
 */

export const WAITLIST_API = "https://sso.facile.studio/api/waitlist";

/** The version of the consent line shown next to the checkbox. Bump it when that text changes. */
export const CONSENT_VERSION = "2026-11-14";

/** The most charms one pre-order may ask for; Registre refuses more. */
export const MAX_QUANTITY = 10;

/** What a join can end in, one value per message the form shows. */
export type JoinResult =
  | "ok"
  | "invalid_email"
  | "invalid_quantity"
  | "consent_required"
  | "rate_limited"
  | "failed";

interface Challenge {
  algorithm: string;
  challenge: string;
  maxNumber: number;
  salt: string;
  signature: string;
}

const REFUSALS: Record<string, JoinResult> = {
  invalid_email: "invalid_email",
  invalid_quantity: "invalid_quantity",
  consent_required: "consent_required",
};

/**
 * Whether a quantity is unset, or a whole number from 1 to MAX_QUANTITY.
 */
export function quantityOk(value: number | null | undefined): boolean {
  return value == null || (Number.isInteger(value) && value >= 1 && value <= MAX_QUANTITY);
}

/**
 * Turns what the number field holds into a quantity; an empty field is unset.
 */
export function readQuantity(typed: string | number | null | undefined): number | undefined {
  if (typed == null || typed === "") return undefined;
  const value = Number(typed);
  return Number.isNaN(value) ? undefined : value;
}

/**
 * Moves a quantity by one step inside 1 to MAX_QUANTITY; stepping up from unset gives 1.
 */
export function stepQuantity(value: number | null | undefined, delta: number): number | null | undefined {
  if (value == null || !Number.isFinite(value)) return delta > 0 ? 1 : value;
  return Math.min(MAX_QUANTITY, Math.max(1, Math.round(value) + delta));
}

/**
 * The SHA-256 of a string, as lowercase hex.
 */
async function hex(text: string): Promise<string> {
  const digest = await crypto.subtle.digest("SHA-256", new TextEncoder().encode(text));
  return Array.from(new Uint8Array(digest), (byte) => byte.toString(16).padStart(2, "0")).join("");
}

/**
 * Finds the challenge's secret number and returns the base64 payload Registre verifies.
 */
async function solve(challenge: Challenge): Promise<string> {
  for (let number = 0; number <= challenge.maxNumber; number++) {
    if ((await hex(challenge.salt + number)) === challenge.challenge) {
      const { algorithm, salt, signature } = challenge;
      return btoa(JSON.stringify({ algorithm, challenge: challenge.challenge, number, salt, signature }));
    }
  }
  throw new Error("altcha: no solution");
}

/**
 * Maps a refused join to the message the form shows.
 */
async function refusal(response: Response): Promise<JoinResult> {
  if (response.status === 429) return "rate_limited";
  const body = (await response.json().catch(() => ({}))) as { error?: string };
  return REFUSALS[body.error ?? ""] ?? "failed";
}

/**
 * Puts an address on the Pastille list, with the quantity only when one is set, and answers which
 * message the form should show.
 */
export async function joinWaitlist(
  email: string,
  locale: string,
  quantity: number | null | undefined,
): Promise<JoinResult> {
  try {
    const challenge = await fetch(`${WAITLIST_API}/challenge`, { cache: "no-store" });
    if (!challenge.ok) return refusal(challenge);
    const altcha = await solve((await challenge.json()) as Challenge);
    const response = await fetch(WAITLIST_API, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        email: email.trim(),
        product: "pastille",
        locale,
        consent: CONSENT_VERSION,
        altcha,
        ...(quantity == null ? {} : { quantity }),
      }),
    });
    return response.ok ? "ok" : refusal(response);
  } catch {
    return "failed";
  }
}
