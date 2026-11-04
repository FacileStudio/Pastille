type Element = Record<string, any>;
type Placed = { pcb: Element; source: Element; cad?: Element };

const UNPLACED = new Set(["simple_test_point", "simple_fiducial"]);
const WIRE_PADS = new Set(["J_SPK", "J_HAP"]);

const csvCell = (value: unknown): string => {
  const text = String(value ?? "");
  return /[",\n]/.test(text) ? `"${text.replaceAll('"', '""')}"` : text;
};

const csv = (rows: unknown[][]): string => `${rows.map((row) => row.map(csvCell).join(",")).join("\n")}\n`;

const placed = (circuit: Element[]): Placed[] => {
  const sources = new Map(circuit.filter((e) => e.type === "source_component").map((e) => [e.source_component_id, e]));
  const cad = new Map(circuit.filter((e) => e.type === "cad_component").map((e) => [e.pcb_component_id, e]));
  return circuit
    .filter((e) => e.type === "pcb_component")
    .map((pcb) => ({ pcb, source: sources.get(pcb.source_component_id) ?? {}, cad: cad.get(pcb.pcb_component_id) }))
    .filter(({ source }) => source.name && !UNPLACED.has(source.ftype) && !WIRE_PADS.has(source.name));
};

const value = (source: Element): string =>
  source.display_resistance ?? source.display_capacitance ?? source.manufacturer_part_number ?? source.name;

/** The JLCPCB assembly BOM: one line per part number, with every designator that uses it. */
export const bomCsv = (circuit: Element[]): string => {
  const lines = new Map<string, { comment: string; refs: string[]; footprint: string; lcsc: string; mpn: string }>();
  for (const { source, cad } of placed(circuit)) {
    const mpn = source.manufacturer_part_number ?? "";
    const line = lines.get(mpn) ?? {
      comment: value(source),
      refs: [] as string[],
      footprint: cad?.footprinter_string ?? "custom",
      lcsc: source.supplier_part_numbers?.jlcpcb?.[0] ?? "",
      mpn,
    };
    line.refs.push(source.name);
    lines.set(mpn, line);
  }
  const rows = [...lines.values()].map((l) => [l.comment, l.refs.sort().join(" "), l.footprint, l.lcsc, l.mpn]);
  return csv([["Comment", "Designator", "Footprint", "LCSC Part #", "Manufacturer Part"], ...rows]);
};

/** The JLCPCB pick-and-place (CPL) file, in mm from the board centre, rotations as tscircuit places them. */
export const cplCsv = (circuit: Element[]): string => {
  const rows = placed(circuit).map(({ pcb, source }) => [
    source.name,
    `${pcb.center.x.toFixed(3)}mm`,
    `${pcb.center.y.toFixed(3)}mm`,
    pcb.layer === "bottom" ? "Bottom" : "Top",
    (((pcb.rotation ?? 0) % 360) + 360) % 360,
  ]);
  rows.sort((a, b) => String(a[0]).localeCompare(String(b[0])));
  return csv([["Designator", "Mid X", "Mid Y", "Layer", "Rotation"], ...rows]);
};
