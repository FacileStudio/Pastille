export const USB_Y_MM = -12.7;

const PHANTOM_PINS = Array.from({ length: 12 }, (_, i) => [`pin${i + 1}`, { doNotConnect: true }]);

const USB_PIN_ATTRIBUTES = {
  SBU1: { doNotConnect: true },
  SBU2: { doNotConnect: true },
  ...Object.fromEntries(PHANTOM_PINS),
};

export const Usb = () => (
  <>
    <connector
      name="J_USB"
      standard="usb_c"
      manufacturerPartNumber="TYPE-C 16PIN 2MD(073)"
      supplierPartNumbers={{ jlcpcb: ["C2765186"] }}
      pcbX={0}
      pcbY={USB_Y_MM}
      schX={-50}
      schY={18}
      pinAttributes={USB_PIN_ATTRIBUTES}
    />
    <diode name="D_TVS" footprint="smf" manufacturerPartNumber="SMF5.0A" pcbX={6.6} pcbY={-13.4} pcbRotation={90}
      schX={-44} schY={18} />
    <resistor name="R_CC1" resistance="5.1k" footprint="0402" manufacturerPartNumber="RC0402FR-075K1L"
      pcbX={-1.4} pcbY={-8.9} schX={-50} schY={10} />
    <resistor name="R_CC2" resistance="5.1k" footprint="0402" manufacturerPartNumber="RC0402FR-075K1L"
      pcbX={2.2} pcbY={-8.9} schX={-44} schY={10} />
    <chip
      name="U_ESD"
      footprint="sot23_6"
      manufacturerPartNumber="USBLC6-2SC6"
      supplierPartNumbers={{ jlcpcb: ["C7519"] }}
      pcbX={7.2}
      pcbY={-8.6}
      pcbRotation={90}
      schX={-44}
      schY={4}
      pinLabels={{ pin1: "IO1_A", pin2: "GND", pin3: "IO2_A", pin4: "IO2_B", pin5: "VBUS", pin6: "IO1_B" }}
      pinAttributes={{ VBUS: { requiresPower: true }, GND: { requiresGround: true } }}
    />
    <trace from=".J_USB > .VBUS1" to="net.VBUS" />
    <trace from=".J_USB > .VBUS2" to="net.VBUS" />
    <trace from=".J_USB > .GND1" to="net.GND" />
    <trace from=".J_USB > .GND2" to="net.GND" />
    <trace from=".J_USB > .SHELL1" to="net.GND" />
    <trace from=".J_USB > .SHELL2" to="net.GND" />
    <trace from=".D_TVS > .cathode" to="net.VBUS" />
    <trace from=".D_TVS > .anode" to="net.GND" />
    <trace from=".J_USB > .CC1" to=".R_CC1 > .pin1" />
    <trace from=".R_CC1 > .pin2" to="net.GND" />
    <trace from=".J_USB > .CC2" to=".R_CC2 > .pin1" />
    <trace from=".R_CC2 > .pin2" to="net.GND" />
    <trace from=".J_USB > .DM1" to="net.USB_DM" />
    <trace from=".J_USB > .DM2" to="net.USB_DM" />
    <trace from=".J_USB > .DP1" to="net.USB_DP" />
    <trace from=".J_USB > .DP2" to="net.USB_DP" />
    <trace from=".U_ESD > .IO1_A" to="net.USB_DP" />
    <trace from=".U_ESD > .IO1_B" to="net.USB_DP" />
    <trace from=".U_ESD > .IO2_A" to="net.USB_DM" />
    <trace from=".U_ESD > .IO2_B" to="net.USB_DM" />
    <trace from=".U_ESD > .VBUS" to="net.VBUS" />
    <trace from=".U_ESD > .GND" to="net.GND" />
  </>
);
