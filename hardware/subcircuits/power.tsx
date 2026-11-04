export const Power = () => (
  <>
    <chip
      name="U_CHG"
      footprint="soic8_thermalpad2.4x3.1mm"
      manufacturerPartNumber="TP4056-42-ESOP8"
      supplierPartNumbers={{ jlcpcb: ["C16581"] }}
      pcbX={12.2}
      pcbY={-5.85}
      schX={-32}
      schY={18}
      pinLabels={{
        pin1: "TEMP",
        pin2: "PROG",
        pin3: "GND",
        pin4: "VCC",
        pin5: "BAT",
        pin6: "STDBY",
        pin7: "CHRG",
        pin8: "CE",
        pin9: "thermalpad",
      }}
      pinAttributes={{
        VCC: { requiresPower: true },
        GND: { requiresGround: true },
        STDBY: { doNotConnect: true },
      }}
    />
    <capacitor name="C_VBUS" capacitance="10uF" footprint="0402" manufacturerPartNumber="GRM155R60J106ME15D"
      pcbX={7.4} pcbY={-5.6} schX={-38} schY={18} />
    <resistor name="R_PROG" resistance="18k" footprint="0402" manufacturerPartNumber="RC0402FR-0718KL"
      pcbX={7.4} pcbY={-4.0} schX={-32} schY={10} />
    <resistor name="R_CHG_PU" resistance="10k" footprint="0402" manufacturerPartNumber="RC0402FR-0710KL"
      pcbX={-9.2} pcbY={-9.0} schX={-32} schY={4} />
    <capacitor name="C_BAT" capacitance="10uF" footprint="0402" manufacturerPartNumber="GRM155R60J106ME15D"
      pcbX={12.0} pcbY={7.3} schX={-26} schY={18} />
    <connector
      name="J_BAT"
      standard="jst_sh"
      pinCount={2}
      manufacturerPartNumber="SM02B-SRSS-TB(LF)(SN)"
      supplierPartNumbers={{ jlcpcb: ["C160402"] }}
      pcbX={13.6}
      pcbY={4.05}
      pcbRotation={90}
      schX={-26}
      schY={10}
    />
    <resistor name="R_BATT_TOP" resistance="1M" footprint="0402" manufacturerPartNumber="RC0402FR-071ML"
      pcbX={-5.65} pcbY={-13.0} pcbRotation={90} schX={-26} schY={4} />
    <resistor name="R_BATT_BOT" resistance="1M" footprint="0402" manufacturerPartNumber="RC0402FR-071ML"
      pcbX={-5.65} pcbY={-15.1} pcbRotation={90} schX={-26} schY={-2} />
    <capacitor name="C_BATT_SENSE" capacitance="100nF" footprint="0402" manufacturerPartNumber="GRM155R71C104KA88D"
      pcbX={-5.65} pcbY={-10.9} pcbRotation={90} schX={-20} schY={-2} />
    <diode name="D_SYS" footprint="sod323" manufacturerPartNumber="1N5819WS" pcbX={8.6} pcbY={1.5}
      schX={-20} schY={18} />
    <chip
      name="Q_PATH"
      footprint="sot23"
      manufacturerPartNumber="AO3401A"
      supplierPartNumbers={{ jlcpcb: ["C15127"] }}
      pinLabels={{ pin1: "G", pin2: "S", pin3: "D" }}
      pcbX={8.6}
      pcbY={-1.3}
      schX={-20}
      schY={12}
    />
    <resistor name="R_PATH_G" resistance="100k" footprint="0402" manufacturerPartNumber="RC0402FR-07100KL"
      pcbX={5.2} pcbY={-1.2} pcbRotation={90} schX={-14} schY={12} />
    <capacitor name="C_SYS" capacitance="4.7uF" footprint="0603" manufacturerPartNumber="GRM188R61A475KE15D"
      pcbX={5.4} pcbY={1.4} pcbRotation={90} schX={-20} schY={6} />
    <chip
      name="U_LDO"
      footprint="sot23_5"
      manufacturerPartNumber="RT9080-33GJ5"
      supplierPartNumbers={{ jlcpcb: ["C841192"] }}
      pcbX={8.4}
      pcbY={4.3}
      schX={-14}
      schY={18}
      pinLabels={{ pin1: "VIN", pin2: "GND", pin3: "EN", pin4: "NC", pin5: "VOUT" }}
      pinAttributes={{ VIN: { requiresPower: true }, GND: { requiresGround: true }, NC: { doNotConnect: true } }}
    />
    <capacitor name="C_LDO_OUT" capacitance="10uF" footprint="0603" manufacturerPartNumber="GRM188R61A106KE69D"
      pcbX={5.4} pcbY={4.6} pcbRotation={90} schX={-14} schY={12} />
  </>
);
