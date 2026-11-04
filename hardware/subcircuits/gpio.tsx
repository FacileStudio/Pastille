const GPIO_NETS: [number, string][] = [
  [1, "BATT_SENSE"],
  [2, "POWER_CHG_STAT"],
  [4, "MIC_SCK"],
  [5, "MIC_WS"],
  [6, "MIC_SD"],
  [7, "TOUCH_WAKE"],
  [8, "I2C_SDA"],
  [9, "I2C_SCL"],
  [10, "LCD_CS"],
  [11, "LCD_DC"],
  [12, "LCD_SCLK"],
  [13, "LCD_MOSI"],
  [14, "LCD_RST"],
  [15, "SPK_BCLK"],
  [16, "SPK_LRCLK"],
  [17, "SPK_DIN"],
  [18, "SPK_SD_MODE"],
  [19, "USB_DM"],
  [20, "USB_DP"],
  [21, "LCD_BL"],
];

const GpioTrace = ({ gpio, net }: { gpio: number; net: string }) => (
  <trace from={`.U_ESP32_S3 > .GPIO${gpio}`} to={`net.${net}`} />
);

/** Every MCU signal net. The GPIO numbers are the source of `apps/firmware/src/pins.rs`. */
export const GpioNets = () => (
  <>
    {GPIO_NETS.map(([gpio, net]) => (
      <GpioTrace key={net} gpio={gpio} net={net} />
    ))}
  </>
);
