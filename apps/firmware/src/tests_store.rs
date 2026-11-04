use crate::ble::{ProvisionConfig, ProvisionState, TransportKind};
use crate::ble_link::{firmware_version, info_record};
use crate::link::DeviceInfo;
use crate::store::{crc32, decode_record, encode_record, newest, Stored, BOND_BYTES, RECORD_BYTES};

fn sample() -> Stored {
    Stored {
        config: Some(ProvisionConfig {
            gateway_url: "wss://bulle.facile.studio/api/hardware/ws".try_into().unwrap(),
            device_token: "bohs_tok_0123456789abcdef0123456789abcdef".try_into().unwrap(),
            wifi_ssid: "Atelier".try_into().unwrap(),
            wifi_psk: "correct horse battery staple".try_into().unwrap(),
            prefer_ble: true,
        }),
        bond: Some([0xA5; BOND_BYTES]),
    }
}

#[test]
fn test_crc32_matches_the_ieee_check_value() {
    assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
    assert_eq!(crc32(b""), 0);
}

#[test]
fn test_record_roundtrip_and_corruption() {
    let mut out = [0u8; RECORD_BYTES];
    let len = encode_record(7, &sample(), &mut out);
    assert_eq!(decode_record(&out[..len]), Some((7, sample())));
    let empty = Stored::default();
    let mut blank = [0u8; RECORD_BYTES];
    let blank_len = encode_record(1, &empty, &mut blank);
    assert_eq!(decode_record(&blank[..blank_len]), Some((1, empty)));

    let mut longest = sample();
    let cfg = longest.config.as_mut().unwrap();
    cfg.gateway_url = "x".repeat(96).as_str().try_into().unwrap();
    cfg.wifi_ssid = "s".repeat(32).as_str().try_into().unwrap();
    cfg.wifi_psk = "p".repeat(64).as_str().try_into().unwrap();
    let longest_len = encode_record(2, &longest, &mut out);
    assert!(longest_len <= RECORD_BYTES);
    assert_eq!(decode_record(&out[..longest_len]), Some((2, longest)));

    let len = encode_record(7, &sample(), &mut out);
    for flip in [0, 5, 12, len - 1] {
        let mut torn = out;
        torn[flip] ^= 0x40;
        assert_eq!(decode_record(&torn[..len]), None, "byte {flip}");
    }
    assert_eq!(decode_record(&[0xFF; RECORD_BYTES]), None);
    assert_eq!(decode_record(&out[..len - 1]), None);
}

#[test]
fn test_newest_slot_wins_across_wraparound() {
    let s = Stored::default();
    assert_eq!(newest(Some((3, s.clone())), Some((4, s.clone()))).map(|r| r.0), Some(4));
    assert_eq!(newest(Some((9, s.clone())), Some((2, s.clone()))).map(|r| r.0), Some(9));
    assert_eq!(newest(Some((u32::MAX, s.clone())), Some((0, s.clone()))).map(|r| r.0), Some(0));
    assert_eq!(newest(None, Some((5, s.clone()))).map(|r| r.0), Some(5));
    assert_eq!(newest(None, None), None);
}

#[test]
fn test_b001_info_record() {
    let info = DeviceInfo { state: ProvisionState::Authorized, transport: TransportKind::UsbSerial, battery_pct: 64 };
    let [major, minor, patch] = firmware_version();
    assert_eq!(info_record(&info), [0x01, 0x02, 0x0F, 64, major, minor, patch, 0x02]);
    let version = format!("{major}.{minor}.{patch}");
    assert_eq!(version, env!("CARGO_PKG_VERSION"));
}
