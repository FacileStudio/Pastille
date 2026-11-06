import { PREORDER, clampQuantity, orderMailto, orderText, validateOrder } from "./order";

/**
 * Throws when `ok` is false, naming the failed expectation.
 */
function expect(ok: boolean, what: string) {
  if (!ok) throw new Error(`preorder: ${what}`);
}

const valid = { name: "Ada", email: "ada@example.com", quantity: 2, message: "" };

expect(Object.keys(validateOrder(valid)).length === 0, "a complete order is valid");
expect(validateOrder({ ...valid, name: "  " }).name !== undefined, "a blank name is rejected");
expect(validateOrder({ ...valid, email: "ada@" }).email !== undefined, "a malformed email is rejected");
expect(validateOrder({ ...valid, quantity: 0 }).quantity !== undefined, "zero units is rejected");
expect(validateOrder({ ...valid, quantity: PREORDER.maxQuantity + 1 }).quantity !== undefined, "the cap holds");
expect(validateOrder({ ...valid, quantity: 1.5 }).quantity !== undefined, "fractions are rejected");
expect(clampQuantity(99) === PREORDER.maxQuantity && clampQuantity(-3) === 1, "the stepper clamps");
expect(clampQuantity(Number.NaN) === 1, "an empty quantity falls back to one");
expect(orderText(valid).includes(`Total before shipping: $${2 * PREORDER.unitPrice}`), "the total is in the summary");
expect(orderMailto(valid).startsWith(`mailto:${PREORDER.contact}?subject=`), "the link targets the contact");

console.log("preorder: ok");
