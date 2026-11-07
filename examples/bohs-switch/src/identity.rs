const SOURCES: [&str; 3] = [
    "/sys/firmware/devicetree/base/serial-number",
    "/proc/cpuinfo",
    "/etc/machine-id",
];

/// Answers the 8 lowercase hex characters the gateway knows this board by: the end of the
/// Raspberry Pi serial number, which survives a reflash, or of the machine id elsewhere.
pub fn hardware_id() -> Result<String, String> {
    SOURCES
        .iter()
        .filter_map(|path| std::fs::read_to_string(path).ok())
        .find_map(|text| from_text(&text))
        .ok_or_else(|| "no serial number or machine id to derive a hardware id from".into())
}

/// Answers the last 8 hex digits of the serial in `text`: a bare serial, or the `Serial` line
/// of `/proc/cpuinfo`.
pub fn from_text(text: &str) -> Option<String> {
    let serial = text
        .lines()
        .find_map(|line| line.strip_prefix("Serial").and_then(|rest| rest.split(':').nth(1)))
        .unwrap_or(text);
    let hex: String = serial
        .chars()
        .filter(char::is_ascii_hexdigit)
        .collect::<String>()
        .to_lowercase();
    if hex.len() < 8 || hex.chars().all(|c| c == '0') {
        return None;
    }
    Some(hex[hex.len() - 8..].to_string())
}
