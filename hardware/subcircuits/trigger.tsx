import { WirePadsFootprint } from "./wire-pads";

export const Trigger = () => (
  <>
    <testpoint
      name="TP_TOUCH"
      footprintVariant="pad"
      padShape="circle"
      padDiameter="2mm"
      pcbX={5.0}
      pcbY={15.6}
      schX={-48}
      schY={-16}
    />
    <resistor
      name="R_TOUCH"
      resistance="510"
      footprint="0402"
      manufacturerPartNumber="RC0402FR-07510RL"
      pcbX={2.6}
      pcbY={14.6}
      schX={-40}
      schY={-16}
    />
    <chip
      name="U_HAP"
      footprint="msop10"
      manufacturerPartNumber="DRV2605LDGSR"
      pcbX={7.6}
      pcbY={10.8}
      schX={-32}
      schY={-22}
      pinLabels={{
        pin1: "REG",
        pin2: "SCL",
        pin3: "SDA",
        pin4: "IN_TRIG",
        pin5: "EN",
        pin6: "VDD_NC",
        pin7: "OUT_P",
        pin8: "GND",
        pin9: "OUT_N",
        pin10: "VDD",
      }}
      pinAttributes={{
        VDD: { requiresPower: true },
        GND: { requiresGround: true },
        VDD_NC: { doNotConnect: true },
      }}
    />
    <capacitor
      name="C_HAP"
      capacitance="1uF"
      footprint="0402"
      manufacturerPartNumber="GRM155R61A105KE15D"
      pcbX={5.6}
      pcbY={13.6}
      schX={-24}
      schY={-16}
    />
    <capacitor
      name="C_HAP_REG"
      capacitance="1uF"
      footprint="0402"
      manufacturerPartNumber="GRM155R61A105KE15D"
      pcbX={7.6}
      pcbY={13.6}
      schX={-24}
      schY={-22}
    />
    <resistor
      name="R_SDA"
      resistance="4.7k"
      footprint="0402"
      manufacturerPartNumber="RC0402FR-074K7L"
      pcbX={9.8}
      pcbY={13.6}
      schX={-40}
      schY={-22}
    />
    <resistor
      name="R_SCL"
      resistance="4.7k"
      footprint="0402"
      manufacturerPartNumber="RC0402FR-074K7L"
      pcbX={10.2}
      pcbY={8.4}
      schX={-40}
      schY={-28}
    />
    <chip
      name="J_HAP"
      footprint={<WirePadsFootprint />}
      manufacturerPartNumber="VG0832012"
      layer="bottom"
      pcbX={12.6}
      pcbY={-6.6}
      schX={-24}
      schY={-28}
      pinLabels={{ pin1: "LRA_P", pin2: "LRA_N" }}
    />
    <trace from=".TP_TOUCH > .pin1" to=".R_TOUCH > .pin1" />
    <trace from=".R_TOUCH > .pin2" to="net.TOUCH_WAKE" />
    <trace from=".U_HAP > .VDD" to="net.VDD3V3" />
    <trace from=".U_HAP > .EN" to="net.VDD3V3" />
    <trace from=".U_HAP > .GND" to="net.GND" />
    <trace from=".U_HAP > .IN_TRIG" to="net.GND" />
    <trace from=".C_HAP > .pin1" to="net.VDD3V3" />
    <trace from=".C_HAP > .pin2" to="net.GND" />
    <trace from=".U_HAP > .REG" to=".C_HAP_REG > .pin1" />
    <trace from=".C_HAP_REG > .pin2" to="net.GND" />
    <trace from=".U_HAP > .SDA" to="net.I2C_SDA" />
    <trace from=".U_HAP > .SCL" to="net.I2C_SCL" />
    <trace from=".R_SDA > .pin1" to="net.I2C_SDA" />
    <trace from=".R_SDA > .pin2" to="net.VDD3V3" />
    <trace from=".R_SCL > .pin1" to="net.I2C_SCL" />
    <trace from=".R_SCL > .pin2" to="net.VDD3V3" />
    <trace from=".U_HAP > .OUT_P" to=".J_HAP > .LRA_P" />
    <trace from=".U_HAP > .OUT_N" to=".J_HAP > .LRA_N" />
  </>
);
