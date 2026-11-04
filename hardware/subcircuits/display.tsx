export const Display = () => (
  <>
    <pinheader
      name="J_LCD"
      pinCount={8}
      pitch="2.54mm"
      footprint="pinrow8_p2.54"
      manufacturerPartNumber="GC9A01-128"
      pcbX={0}
      pcbY={12.8}
      schX={36}
      schY={-18}
    />
    <capacitor
      name="C_LCD"
      capacitance="100nF"
      footprint="0402"
      manufacturerPartNumber="GRM155R71C104KA88D"
      pcbX={9.4}
      pcbY={6.8}
      schX={44}
      schY={-18}
    />
    <trace from=".J_LCD > .pin1" to="net.GND" />
    <trace from=".J_LCD > .pin2" to="net.VDD3V3" />
    <trace from=".C_LCD > .pin1" to="net.VDD3V3" />
    <trace from=".C_LCD > .pin2" to="net.GND" />
    <trace from=".J_LCD > .pin3" to="net.LCD_SCLK" />
    <trace from=".J_LCD > .pin4" to="net.LCD_MOSI" />
    <trace from=".J_LCD > .pin5" to="net.LCD_DC" />
    <trace from=".J_LCD > .pin6" to="net.LCD_CS" />
    <trace from=".J_LCD > .pin7" to="net.LCD_RST" />
    <trace from=".J_LCD > .pin8" to="net.LCD_BL" />
  </>
);
