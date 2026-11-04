const MIC_PORT_Y = -0.71;
const MIC_RING_SEGMENTS = 32;

const micRingPoint = (radius: number, step: number) => {
  const angle = (2 * Math.PI * step) / MIC_RING_SEGMENTS;
  return {
    x: Number((radius * Math.cos(angle)).toFixed(4)),
    y: Number((MIC_PORT_Y + radius * Math.sin(angle)).toFixed(4)),
  };
};

const MIC_RING_STEPS = Array.from({ length: MIC_RING_SEGMENTS + 1 }, (_, i) => i);
const MIC_GND_RING = [
  ...MIC_RING_STEPS.map((i) => micRingPoint(0.812, i)),
  ...MIC_RING_STEPS.map((i) => micRingPoint(0.512, MIC_RING_SEGMENTS - i)),
];

const MIC_PADS = [
  { pin: "pin1", x: -0.9, y: 1.364, width: 0.6, height: 0.522 },
  { pin: "pin6", x: 0, y: 1.364, width: 0.6, height: 0.522 },
  { pin: "pin5", x: 0.9, y: 1.364, width: 0.6, height: 0.522 },
  { pin: "pin2", x: -0.9, y: 0.542, width: 0.6, height: 0.522 },
  { pin: "pin4", x: 0.9, y: 0.542, width: 0.6, height: 0.522 },
  { pin: "pin3", x: -0.25, y: MIC_PORT_Y - 0.95, width: 0.3, height: 0.7 },
  { pin: "pin3", x: 0.25, y: MIC_PORT_Y - 0.95, width: 0.3, height: 0.7 },
];

type MicPadProps = (typeof MIC_PADS)[number];

const MicPad = ({ pin, x, y, width, height }: MicPadProps) => (
  <smtpad
    shape="rect"
    width={`${width}mm`}
    height={`${height}mm`}
    pcbX={x}
    pcbY={y}
    portHints={[pin]}
    noSolderPaste={pin === "pin3"}
  />
);

/** SPH0645LM4H-B land pattern with the bottom acoustic port and its ground ring. */
export const MicLga = () => (
  <footprint>
    {MIC_PADS.map((pad) => (
      <MicPad key={`${pad.pin}${pad.x}`} {...pad} />
    ))}
    <smtpad shape="polygon" points={MIC_GND_RING} portHints={["pin3"]} />
    <hole diameter="0.5mm" pcbX={0} pcbY={MIC_PORT_Y} />
    <courtyardrect width="3.15mm" height="4.3mm" pcbX={0} pcbY={-0.15} />
    <silkscreenpath
      route={[
        { x: -1.45, y: 1.3 },
        { x: -1.45, y: 1.9 },
        { x: -0.85, y: 1.9 },
      ]}
    />
  </footprint>
);
