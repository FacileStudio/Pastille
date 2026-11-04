export const Trigger = () => (
  <>
    <testpoint
      name="TP_TOUCH"
      footprintVariant="pad"
      padShape="circle"
      padDiameter="2mm"
      pcbX={-7.8}
      pcbY={9.5}
      schX={-48}
      schY={-16}
    />
    <resistor
      name="R_TOUCH"
      resistance="510"
      footprint="0402"
      manufacturerPartNumber="RC0402FR-07510RL"
      pcbX={-11.0}
      pcbY={9.5}
      schX={-40}
      schY={-16}
    />
    <chip
      name="U_HAP"
      footprint="soic8"
      manufacturerPartNumber="DRV2605LDGSR"
      pcbX={7.9}
      pcbY={-12.2}
      schX={-32}
      schY={-22}
      pinLabels={{
        pin1: "REG",
        pin2: "SCL",
        pin3: "SDA",
        pin4: "IN_TRIG",
        pin5: "EN",
        pin6: "GND",
        pin7: "OUT_P",
        pin8: "VDD",
      }}
      pinAttributes={{
        VDD: { requiresPower: true },
        GND: { requiresGround: true },
      }}
    />
    <capacitor
      name="C_HAP"
      capacitance="100nF"
      footprint="0402"
      manufacturerPartNumber="GRM155R71C104KA88D"
      pcbX={12.4}
      pcbY={-11.8}
      schX={-24}
      schY={-16}
    />
    <capacitor
      name="C_HAP_REG"
      capacitance="1uF"
      footprint="0402"
      manufacturerPartNumber="GRM155R61A105KE15D"
      pcbX={8.6}
      pcbY={-7.3}
      schX={-24}
      schY={-22}
    />
    <resistor
      name="R_SDA"
      resistance="4.7k"
      footprint="0402"
      manufacturerPartNumber="RC0402FR-074K7L"
      pcbX={5.4}
      pcbY={3.5}
      schX={-40}
      schY={-22}
    />
    <resistor
      name="R_SCL"
      resistance="4.7k"
      footprint="0402"
      manufacturerPartNumber="RC0402FR-074K7L"
      pcbX={8.4}
      pcbY={3.5}
      schX={-40}
      schY={-28}
    />
    <pinheader
      name="J_HAP"
      pinCount={2}
      pitch="2.54mm"
      footprint="pinrow2_p2.54"
      manufacturerPartNumber="LRA-0827"
      pcbX={11.5}
      pcbY={4.4}
      schX={-24}
      schY={-28}
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
    <trace from=".U_HAP > .OUT_P" to=".J_HAP > .pin1" />
    <trace from=".J_HAP > .pin2" to="net.GND" />
  </>
);
