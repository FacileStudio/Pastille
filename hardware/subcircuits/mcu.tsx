export const Mcu = () => (
  <>
    <chip
      name="U_ESP32_S3"
      footprint="qfn32_w6_h6_p0.5"
      manufacturerPartNumber="ESP32-S3R8"
      pcbX={0}
      pcbY={1.5}
      schX={0}
      schY={0}
      pinLabels={{
        pin1: "GND",
        pin2: "VDD3V3",
        pin3: "CHIP_PU",
        pin4: "GPIO0",
        pin5: "GPIO1",
        pin6: "GPIO2",
        pin7: "GPIO4",
        pin8: "GPIO5",
        pin9: "GPIO6",
        pin10: "GPIO7",
        pin11: "GPIO8",
        pin12: "GPIO9",
        pin13: "GPIO10",
        pin14: "GPIO11",
        pin15: "GPIO12",
        pin16: "GPIO13",
        pin17: "GPIO14",
        pin18: "GPIO15",
        pin19: "GPIO16",
        pin20: "GPIO17",
        pin21: "GPIO18",
        pin22: "GPIO19",
        pin23: "GPIO20",
        pin24: "GPIO21",
      }}
      pinAttributes={{
        VDD3V3: { requiresPower: true },
        GND: { requiresGround: true },
        CHIP_PU: { mustBeConnected: true },
      }}
    />
    <resistor
      name="R_PU"
      resistance="10k"
      footprint="0402"
      manufacturerPartNumber="RC0402FR-0710KL"
      pcbX={-7.5}
      pcbY={6.8}
      schX={-8}
      schY={6}
    />
    <capacitor
      name="C_PU"
      capacitance="1uF"
      footprint="0402"
      manufacturerPartNumber="GRM155R61A105KE15D"
      pcbX={-5.0}
      pcbY={6.8}
      schX={-8}
      schY={0}
    />
    <capacitor
      name="C_MCU_1"
      capacitance="100nF"
      footprint="0402"
      manufacturerPartNumber="GRM155R71C104KA88D"
      pcbX={-2.2}
      pcbY={6.8}
      schX={8}
      schY={6}
    />
    <capacitor
      name="C_MCU_2"
      capacitance="1uF"
      footprint="0402"
      manufacturerPartNumber="GRM155R61A105KE15D"
      pcbX={0.6}
      pcbY={6.8}
      schX={8}
      schY={0}
    />
    <capacitor
      name="C_MCU_BULK"
      capacitance="10uF"
      footprint="0603"
      manufacturerPartNumber="GRM188R61A106KE69D"
      pcbX={3.8}
      pcbY={6.8}
      schX={8}
      schY={-6}
    />
    <resistor
      name="R_BOOT"
      resistance="10k"
      footprint="0402"
      manufacturerPartNumber="RC0402FR-0710KL"
      pcbX={6.8}
      pcbY={6.8}
      schX={-8}
      schY={-6}
    />
    <pushbutton
      name="SW_BOOT"
      footprint="pushbutton"
      manufacturerPartNumber="TL3305AF160QG"
      internallyConnectedPins={[
        ["pin1", "pin2"],
        ["pin3", "pin4"],
      ]}
      pcbX={-12.2}
      pcbY={3.2}
      schX={-8}
      schY={-12}
    />
    <trace from=".U_ESP32_S3 > .VDD3V3" to="net.VDD3V3" />
    <trace from=".U_ESP32_S3 > .GND" to="net.GND" />
    <trace from=".U_ESP32_S3 > .CHIP_PU" to="net.CHIP_PU" />
    <trace from=".R_PU > .pin1" to="net.CHIP_PU" />
    <trace from=".R_PU > .pin2" to="net.VDD3V3" />
    <trace from=".C_PU > .pin1" to="net.CHIP_PU" />
    <trace from=".C_PU > .pin2" to="net.GND" />
    <trace from=".C_MCU_1 > .pin1" to="net.VDD3V3" />
    <trace from=".C_MCU_1 > .pin2" to="net.GND" />
    <trace from=".C_MCU_2 > .pin1" to="net.VDD3V3" />
    <trace from=".C_MCU_2 > .pin2" to="net.GND" />
    <trace from=".C_MCU_BULK > .pin1" to="net.VDD3V3" />
    <trace from=".C_MCU_BULK > .pin2" to="net.GND" />
    <trace from=".U_ESP32_S3 > .GPIO0" to="net.BOOT_BTN" />
    <trace from=".R_BOOT > .pin1" to="net.BOOT_BTN" />
    <trace from=".R_BOOT > .pin2" to="net.VDD3V3" />
    <trace from=".SW_BOOT > .pin1" to="net.BOOT_BTN" />
    <trace from=".SW_BOOT > .pin3" to="net.GND" />
  </>
);
