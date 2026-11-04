const PAD_MM = 1.4;
const PITCH_MM = 2.4;

const WireLand = ({ pin }: { pin: number }) => (
  <smtpad
    shape="rect"
    width={`${PAD_MM}mm`}
    height={`${PAD_MM}mm`}
    pcbX={(pin - 1.5) * PITCH_MM}
    pcbY={0}
    portHints={[`pin${pin}`]}
  />
);

/** Two hand-solder lands for the leads of a part that sits under the board. */
export const WirePadsFootprint = () => (
  <footprint>
    <WireLand pin={1} />
    <WireLand pin={2} />
    <courtyardrect width={`${PITCH_MM + PAD_MM + 0.4}mm`} height={`${PAD_MM + 0.4}mm`} pcbX={0} pcbY={0} />
  </footprint>
);
