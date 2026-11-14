import { version } from "../../package.json";

export interface FeatureItem {
  icon: string;
  title: string;
  description: string;
}

export const VERSION: string = version;

export const RELEASE_TAG = `v${VERSION}`;

export const NAV_SECTIONS = [
  { id: "disassembly", label: "Inside" },
  { id: "features", label: "Hardware" },
  { id: "viewer", label: "Board" },
  { id: "preorder", label: "Pre-order" },
] as const;

export const BATTERY = {
  cell: "402020",
  capacity: "120 mAh",
  chargeCurrent: "about 67 mA",
};

export const FEATURES: FeatureItem[] = [
  {
    icon: "solar:soundwave-bold-duotone",
    title: "Tap to talk",
    description: "A mic and a speaker run at once, so you can cut Bulle off mid-sentence.",
  },
  {
    icon: "solar:eye-bold-duotone",
    title: "A round face",
    description: "A 1.28 inch screen shows Bulle listening, thinking and answering.",
  },
  {
    icon: "solar:battery-charge-bold-duotone",
    title: "Days on a charge",
    description: `About 3.5 days at 20 minutes of talk a day, from a ${BATTERY.capacity} cell (estimate).`,
  },
  {
    icon: "solar:shield-keyhole-bold-duotone",
    title: "Yours, end to end",
    description: "Open hardware and Rust firmware, talking only to the Bulle server you run.",
  },
];
