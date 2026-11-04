const LINKS: [string, string][] = [
  [".C_VBUS > .pin1", "VBUS"],
  [".C_VBUS > .pin2", "GND"],
  [".U_CHG > .VCC", "VBUS"],
  [".U_CHG > .CE", "VBUS"],
  [".U_CHG > .GND", "GND"],
  [".U_CHG > .thermalpad", "GND"],
  [".U_CHG > .TEMP", "GND"],
  [".R_PROG > .pin2", "GND"],
  [".U_CHG > .CHRG", "POWER_CHG_STAT"],
  [".R_CHG_PU > .pin1", "POWER_CHG_STAT"],
  [".R_CHG_PU > .pin2", "VDD3V3"],
  [".U_CHG > .BAT", "VBAT"],
  [".C_BAT > .pin1", "VBAT"],
  [".C_BAT > .pin2", "GND"],
  [".J_BAT > .pin1", "VBAT"],
  [".J_BAT > .pin2", "GND"],
  [".R_BATT_TOP > .pin1", "VBAT"],
  [".R_BATT_TOP > .pin2", "BATT_SENSE"],
  [".R_BATT_BOT > .pin1", "BATT_SENSE"],
  [".R_BATT_BOT > .pin2", "GND"],
  [".C_BATT_SENSE > .pin1", "BATT_SENSE"],
  [".C_BATT_SENSE > .pin2", "GND"],
  [".D_SYS > .anode", "VBUS"],
  [".D_SYS > .cathode", "VSYS"],
  [".Q_PATH > .D", "VBAT"],
  [".Q_PATH > .S", "VSYS"],
  [".Q_PATH > .G", "VBUS"],
  [".R_PATH_G > .pin1", "VBUS"],
  [".R_PATH_G > .pin2", "GND"],
  [".C_SYS > .pin1", "VSYS"],
  [".C_SYS > .pin2", "GND"],
  [".U_LDO > .VIN", "VSYS"],
  [".U_LDO > .EN", "VSYS"],
  [".U_LDO > .GND", "GND"],
  [".U_LDO > .VOUT", "VDD3V3"],
  [".C_LDO_OUT > .pin1", "VDD3V3"],
  [".C_LDO_OUT > .pin2", "GND"],
];

const Link = ({ from, net }: { from: string; net: string }) => <trace from={from} to={`net.${net}`} />;

/** The charger, power path, battery sense and 3.3 V regulator nets. */
export const PowerNets = () => (
  <>
    {LINKS.map(([from, net]) => (
      <Link key={from} from={from} net={net} />
    ))}
    <trace from=".U_CHG > .PROG" to=".R_PROG > .pin1" />
  </>
);
