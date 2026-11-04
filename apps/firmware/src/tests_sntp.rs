use crate::clock::{set_wall_time, TIME_FLOOR_UNIX_S};
use crate::sntp::{parse_sntp_reply, sntp_request, SNTP_PACKET_BYTES};

fn reply(ntp_seconds: u32, fraction: u32, mode: u8, stratum: u8) -> [u8; SNTP_PACKET_BYTES] {
    let mut packet = [0u8; SNTP_PACKET_BYTES];
    packet[0] = 0x20 | mode;
    packet[1] = stratum;
    packet[40..44].copy_from_slice(&ntp_seconds.to_be_bytes());
    packet[44..48].copy_from_slice(&fraction.to_be_bytes());
    packet
}

#[test]
fn test_sntp_request_and_reply_parsing() {
    assert_eq!(sntp_request()[0], 0x23);
    let unix = 1_793_500_000u64;
    let ntp = (unix + 2_208_988_800) as u32;
    assert_eq!(parse_sntp_reply(&reply(ntp, 0x8000_0000, 4, 2)), Some(unix * 1000 + 500));
    let after_2036 = 2_200_000_000u64;
    let era_one = (after_2036 + 2_208_988_800 - (1 << 32)) as u32;
    assert_eq!(parse_sntp_reply(&reply(era_one, 0, 4, 2)), Some(after_2036 * 1000));
    assert_eq!(parse_sntp_reply(&reply(ntp, 0, 3, 2)), None);
    assert_eq!(parse_sntp_reply(&reply(ntp, 0, 4, 0)), None);
    let mut alarm = reply(ntp, 0, 4, 2);
    alarm[0] |= 0xC0;
    assert_eq!(parse_sntp_reply(&alarm), None);
    assert_eq!(parse_sntp_reply(&[0u8; 20]), None);
}

#[test]
fn test_wall_clock_floor() {
    assert!(!set_wall_time((TIME_FLOOR_UNIX_S - 1) * 1000, 0));
    assert!(!set_wall_time(TIME_FLOOR_UNIX_S * 1000, TIME_FLOOR_UNIX_S * 1000 + 1));
}
