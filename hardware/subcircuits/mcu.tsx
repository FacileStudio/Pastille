import { Mini1Footprint } from "./mini1";
import { MINI1_GROUNDS, MINI1_LABELS, MINI1_UNUSED } from "./mini1-pins";

export const MCU_AT = { x: -3.47, y: 0 };

const PIN_ATTRIBUTES = {
  VDD3V3: { requiresPower: true },
  GND1: { requiresGround: true },
  CHIP_PU: { mustBeConnected: true },
  ...Object.fromEntries(MINI1_UNUSED.map((label) => [label, { doNotConnect: true }])),
};

const GroundTrace = ({ label }: { label: string }) => <trace from={`.U_ESP32_S3 > .${label}`} to="net.GND" />;

const TestPad = ({ name, x, y }: { name: string; x: number; y: number }) => (
  <testpoint name={name} footprintVariant="pad" padShape="circle" padDiameter="1mm" pcbX={x} pcbY={y} />
);

export const Mcu = () => (
  <>
    <chip
      name="U_ESP32_S3"
      footprint={<Mini1Footprint />}
      manufacturerPartNumber="ESP32-S3-MINI-1-N4R2"
      supplierPartNumbers={{ jlcpcb: ["C3013941"] }}
      pcbX={MCU_AT.x}
      pcbY={MCU_AT.y}
      schX={0}
      schY={0}
      pinLabels={MINI1_LABELS}
      pinAttributes={PIN_ATTRIBUTES}
    />
    <resistor name="R_PU" resistance="10k" footprint="0402" manufacturerPartNumber="RC0402FR-0710KL"
      pcbX={-8.1} pcbY={8.9} schX={-8} schY={6} />
    <capacitor name="C_PU" capacitance="1uF" footprint="0402" manufacturerPartNumber="GRM155R61A105KE15D"
      pcbX={-10.0} pcbY={8.9} schX={-8} schY={0} />
    <capacitor name="C_MCU_1" capacitance="100nF" footprint="0402" manufacturerPartNumber="GRM155R71C104KA88D"
      pcbX={-6.6} pcbY={-8.6} schX={8} schY={6} />
    <capacitor name="C_MCU_BULK" capacitance="22uF" footprint="0603" manufacturerPartNumber="GRM188R61A226ME15D"
      pcbX={-4.0} pcbY={-8.75} schX={8} schY={0} />
    <resistor name="R_BOOT" resistance="10k" footprint="0402" manufacturerPartNumber="RC0402FR-0710KL"
      pcbX={-6.2} pcbY={8.9} schX={-8} schY={-6} />
    <TestPad name="TP_EN" x={-9.8} y={10.6} />
    <TestPad name="TP_BOOT" x={-7.8} y={10.6} />
    <TestPad name="TP_GND" x={-5.8} y={10.6} />
    {MINI1_GROUNDS.map((label) => (
      <GroundTrace key={label} label={label} />
    ))}
    <trace from=".U_ESP32_S3 > .VDD3V3" to="net.VDD3V3" />
    <trace from=".U_ESP32_S3 > .CHIP_PU" to="net.CHIP_PU" />
    <trace from=".R_PU > .pin1" to="net.CHIP_PU" />
    <trace from=".R_PU > .pin2" to="net.VDD3V3" />
    <trace from=".C_PU > .pin1" to="net.CHIP_PU" />
    <trace from=".C_PU > .pin2" to="net.GND" />
    <trace from=".TP_EN > .pin1" to="net.CHIP_PU" />
    <trace from=".C_MCU_1 > .pin1" to="net.VDD3V3" />
    <trace from=".C_MCU_1 > .pin2" to="net.GND" />
    <trace from=".C_MCU_BULK > .pin1" to="net.VDD3V3" />
    <trace from=".C_MCU_BULK > .pin2" to="net.GND" />
    <trace from=".U_ESP32_S3 > .GPIO0" to="net.BOOT_BTN" />
    <trace from=".R_BOOT > .pin1" to="net.BOOT_BTN" />
    <trace from=".R_BOOT > .pin2" to="net.VDD3V3" />
    <trace from=".TP_BOOT > .pin1" to="net.BOOT_BTN" />
    <trace from=".TP_GND > .pin1" to="net.GND" />
  </>
);
