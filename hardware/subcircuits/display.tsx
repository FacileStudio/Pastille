const FPC_PINS = ["GND1", "LEDK", "LEDA", "VDD", "GND5", "GND6", "DC", "CS", "SCL", "SDA", "RESET", "GND12"];
const FPC_PITCH = 0.5;
const FPC_ROW = 1.65;

const Contact = ({ index }: { index: number }) => (
  <smtpad
    shape="rect"
    width="0.3mm"
    height="0.8mm"
    pcbX={Number(((index - 5.5) * FPC_PITCH).toFixed(2))}
    pcbY={-FPC_ROW}
    portHints={[`pin${index + 1}`]}
  />
);

/** Hirose FH34SRJ-12S-0.5SH(50) land pattern. The FPC enters from +Y; contact 1 sits at -X. */
const Fh34Footprint = () => (
  <footprint>
    {FPC_PINS.map((label, index) => (
      <Contact key={label} index={index} />
    ))}
    <smtpad shape="rect" width="0.4mm" height="0.8mm" pcbX={-3.75} pcbY={FPC_ROW} portHints={["pin13"]} />
    <smtpad shape="rect" width="0.4mm" height="0.8mm" pcbX={3.75} pcbY={FPC_ROW} portHints={["pin14"]} />
    <courtyardrect width="8.6mm" height="4.6mm" pcbX={0} pcbY={0.1} />
    <silkscreenpath route={[{ x: -4.2, y: -1.0 }, { x: -4.2, y: 2.2 }]} />
  </footprint>
);

const LABELS = {
  ...Object.fromEntries(FPC_PINS.map((label, index) => [`pin${index + 1}`, label])),
  pin13: "MT1",
  pin14: "MT2",
};

export const Display = () => (
  <>
    <chip
      name="J_LCD"
      footprint={<Fh34Footprint />}
      manufacturerPartNumber="FH34SRJ-12S-0.5SH(50)"
      supplierPartNumbers={{ jlcpcb: ["C424659"] }}
      pcbX={0}
      pcbY={11.2}
      schX={36}
      schY={-18}
      pinLabels={LABELS}
      pinAttributes={{
        VDD: { requiresPower: true },
        GND1: { requiresGround: true },
        MT1: { doNotConnect: true },
        MT2: { doNotConnect: true },
      }}
    />
    <capacitor name="C_LCD" capacitance="1uF" footprint="0402" manufacturerPartNumber="GRM155R61A105KE15D"
      pcbX={-3.2} pcbY={14.8} schX={44} schY={-18} />
    <chip name="Q_BL" footprint="sot23" manufacturerPartNumber="AO3400A" supplierPartNumbers={{ jlcpcb: ["C20917"] }}
      pinLabels={{ pin1: "G", pin2: "S", pin3: "D" }} pcbX={-7.6} pcbY={13.0} schX={44} schY={-26} />
    <resistor name="R_BL" resistance="10" footprint="0402" manufacturerPartNumber="RC0402FR-0710RL"
      pcbX={-5.0} pcbY={13.0} pcbRotation={90} schX={40} schY={-26} />
    <resistor name="R_BL_PD" resistance="100k" footprint="0402" manufacturerPartNumber="RC0402FR-07100KL"
      pcbX={-6.6} pcbY={15.6} schX={48} schY={-26} />
    <trace from=".J_LCD > .GND1" to="net.GND" />
    <trace from=".J_LCD > .GND5" to="net.GND" />
    <trace from=".J_LCD > .GND6" to="net.GND" />
    <trace from=".J_LCD > .GND12" to="net.GND" />
    <trace from=".J_LCD > .VDD" to="net.VDD3V3" />
    <trace from=".J_LCD > .LEDA" to="net.VDD3V3" />
    <trace from=".C_LCD > .pin1" to="net.VDD3V3" />
    <trace from=".C_LCD > .pin2" to="net.GND" />
    <trace from=".J_LCD > .SCL" to="net.LCD_SCLK" />
    <trace from=".J_LCD > .SDA" to="net.LCD_MOSI" />
    <trace from=".J_LCD > .DC" to="net.LCD_DC" />
    <trace from=".J_LCD > .CS" to="net.LCD_CS" />
    <trace from=".J_LCD > .RESET" to="net.LCD_RST" />
    <trace from=".J_LCD > .LEDK" to=".R_BL > .pin1" />
    <trace from=".R_BL > .pin2" to=".Q_BL > .D" />
    <trace from=".Q_BL > .S" to="net.GND" />
    <trace from=".Q_BL > .G" to="net.LCD_BL" />
    <trace from=".R_BL_PD > .pin1" to="net.LCD_BL" />
    <trace from=".R_BL_PD > .pin2" to="net.GND" />
  </>
);
