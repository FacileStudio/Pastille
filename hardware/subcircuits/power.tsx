export const Power = () => (
  <>
    <connector
      name="J_USB"
      standard="usb_c"
      pcbX={0}
      pcbY={-15}
    />
    <resistor
      name="R_CC1"
      resistance="5.1k"
      footprint="0402"
      pcbX={-3}
      pcbY={-9.5}
    />
    <resistor
      name="R_CC2"
      resistance="5.1k"
      footprint="0402"
      pcbX={3}
      pcbY={-9.5}
    />
    <capacitor
      name="C_VBUS"
      capacitance="10uF"
      footprint="0603"
      pcbX={0}
      pcbY={-9.5}
    />
    <chip
      name="U_CHG"
      footprint="soic8"
      pcbX={-11}
      pcbY={-7}
      pinLabels={{
        pin1: "TEMP",
        pin2: "PROG",
        pin3: "GND",
        pin4: "VCC",
        pin5: "BAT",
        pin6: "STDBY",
        pin7: "CHRG",
        pin8: "CE",
      }}
      pinAttributes={{
        VCC: { requiresPower: true },
        GND: { requiresGround: true },
      }}
    />
    <resistor
      name="R_PROG"
      resistance="2k"
      footprint="0402"
      pcbX={-15}
      pcbY={-7}
    />
    <capacitor
      name="C_BAT"
      capacitance="10uF"
      footprint="0603"
      pcbX={-11}
      pcbY={-3}
    />
    <connector
      name="J_BAT"
      standard="jst_ph"
      pinCount={2}
      pcbX={-15}
      pcbY={-3}
    />
    <chip
      name="U_LDO"
      footprint="sot23_5"
      pcbX={-6}
      pcbY={-6}
      pinLabels={{
        pin1: "VIN",
        pin2: "GND",
        pin3: "EN",
        pin4: "NC",
        pin5: "VOUT",
      }}
      pinAttributes={{
        VIN: { requiresPower: true },
        GND: { requiresGround: true },
      }}
    />
    <capacitor
      name="C_LDO_IN"
      capacitance="4.7uF"
      footprint="0603"
      pcbX={-6}
      pcbY={-9.5}
    />
    <capacitor
      name="C_LDO_OUT"
      capacitance="10uF"
      footprint="0805"
      pcbX={-2}
      pcbY={-6}
    />
    <trace from=".J_USB > .VBUS" to="net.VBUS" />
    <trace from=".J_USB > .GND" to="net.GND" />
    <trace from=".J_USB > .CC1" to=".R_CC1 > .pin1" />
    <trace from=".R_CC1 > .pin2" to="net.GND" />
    <trace from=".J_USB > .CC2" to=".R_CC2 > .pin1" />
    <trace from=".R_CC2 > .pin2" to="net.GND" />
    <trace from=".C_VBUS > .pin1" to="net.VBUS" />
    <trace from=".C_VBUS > .pin2" to="net.GND" />
    <trace from=".U_CHG > .VCC" to="net.VBUS" />
    <trace from=".U_CHG > .CE" to="net.VBUS" />
    <trace from=".U_CHG > .GND" to="net.GND" />
    <trace from=".U_CHG > .TEMP" to="net.GND" />
    <trace from=".U_CHG > .PROG" to=".R_PROG > .pin1" />
    <trace from=".R_PROG > .pin2" to="net.GND" />
    <trace from=".U_CHG > .BAT" to="net.VBAT" />
    <trace from=".C_BAT > .pin1" to="net.VBAT" />
    <trace from=".C_BAT > .pin2" to="net.GND" />
    <trace from=".J_BAT > .pin1" to="net.VBAT" />
    <trace from=".J_BAT > .pin2" to="net.GND" />
    <trace from=".U_LDO > .VIN" to="net.VBAT" />
    <trace from=".U_LDO > .EN" to="net.VBAT" />
    <trace from=".U_LDO > .GND" to="net.GND" />
    <trace from=".C_LDO_IN > .pin1" to="net.VBAT" />
    <trace from=".C_LDO_IN > .pin2" to="net.GND" />
    <trace from=".U_LDO > .VOUT" to="net.VDD3V3" />
    <trace from=".C_LDO_OUT > .pin1" to="net.VDD3V3" />
    <trace from=".C_LDO_OUT > .pin2" to="net.GND" />
  </>
);
