const PITCH = 0.85;
const ROW = 7.0;
const SIDE_PINS = 15;
const EPAD_PITCH = 1.7;
const EPAD_PASTE = 1.0;

type Pad = { pin: number; x: number; y: number; width: number; height: number };

const offset = (i: number): number => Number(((i - (SIDE_PINS - 1) / 2) * PITCH).toFixed(3));

const side = (first: number, at: (i: number) => [number, number], width: number, height: number): Pad[] =>
  Array.from({ length: SIDE_PINS }, (_, i) => {
    const [x, y] = at(i);
    return { pin: first + i, x, y, width, height };
  });

const CORNERS: Pad[] = [
  { pin: 62, x: -ROW, y: ROW, width: 0.8, height: 0.8 },
  { pin: 63, x: -ROW, y: -ROW, width: 0.8, height: 0.8 },
  { pin: 64, x: ROW, y: -ROW, width: 0.8, height: 0.8 },
  { pin: 65, x: ROW, y: ROW, width: 0.8, height: 0.8 },
];

/** The 65 lands of the ESP32-S3-MINI-1 in the datasheet frame (v1.7, figure 11-1), centred on the pad field. */
export const MINI1_PADS: Pad[] = [
  ...side(1, (i) => [-ROW, -offset(i)], 0.8, 0.4),
  ...side(16, (i) => [offset(i), -ROW], 0.4, 0.8),
  ...side(31, (i) => [ROW, offset(i)], 0.8, 0.4),
  ...side(46, (i) => [-offset(i), ROW], 0.4, 0.8),
  ...CORNERS,
];

const PASTE = [-1, 0, 1].flatMap((i) => [-1, 0, 1].map((j) => [i * EPAD_PITCH, j * EPAD_PITCH]));

const Land = ({ pin, x, y, width, height }: Pad) => (
  <smtpad shape="rect" width={`${height}mm`} height={`${width}mm`} pcbX={-y} pcbY={x} portHints={[`pin${pin}`]} />
);

const Paste = ({ at }: { at: number[] }) => (
  <solderpaste shape="rect" width={`${EPAD_PASTE}mm`} height={`${EPAD_PASTE}mm`} pcbX={at[0]} pcbY={at[1]} />
);

/** ESP32-S3-MINI-1 land pattern turned a quarter turn: 64 perimeter lands, one 4.5 mm ground pad, antenna toward -X. */
export const Mini1Footprint = () => (
  <footprint>
    {MINI1_PADS.map((pad) => (
      <Land key={pad.pin} {...pad} />
    ))}
    <smtpad shape="rect" width="4.5mm" height="4.5mm" pcbX={0} pcbY={0} portHints={["pin61"]} noSolderPaste />
    {PASTE.map((at) => (
      <Paste key={`${at[0]}_${at[1]}`} at={at} />
    ))}
    <courtyardrect width="21mm" height="15.9mm" pcbX={-2.55} pcbY={0} />
    <silkscreenpath
      route={[
        { x: -7.4, y: -7.8 },
        { x: 7.8, y: -7.8 },
        { x: 7.8, y: 7.8 },
        { x: -7.4, y: 7.8 },
      ]}
    />
  </footprint>
);
