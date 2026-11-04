export const Audio = () => (
  <>
    <chip
      name="U_MIC"
      footprint="soic8"
      manufacturerPartNumber="SPH0645LM4H-B"
      pcbX={11.5}
      pcbY={-5.5}
      schX={32}
      schY={18}
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
      manufacturerPartNumber="GRM155R71C104KA88D"
      pcbX={9.8}
      pcbY={-9.0}
      schX={40}
      schY={18}
    />
    <capacitor
      name="C_MIC_BULK"
      capacitance="1uF"
      footprint="0402"
      manufacturerPartNumber="GRM155R61A105KE15D"
      pcbX={12.2}
      pcbY={-9.0}
      schX={40}
      schY={12}
    />
    <chip
      name="U_AMP"
      footprint="soic14"
      manufacturerPartNumber="MAX98357AETE+"
      pcbX={11.5}
      pcbY={2.0}
      schX={32}
      schY={6}
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
    <resistor
      name="R_AMP_SD"
      resistance="100k"
      footprint="0402"
      manufacturerPartNumber="RC0402FR-07100KL"
      pcbX={6.0}
      pcbY={2.0}
      schX={24}
      schY={6}
    />
    <capacitor
      name="C_AMP"
      capacitance="10uF"
      footprint="0603"
      manufacturerPartNumber="GRM188R61A106KE69D"
      pcbX={9.5}
      pcbY={7.8}
      schX={40}
      schY={6}
    />
    <capacitor
      name="C_AMP_HF"
      capacitance="100nF"
      footprint="0402"
      manufacturerPartNumber="GRM155R71C104KA88D"
      pcbX={12.5}
      pcbY={7.8}
      schX={40}
      schY={0}
    />
    <pinheader
      name="J_SPK"
      pinCount={2}
      pitch="2.54mm"
      footprint="pinrow2_p2.54"
      manufacturerPartNumber="CDS-20148"
      pcbX={10.5}
      pcbY={10.5}
      schX={48}
      schY={6}
    />
    <trace from=".U_MIC > .VDD" to="net.VDD3V3" />
    <trace from=".U_MIC > .GND" to="net.GND" />
    <trace from=".U_MIC > .L_R" to="net.GND" />
    <trace from=".C_MIC > .pin1" to="net.VDD3V3" />
    <trace from=".C_MIC > .pin2" to="net.GND" />
    <trace from=".C_MIC_BULK > .pin1" to="net.VDD3V3" />
    <trace from=".C_MIC_BULK > .pin2" to="net.GND" />
    <trace from=".U_MIC > .WS" to="net.MIC_WS" />
    <trace from=".U_MIC > .SCK" to="net.MIC_SCK" />
    <trace from=".U_MIC > .SD" to="net.MIC_SD" />
    <trace from=".U_AMP > .VIN" to="net.VSYS" />
    <trace from=".U_AMP > .GND" to="net.GND" />
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
