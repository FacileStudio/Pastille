export const Audio = () => (
  <>
    <chip
      name="U_MIC"
      footprint="soic8"
      pcbX={11}
      pcbY={-8}
      pinLabels={{
        pin1: "WS",
        pin2: "SCK",
        pin3: "VDD",
        pin4: "GND",
        pin5: "SD",
        pin6: "L_R",
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
      pcbX={11}
      pcbY={-4}
    />
    <chip
      name="U_AMP"
      footprint="soic14"
      pcbX={11}
      pcbY={1}
      pinLabels={{
        pin1: "DIN",
        pin2: "BCLK",
        pin3: "LRCLK",
        pin4: "SD_MODE",
        pin5: "GND",
        pin6: "VIN",
        pin7: "OUTP",
        pin8: "OUTN",
      }}
      pinAttributes={{
        VIN: { requiresPower: true },
        GND: { requiresGround: true },
      }}
    />
    <capacitor
      name="C_AMP"
      capacitance="10uF"
      footprint="0805"
      pcbX={11}
      pcbY={6}
    />
    <pinheader
      name="J_SPK"
      pinCount={2}
      pitch="2.54mm"
      footprint="pinrow2_p2.54"
      pcbX={11}
      pcbY={11}
    />
    <trace from=".U_MIC > .VDD" to="net.VDD3V3" />
    <trace from=".U_MIC > .GND" to="net.GND" />
    <trace from=".U_MIC > .L_R" to="net.GND" />
    <trace from=".C_MIC > .pin1" to="net.VDD3V3" />
    <trace from=".C_MIC > .pin2" to="net.GND" />
    <trace from=".U_MIC > .WS" to="net.MIC_WS" />
    <trace from=".U_MIC > .SCK" to="net.MIC_SCK" />
    <trace from=".U_MIC > .SD" to="net.MIC_SD" />
    <trace from=".U_AMP > .VIN" to="net.VDD3V3" />
    <trace from=".U_AMP > .GND" to="net.GND" />
    <trace from=".C_AMP > .pin1" to="net.VDD3V3" />
    <trace from=".C_AMP > .pin2" to="net.GND" />
    <trace from=".U_AMP > .BCLK" to="net.SPK_BCLK" />
    <trace from=".U_AMP > .LRCLK" to="net.SPK_LRCLK" />
    <trace from=".U_AMP > .DIN" to="net.SPK_DIN" />
    <trace from=".U_AMP > .SD_MODE" to="net.SPK_SD_MODE" />
    <trace from=".U_AMP > .OUTP" to=".J_SPK > .pin1" />
    <trace from=".U_AMP > .OUTN" to=".J_SPK > .pin2" />
  </>
);
