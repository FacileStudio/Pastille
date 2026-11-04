import { Mcu } from "./subcircuits/mcu";
import { Power } from "./subcircuits/power";
import { Audio } from "./subcircuits/audio";
import { Display } from "./subcircuits/display";
import { Trigger } from "./subcircuits/trigger";

const BOARD_RADIUS_MM = 19;
const BOARD_OUTLINE = Array.from({ length: 64 }, (_, i) => {
  const angle = (2 * Math.PI * i) / 64;
  return {
    x: Number((BOARD_RADIUS_MM * Math.cos(angle)).toFixed(3)),
    y: Number((BOARD_RADIUS_MM * Math.sin(angle)).toFixed(3)),
  };
});

export default () => (
  <board
    width="38mm"
    height="38mm"
    outline={BOARD_OUTLINE}
    solderMaskColor="black"
    silkscreenColor="white"
    layers={4}
    routeRemaining={true}
    autorouterVersion="beta_pipeline9"
  >
    <Mcu />
    <Power />
    <Audio />
    <Display />
    <Trigger />
    <connector
      name="J_USB"
      standard="usb_c"
      pcbX={0}
      pcbY={-13.0}
      schX={-50}
      schY={18}
    />

    <trace from=".U_ESP32_S3 > .GPIO1" to="net.BATT_SENSE" />
    <trace from=".U_ESP32_S3 > .GPIO2" to="net.POWER_CHG_STAT" />
    <trace from=".U_ESP32_S3 > .GPIO4" to="net.MIC_SCK" />
    <trace from=".U_ESP32_S3 > .GPIO5" to="net.MIC_WS" />
    <trace from=".U_ESP32_S3 > .GPIO6" to="net.MIC_SD" />
    <trace from=".U_ESP32_S3 > .GPIO7" to="net.TOUCH_WAKE" />
    <trace from=".U_ESP32_S3 > .GPIO8" to="net.I2C_SDA" />
    <trace from=".U_ESP32_S3 > .GPIO9" to="net.I2C_SCL" />
    <trace from=".U_ESP32_S3 > .GPIO10" to="net.LCD_CS" />
    <trace from=".U_ESP32_S3 > .GPIO11" to="net.LCD_DC" />
    <trace from=".U_ESP32_S3 > .GPIO12" to="net.LCD_SCLK" />
    <trace from=".U_ESP32_S3 > .GPIO13" to="net.LCD_MOSI" />
    <trace from=".U_ESP32_S3 > .GPIO14" to="net.LCD_RST" />
    <trace from=".U_ESP32_S3 > .GPIO15" to="net.SPK_BCLK" />
    <trace from=".U_ESP32_S3 > .GPIO16" to="net.SPK_LRCLK" />
    <trace from=".U_ESP32_S3 > .GPIO17" to="net.SPK_DIN" />
    <trace from=".U_ESP32_S3 > .GPIO18" to="net.SPK_SD_MODE" />
    <trace from=".U_ESP32_S3 > .GPIO19" to="net.USB_DM" />
    <trace from=".U_ESP32_S3 > .GPIO20" to="net.USB_DP" />
    <trace from=".U_ESP32_S3 > .GPIO21" to="net.LCD_BL" />

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
    <trace from=".J_USB > .DM1" to=".R_USB_DM > .pin1" />
    <trace from=".J_USB > .DM2" to=".R_USB_DM > .pin1" />
    <trace from=".R_USB_DM > .pin2" to="net.USB_DM" />
    <trace from=".J_USB > .DP1" to=".R_USB_DP > .pin1" />
    <trace from=".J_USB > .DP2" to=".R_USB_DP > .pin1" />
    <trace from=".R_USB_DP > .pin2" to="net.USB_DP" />
    <trace from=".C_VBUS > .pin1" to="net.VBUS" />
    <trace from=".C_VBUS > .pin2" to="net.GND" />
    <trace from=".C_VBUS_HF > .pin1" to="net.VBUS" />
    <trace from=".C_VBUS_HF > .pin2" to="net.GND" />
    <trace from=".U_CHG > .VCC" to="net.VBUS" />
    <trace from=".U_CHG > .CE" to="net.VBUS" />
    <trace from=".U_CHG > .GND" to="net.GND" />
    <trace from=".U_CHG > .TEMP" to="net.GND" />
    <trace from=".U_CHG > .PROG" to=".R_PROG > .pin1" />
    <trace from=".R_PROG > .pin2" to="net.GND" />
    <trace from=".U_CHG > .CHRG" to="net.POWER_CHG_STAT" />
    <trace from=".R_CHG_PU > .pin1" to="net.POWER_CHG_STAT" />
    <trace from=".R_CHG_PU > .pin2" to="net.VDD3V3" />
    <trace from=".U_CHG > .BAT" to="net.VBAT" />
    <trace from=".C_BAT > .pin1" to="net.VBAT" />
    <trace from=".C_BAT > .pin2" to="net.GND" />
    <trace from=".J_BAT > .pin1" to="net.VBAT" />
    <trace from=".J_BAT > .pin2" to="net.GND" />
    <trace from=".R_BATT_TOP > .pin1" to="net.VBAT" />
    <trace from=".R_BATT_TOP > .pin2" to="net.BATT_SENSE" />
    <trace from=".R_BATT_BOT > .pin1" to="net.BATT_SENSE" />
    <trace from=".R_BATT_BOT > .pin2" to="net.GND" />
    <trace from=".C_BATT_SENSE > .pin1" to="net.BATT_SENSE" />
    <trace from=".C_BATT_SENSE > .pin2" to="net.GND" />
    <trace from=".D_SYS > .anode" to="net.VBUS" />
    <trace from=".D_SYS > .cathode" to="net.VSYS" />
    <trace from=".D_PATH > .anode" to="net.VBAT" />
    <trace from=".D_PATH > .cathode" to="net.VSYS" />
    <trace from=".C_SYS > .pin1" to="net.VSYS" />
    <trace from=".C_SYS > .pin2" to="net.GND" />
    <trace from=".C_SYS_HF > .pin1" to="net.VSYS" />
    <trace from=".C_SYS_HF > .pin2" to="net.GND" />
    <trace from=".U_LDO > .VIN" to="net.VSYS" />
    <trace from=".U_LDO > .EN" to="net.VSYS" />
    <trace from=".U_LDO > .GND" to="net.GND" />
    <trace from=".U_LDO > .VOUT" to="net.VDD3V3" />
    <trace from=".C_LDO_OUT > .pin1" to="net.VDD3V3" />
    <trace from=".C_LDO_OUT > .pin2" to="net.GND" />
    <trace from=".C_LDO_OUT_HF > .pin1" to="net.VDD3V3" />
    <trace from=".C_LDO_OUT_HF > .pin2" to="net.GND" />

    <hole diameter="2.2mm" pcbX={-11.5} pcbY={12.8} />
    <hole diameter="2.2mm" pcbX={11.5} pcbY={12.8} />
    <fiducial name="FID1" padDiameter="1mm" soldermaskPullback="0.5mm" pcbX={-16.5} pcbY={-5.5} />
    <fiducial name="FID2" padDiameter="1mm" soldermaskPullback="0.5mm" pcbX={16.5} pcbY={3.0} />
    <fiducial name="FID3" padDiameter="1mm" soldermaskPullback="0.5mm" pcbX={0} pcbY={16.8} />
    <silkscreentext text="PASTILLE BOHS-01" layer="top" fontSize="1mm" pcbX={0} pcbY={10.4} />
    <silkscreentext text="REV 0.2.2" layer="top" fontSize="0.75mm" pcbX={0} pcbY={8.8} />

    <copperpour layer="bottom" connectsTo="net.GND" />
  </board>
);
