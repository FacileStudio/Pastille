import { Mcu } from "./subcircuits/mcu";
import { Usb, USB_Y_MM } from "./subcircuits/usb";
import { Power } from "./subcircuits/power";
import { PowerNets } from "./subcircuits/power-nets";
import { Audio } from "./subcircuits/audio";
import { Display } from "./subcircuits/display";
import { Trigger } from "./subcircuits/trigger";
import { GpioNets } from "./subcircuits/gpio";

const BOARD_RADIUS_MM = 19;
const USB_FRONT_MM = 5.857;
const FLAT_Y_MM = USB_Y_MM - USB_FRONT_MM;
const FLAT_X_MM = Math.sqrt(BOARD_RADIUS_MM ** 2 - FLAT_Y_MM ** 2);
const ALL_LAYERS: ("top" | "inner1" | "inner2" | "bottom")[] = ["top", "inner1", "inner2", "bottom"];
const round3 = (v: number) => Number(v.toFixed(3));
const angleOf = (p: { x: number; y: number }) => (Math.atan2(p.y, p.x) + 2 * Math.PI) % (2 * Math.PI);
const CIRCLE = Array.from({ length: 64 }, (_, i) => {
  const angle = (2 * Math.PI * i) / 64;
  return { x: round3(BOARD_RADIUS_MM * Math.cos(angle)), y: round3(BOARD_RADIUS_MM * Math.sin(angle)) };
});
const BOARD_OUTLINE = [
  ...CIRCLE.filter((p) => p.y > FLAT_Y_MM + 0.05),
  { x: round3(-FLAT_X_MM), y: round3(FLAT_Y_MM) },
  { x: round3(FLAT_X_MM), y: round3(FLAT_Y_MM) },
].sort((a, b) => angleOf(a) - angleOf(b));

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
    pcbStyle={{ viaPadDiameter: "0.4mm", viaHoleDiameter: "0.2mm" }}
  >
    <net name="GND" nominalTraceWidth="0.25mm" />
    <net name="VBUS" nominalTraceWidth="0.3mm" />
    <net name="VBAT" nominalTraceWidth="0.4mm" />
    <net name="VSYS" nominalTraceWidth="0.3mm" />
    <net name="VDD3V3" nominalTraceWidth="0.3mm" />
    <Mcu />
    <Usb />
    <Power />
    <PowerNets />
    <Audio />
    <Display />
    <Trigger />
    <GpioNets />

    <keepout
      shape="rect"
      pcbX={-15.1}
      pcbY={0}
      width="8.2mm"
      height="17mm"
      layers={ALL_LAYERS}
      excludeRefs={[".U_ESP32_S3"]}
    />
    <hole diameter="2.2mm" pcbX={-14.2} pcbY={-10.0} />
    <hole diameter="2.2mm" pcbX={15.1} pcbY={8.6} />
    <fiducial name="FID1" padDiameter="1mm" soldermaskPullback="0.5mm" pcbX={-11.6} pcbY={11.6} />
    <fiducial name="FID2" padDiameter="1mm" soldermaskPullback="0.5mm" pcbX={12.4} pcbY={10.6} />
    <silkscreentext text="PASTILLE BOHS-01" layer="top" fontSize="0.8mm" pcbX={-1.6} pcbY={17.0} />
    <silkscreentext text="REV 0.4.0" layer="top" fontSize="0.7mm" pcbX={-1.6} pcbY={16.0} />

    <copperpour layer="bottom" connectsTo="net.GND" boardEdgeMargin="0.3mm" />
  </board>
);
