import { version } from "../../package.json";

export const VERSION: string = version;

export const RELEASE_TAG = `v${VERSION}`;

export const REPO_URL = "https://github.com/FacileStudio/Pastille";

export const NAV_SECTIONS = [
  { id: "features", label: "nav.features" },
  { id: "pricing", label: "nav.pricing" },
  { id: "disassembly", label: "nav.inside" },
  { id: "viewer", label: "nav.board" },
  { id: "preorder", label: "nav.preorder" },
] as const;

export const BATTERY = {
  cell: "402020",
  capacity: "120 mAh",
  chargeCurrent: "67 mA",
};
