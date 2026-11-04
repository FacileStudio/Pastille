/// Size of an SNTP request and of the part of the reply the device reads.
pub const SNTP_PACKET_BYTES: usize = 48;
/// Seconds from 1900-01-01 (NTP era 0) to 1970-01-01.
const NTP_TO_UNIX_S: u64 = 2_208_988_800;

/// An SNTP v4 client request: LI 0, version 4, mode 3 (client).
pub fn sntp_request() -> [u8; SNTP_PACKET_BYTES] {
    let mut packet = [0u8; SNTP_PACKET_BYTES];
    packet[0] = 0x23;
    packet
}

/// Reads the server transmit time from a reply, in Unix milliseconds. Rejects anything but a
/// synchronized server reply (mode 4, stratum 1 to 15, leap indicator not 3). Timestamps with the
/// top bit clear are read as NTP era 1 (after 2036-02-07), so the range is 1968 to 2104.
pub fn parse_sntp_reply(reply: &[u8]) -> Option<u64> {
    let reply = reply.get(..SNTP_PACKET_BYTES)?;
    let (leap, mode, stratum) = (reply[0] >> 6, reply[0] & 0x07, reply[1]);
    if leap == 3 || mode != 4 || !(1..=15).contains(&stratum) {
        return None;
    }
    let raw = u64::from(u32::from_be_bytes(reply[40..44].try_into().ok()?));
    let seconds = if raw < 0x8000_0000 { raw + (1 << 32) } else { raw };
    let fraction = u64::from(u32::from_be_bytes(reply[44..48].try_into().ok()?));
    let unix_s = seconds.checked_sub(NTP_TO_UNIX_S)?;
    Some(unix_s * 1000 + ((fraction * 1000) >> 32))
}
