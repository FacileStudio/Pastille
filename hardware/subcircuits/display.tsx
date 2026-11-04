export const Display = () => (
  <>
    <pinheader
      name="J_LCD"
      pinCount={8}
      pitch="2.54mm"
      footprint="pinrow8_p2.54"
      pcbX={0}
      pcbY={14}
    />
    <trace from=".J_LCD > .pin1" to="net.GND" />
    <trace from=".J_LCD > .pin2" to="net.VDD3V3" />
    <trace from=".J_LCD > .pin3" to="net.LCD_SCLK" />
    <trace from=".J_LCD > .pin4" to="net.LCD_MOSI" />
    <trace from=".J_LCD > .pin5" to="net.LCD_DC" />
    <trace from=".J_LCD > .pin6" to="net.LCD_CS" />
    <trace from=".J_LCD > .pin7" to="net.LCD_RST" />
    <trace from=".J_LCD > .pin8" to="net.LCD_BL" />
  </>
);
