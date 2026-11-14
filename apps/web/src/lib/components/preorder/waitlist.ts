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

/** What a join can end in, one value per message the form shows. */
export type JoinResult = "ok" | "invalid_email" | "consent_required" | "rate_limited" | "failed";

interface Challenge {
  algorithm: string;
  challenge: string;
  maxNumber: number;
  salt: string;
  signature: string;
}

const REFUSALS: Record<string, JoinResult> = { invalid_email: "invalid_email", consent_required: "consent_required" };

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
 * Puts an address on the Pastille list and answers which message the form should show.
 */
export async function joinWaitlist(email: string, locale: string): Promise<JoinResult> {
  try {
    const challenge = await fetch(`${WAITLIST_API}/challenge`, { cache: "no-store" });
    if (!challenge.ok) return refusal(challenge);
    const altcha = await solve((await challenge.json()) as Challenge);
    const response = await fetch(WAITLIST_API, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ email: email.trim(), product: "pastille", locale, consent: CONSENT_VERSION, altcha }),
    });
    return response.ok ? "ok" : refusal(response);
  } catch {
    return "failed";
  }
}
