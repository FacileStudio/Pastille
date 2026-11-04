export const Mcu = () => (
  <>
    <chip
      name="U_ESP32_S3"
      footprint="qfp32"
      pcbX={0}
      pcbY={1}
      pinLabels={{
        pin1: "GND",
        pin2: "VDD3V3",
        pin3: "GPIO0",
        pin4: "GPIO4",
        pin5: "GPIO5",
        pin6: "GPIO6",
        pin7: "GPIO7",
        pin8: "GPIO10",
        pin9: "GPIO11",
        pin10: "GPIO12",
        pin11: "GPIO13",
        pin12: "GPIO14",
        pin13: "GPIO15",
        pin14: "GPIO16",
        pin15: "GPIO17",
        pin16: "GPIO18",
        pin17: "GPIO21",
      }}
      pinAttributes={{
        VDD3V3: { requiresPower: true },
        GND: { requiresGround: true },
      }}
    />
    <capacitor
      name="C_MCU_1"
      capacitance="100nF"
      footprint="0402"
      pcbX={-2}
      pcbY={7.5}
    />
    <capacitor
      name="C_MCU_BULK"
      capacitance="10uF"
      footprint="0603"
      pcbX={2}
      pcbY={7.5}
    />
    <resistor
      name="R_BOOT"
      resistance="10k"
      footprint="0402"
      pcbX={-6}
      pcbY={7.5}
    />
    <trace from=".U_ESP32_S3 > .VDD3V3" to="net.VDD3V3" />
    <trace from=".C_MCU_1 > .pin1" to="net.VDD3V3" />
    <trace from=".C_MCU_BULK > .pin1" to="net.VDD3V3" />
    <trace from=".R_BOOT > .pin2" to="net.VDD3V3" />
    <trace from=".U_ESP32_S3 > .GND" to="net.GND" />
    <trace from=".C_MCU_1 > .pin2" to="net.GND" />
    <trace from=".C_MCU_BULK > .pin2" to="net.GND" />
    <trace from=".U_ESP32_S3 > .GPIO0" to=".R_BOOT > .pin1" />
  </>
);
