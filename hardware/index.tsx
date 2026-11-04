import { Mcu } from "./subcircuits/mcu";
import { Power } from "./subcircuits/power";
import { Audio } from "./subcircuits/audio";
import { Display } from "./subcircuits/display";
import { Trigger } from "./subcircuits/trigger";

export default () => (
  <board
    width="38mm"
    height="38mm"
    layers={4}
    routeRemaining={true}
    autorouterVersion="beta_pipeline9"
  >
    <Mcu />
    <Power />
    <Audio />
    <Display />
    <Trigger />

    <trace from=".U_ESP32_S3 > .GPIO4" to="net.MIC_SCK" />
    <trace from=".U_ESP32_S3 > .GPIO5" to="net.MIC_WS" />
    <trace from=".U_ESP32_S3 > .GPIO6" to="net.MIC_SD" />
    <trace from=".U_ESP32_S3 > .GPIO7" to="net.TOUCH_WAKE" />
    <trace from=".U_ESP32_S3 > .GPIO10" to="net.LCD_CS" />
    <trace from=".U_ESP32_S3 > .GPIO11" to="net.LCD_DC" />
    <trace from=".U_ESP32_S3 > .GPIO12" to="net.LCD_SCLK" />
    <trace from=".U_ESP32_S3 > .GPIO13" to="net.LCD_MOSI" />
    <trace from=".U_ESP32_S3 > .GPIO14" to="net.LCD_RST" />
    <trace from=".U_ESP32_S3 > .GPIO15" to="net.SPK_BCLK" />
    <trace from=".U_ESP32_S3 > .GPIO16" to="net.SPK_LRCLK" />
    <trace from=".U_ESP32_S3 > .GPIO17" to="net.SPK_DIN" />
    <trace from=".U_ESP32_S3 > .GPIO18" to="net.SPK_SD_MODE" />
    <trace from=".U_ESP32_S3 > .GPIO21" to="net.LCD_BL" />

    <copperpour layer="bottom" connectsTo="net.GND" />
  </board>
);
