const GND_PINS = [1, 2, 42, 43, ...Array.from({ length: 20 }, (_, i) => 46 + i)];

const SIGNALS: Record<number, string> = {
  3: "VDD3V3",
  ...Object.fromEntries(Array.from({ length: 12 }, (_, i) => [4 + i, `GPIO${i}`])),
  ...Object.fromEntries(Array.from({ length: 10 }, (_, i) => [16 + i, `GPIO${12 + i}`])),
  7: "NC_IO3",
  26: "NC_IO26",
  27: "NC_IO47",
  28: "NC_IO33",
  29: "NC_IO34",
  30: "NC_IO48",
  ...Object.fromEntries(Array.from({ length: 8 }, (_, i) => [31 + i, `NC_IO${35 + i}`])),
  39: "NC_TXD0",
  40: "NC_RXD0",
  41: "NC_IO45",
  44: "NC_IO46",
  45: "CHIP_PU",
};

/** Ground pin labels of the module, one per land. */
export const MINI1_GROUNDS = GND_PINS.map((pin) => `GND${pin}`);

/** Pin labels of the ESP32-S3-MINI-1 (datasheet v1.7, table 3-1). EN is named CHIP_PU after the chip pin. */
export const MINI1_LABELS: Record<string, string> = {
  ...Object.fromEntries(GND_PINS.map((pin) => [`pin${pin}`, `GND${pin}`])),
  ...Object.fromEntries(Object.entries(SIGNALS).map(([pin, label]) => [`pin${pin}`, label])),
};

/** Pins this board leaves open. IO26 drives the in-package PSRAM of the -N4R2 version. */
export const MINI1_UNUSED = Object.values(SIGNALS).filter((label) => label.startsWith("NC_"));
