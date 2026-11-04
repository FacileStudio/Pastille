import { MicLga } from "./audio-mic";
import { WirePadsFootprint } from "./wire-pads";

export const Audio = () => (
  <>
    <chip
      name="U_MIC"
      footprint={<MicLga />}
      manufacturerPartNumber="SPH0645LM4H-B"
      pcbX={10.8}
      pcbY={-11.01}
      pcbRotation={180}
      schX={32}
      schY={18}
      pinLabels={{ pin1: "WS", pin2: "SELECT", pin3: "GND", pin4: "BCLK", pin5: "VDD", pin6: "DATA" }}
      pinAttributes={{ VDD: { requiresPower: true }, GND: { requiresGround: true } }}
    />
    <capacitor name="C_MIC" capacitance="100nF" footprint="0402" manufacturerPartNumber="GRM155R71C104KA88D"
      pcbX={13.6} pcbY={-9.2} schX={40} schY={18} />
    <capacitor name="C_MIC_BULK" capacitance="1uF" footprint="0402" manufacturerPartNumber="GRM155R61A105KE15D"
      pcbX={13.4} pcbY={-10.4} schX={40} schY={12} />
    <resistor name="R_MIC_PD" resistance="100k" footprint="0402" manufacturerPartNumber="RC0402FR-07100KL"
      pcbX={-8.2} pcbY={-10.6} schX={40} schY={24} />
    <chip
      name="U_AMP"
      footprint="qfn16_w3.8_h3.8_p0.5_pw0.25_pl0.8_thermalpad1.23x1.23mm"
      manufacturerPartNumber="MAX98357AETE+"
      pcbX={13.8}
      pcbY={-0.85}
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
        NC1: { doNotConnect: true },
        NC2: { doNotConnect: true },
        NC3: { doNotConnect: true },
        NC4: { doNotConnect: true },
      }}
    />
    <resistor name="R_AMP_SD" resistance="100k" footprint="0402" manufacturerPartNumber="RC0402FR-07100KL"
      pcbX={11.15} pcbY={-1.6} pcbRotation={90} schX={24} schY={6} />
    <capacitor name="C_AMP" capacitance="10uF" footprint="0402" manufacturerPartNumber="GRM155R60J106ME15D"
      pcbX={16.6} pcbY={-2.6} pcbRotation={90} schX={40} schY={6} />
    <capacitor name="C_AMP_HF" capacitance="100nF" footprint="0402" manufacturerPartNumber="GRM155R71C104KA88D"
      pcbX={16.6} pcbY={-0.4} pcbRotation={90} schX={40} schY={0} />
    <chip
      name="J_SPK"
      footprint={<WirePadsFootprint />}
      manufacturerPartNumber="CMS-151125-18S-X8"
      layer="bottom"
      pcbX={0}
      pcbY={-8.6}
      schX={48}
      schY={6}
      pinLabels={{ pin1: "SPK_P", pin2: "SPK_N" }}
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
    <trace from=".U_AMP > .GAIN_SLOT" to="net.VSYS" />
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
    <trace from=".U_AMP > .OUTP" to=".J_SPK > .SPK_P" />
    <trace from=".U_AMP > .OUTN" to=".J_SPK > .SPK_N" />
  </>
);
