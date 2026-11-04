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

const MicLga = () => (
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

export const Audio = () => (
  <>
    <chip
      name="U_MIC"
      footprint={<MicLga />}
      manufacturerPartNumber="SPH0645LM4H-B"
      pcbX={13.5}
      pcbY={-0.8}
      schX={32}
      schY={18}
      pinLabels={{
        pin1: "WS",
        pin2: "SELECT",
        pin3: "GND",
        pin4: "BCLK",
        pin5: "VDD",
        pin6: "DATA",
      }}
      pinAttributes={{
        VDD: { requiresPower: true },
        GND: { requiresGround: true },
      }}
    />
    <capacitor
      name="C_MIC"
      capacitance="100nF"
      footprint="0402"
      manufacturerPartNumber="GRM155R71C104KA88D"
      pcbX={8.2}
      pcbY={0.8}
      schX={40}
      schY={18}
    />
    <resistor
      name="R_MIC_PD"
      resistance="100k"
      footprint="0402"
      manufacturerPartNumber="RC0402FR-07100KL"
      pcbX={13.5}
      pcbY={2.6}
      schX={40}
      schY={24}
    />
    <capacitor
      name="C_MIC_BULK"
      capacitance="1uF"
      footprint="0402"
      manufacturerPartNumber="GRM155R61A105KE15D"
      pcbX={8.4}
      pcbY={-1.8}
      schX={40}
      schY={12}
    />
    <chip
      name="U_AMP"
      footprint="qfn16_w3.8_h3.8_p0.5_pw0.25_pl0.8_thermalpad1.23x1.23mm"
      manufacturerPartNumber="MAX98357AETE+"
      pcbX={12.8}
      pcbY={-6.8}
      schX={32}
      schY={6}
      pinLabels={{
        pin1: "DIN",
        pin2: "GAIN_SLOT",
        pin3: "GND1",
        pin4: "SD_MODE",
        pin5: "NC1",
        pin6: "NC2",
        pin7: "VDD1",
        pin8: "VDD2",
        pin9: "OUTP",
        pin10: "OUTN",
        pin11: "GND2",
        pin12: "NC3",
        pin13: "NC4",
        pin14: "LRCLK",
        pin15: "GND3",
        pin16: "BCLK",
        pin17: "thermalpad",
      }}
      pinAttributes={{
        VDD1: { requiresPower: true },
        VDD2: { requiresPower: true },
        GND1: { requiresGround: true },
        GND2: { requiresGround: true },
        GND3: { requiresGround: true },
        NC1: { doNotConnect: true },
        NC2: { doNotConnect: true },
        NC3: { doNotConnect: true },
        NC4: { doNotConnect: true },
      }}
    />
    <resistor
      name="R_AMP_SD"
      resistance="100k"
      footprint="0402"
      manufacturerPartNumber="RC0402FR-07100KL"
      pcbX={5.4}
      pcbY={0.8}
      schX={24}
      schY={6}
    />
    <capacitor
      name="C_AMP"
      capacitance="10uF"
      footprint="0603"
      manufacturerPartNumber="GRM188R61A106KE69D"
      pcbX={8.6}
      pcbY={-4.6}
      schX={40}
      schY={6}
    />
    <capacitor
      name="C_AMP_HF"
      capacitance="100nF"
      footprint="0402"
      manufacturerPartNumber="GRM155R71C104KA88D"
      pcbX={5.5}
      pcbY={-7.3}
      schX={40}
      schY={0}
    />
    <pinheader
      name="J_SPK"
      pinCount={2}
      pitch="2.54mm"
      footprint="pinrow2_p2.54"
      manufacturerPartNumber="CMS-151125-18S-X8"
      pcbX={3.6}
      pcbY={13.4}
      schX={48}
      schY={6}
    />
    <trace from=".U_MIC > .VDD" to="net.VDD3V3" />
    <trace from=".U_MIC > .GND" to="net.GND" />
    <trace from=".U_MIC > .SELECT" to="net.GND" />
    <trace from=".C_MIC > .pin1" to="net.VDD3V3" />
    <trace from=".C_MIC > .pin2" to="net.GND" />
    <trace from=".C_MIC_BULK > .pin1" to="net.VDD3V3" />
    <trace from=".C_MIC_BULK > .pin2" to="net.GND" />
    <trace from=".U_MIC > .WS" to="net.MIC_WS" />
    <trace from=".U_MIC > .BCLK" to="net.MIC_SCK" />
    <trace from=".U_MIC > .DATA" to="net.MIC_SD" />
    <trace from=".R_MIC_PD > .pin1" to="net.MIC_SD" />
    <trace from=".R_MIC_PD > .pin2" to="net.GND" />
    <trace from=".U_AMP > .VDD1" to="net.VSYS" />
    <trace from=".U_AMP > .VDD2" to="net.VSYS" />
    <trace from=".U_AMP > .GND1" to="net.GND" />
    <trace from=".U_AMP > .GND2" to="net.GND" />
    <trace from=".U_AMP > .GND3" to="net.GND" />
    <trace from=".U_AMP > .thermalpad" to="net.GND" />
    <trace from=".C_AMP > .pin1" to="net.VSYS" />
    <trace from=".C_AMP > .pin2" to="net.GND" />
    <trace from=".C_AMP_HF > .pin1" to="net.VSYS" />
    <trace from=".C_AMP_HF > .pin2" to="net.GND" />
    <trace from=".U_AMP > .BCLK" to="net.SPK_BCLK" />
    <trace from=".U_AMP > .LRCLK" to="net.SPK_LRCLK" />
    <trace from=".U_AMP > .DIN" to="net.SPK_DIN" />
    <trace from=".U_AMP > .SD_MODE" to="net.SPK_SD_MODE" />
    <trace from=".R_AMP_SD > .pin1" to="net.SPK_SD_MODE" />
    <trace from=".R_AMP_SD > .pin2" to="net.GND" />
    <trace from=".U_AMP > .OUTP" to=".J_SPK > .pin1" />
    <trace from=".U_AMP > .OUTN" to=".J_SPK > .pin2" />
  </>
);
