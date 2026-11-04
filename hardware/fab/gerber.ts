type Vertex = { x: number; y: number };
type Ring = { vertices: Vertex[] };
export type Pour = { layer: string; brep_shape: { outer_ring: Ring; inner_rings?: Ring[] } };

const COPPER_FILE: Record<string, string> = {
  top: "F_Cu.gbr",
  inner1: "In1_Cu.gbr",
  inner2: "In2_Cu.gbr",
  bottom: "B_Cu.gbr",
};
const UNIT = 1e6;

const coord = ({ x, y }: Vertex): string => `X${Math.round(x * UNIT)}Y${Math.round(y * UNIT)}`;

const region = (ring: Ring): string[] => {
  const [first, ...rest] = ring.vertices;
  return ["G36*", `${coord(first)}D02*`, ...rest.map((v) => `${coord(v)}D01*`), `${coord(first)}D01*`, "G37*"];
};

const pourBlock = (pour: Pour): string[] => {
  const holes = pour.brep_shape.inner_rings ?? [];
  const clear = holes.length ? ["%LPC*%", ...holes.flatMap(region), "%LPD*%"] : [];
  return ["%LPD*%", "G01*", ...region(pour.brep_shape.outer_ring), ...clear];
};

/** Writes each copper pour as a G36 region at the start of its layer, before the pads and traces draw over it. */
export const withPours = (files: Record<string, string>, pours: Pour[]): Record<string, string> => {
  const out = { ...files };
  for (const pour of pours) {
    const name = COPPER_FILE[pour.layer];
    const text = out[name];
    if (!text) throw new Error(`no gerber ${name} for the ${pour.layer} pour`);
    if (text.includes("G36*")) continue;
    const at = text.indexOf("%LPD*%");
    if (at < 0) throw new Error(`${name} has no %LPD*% to insert the pour before`);
    out[name] = `${text.slice(0, at)}${pourBlock(pour).join("\n")}\n${text.slice(at)}`;
  }
  return out;
};

/** Sets the drill file layer span to the real copper count: Fusible writes 1,2 on every board. */
export const withLayerSpan = (files: Record<string, string>, layers: number): Record<string, string> =>
  Object.fromEntries(
    Object.entries(files).map(([name, text]) => [
      name,
      name.endsWith(".drl") ? text.replace(/(TF\.FileFunction,(?:NonPlated|Plated),1,)2,/, `$1${layers},`) : text,
    ]),
  );
