import { BATTERY, RELEASE_TAG } from "$lib/data";

export const PREORDER = {
  unitPrice: 49,
  currency: "USD",
  maxQuantity: 20,
  priceLabel: "Pre-order price",
  shipLabel: "Estimated ship date",
  shipDate: "March 2027",
  contact: "contact@facile.studio",
  paymentNote: "You pay nothing now. We send payment details by email later.",
  shippingNote: "Shipping is not included. We confirm its cost by email.",
  privacyNote: "We use your email only to handle this reservation.",
  includes: [
    { icon: "solar:box-bold-duotone", label: "One assembled Pastille per unit" },
    { icon: "solar:battery-charge-bold-duotone", label: `${BATTERY.capacity} cell, charged over USB-C` },
    { icon: "solar:code-square-bold-duotone", label: "Open hardware and firmware files" },
  ],
} as const;

export interface Order {
  name: string;
  email: string;
  quantity: number;
  message: string;
}

export type OrderField = "name" | "email" | "quantity";

export type OrderErrors = Partial<Record<OrderField, string>>;

export const ORDER_FIELDS: OrderField[] = ["name", "email", "quantity"];

const EMAIL = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;

const money = new Intl.NumberFormat("en-US", {
  style: "currency",
  currency: PREORDER.currency,
  maximumFractionDigits: 0,
});

/**
 * Formats a whole amount in the pre-order currency, for example `$49`.
 */
export function formatPrice(amount: number): string {
  return money.format(amount);
}

/**
 * Rounds a quantity and keeps it between 1 and the pre-order maximum.
 */
export function clampQuantity(value: number): number {
  const whole = Number.isFinite(value) ? Math.round(value) : 1;
  return Math.min(PREORDER.maxQuantity, Math.max(1, whole));
}

/**
 * Returns one plain-language message per invalid field. An empty object means the order is valid.
 */
export function validateOrder(order: Order): OrderErrors {
  const errors: OrderErrors = {};
  const email = order.email.trim();
  if (!order.name.trim()) errors.name = "Enter your name.";
  if (!email) errors.email = "Enter your email address.";
  else if (!EMAIL.test(email)) errors.email = "Enter an email address like name@example.com.";
  if (!Number.isInteger(order.quantity) || order.quantity !== clampQuantity(order.quantity)) {
    errors.quantity = `Choose a whole number from 1 to ${PREORDER.maxQuantity}.`;
  }
  return errors;
}

/**
 * Builds the plain-text order summary used in the email body and the copy button.
 */
export function orderText(order: Order): string {
  const lines = [
    `Pastille pre-order (${RELEASE_TAG})`,
    `Name: ${order.name.trim()}`,
    `Email: ${order.email.trim()}`,
    `Quantity: ${order.quantity}`,
    `Total before shipping: ${formatPrice(order.quantity * PREORDER.unitPrice)}`,
    `${PREORDER.shipLabel}: ${PREORDER.shipDate}`,
  ];
  const message = order.message.trim();
  return message ? [...lines, "", message].join("\n") : lines.join("\n");
}

/**
 * Builds the `mailto:` link that opens the visitor's mail app with the order filled in.
 */
export function orderMailto(order: Order): string {
  const subject = encodeURIComponent(`Pastille pre-order (${RELEASE_TAG})`);
  return `mailto:${PREORDER.contact}?subject=${subject}&body=${encodeURIComponent(orderText(order))}`;
}
