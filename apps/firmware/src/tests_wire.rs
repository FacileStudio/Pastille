use crate::ble::gatt::{
    BLE_CHAR_AUDIO_UUID, BLE_CHAR_CONTROL_UUID, BLE_CHAR_INFO_UUID, BLE_CHAR_PROV_UUID, BLE_SERVICE_UUID,
};
use crate::ble::{ProvisionConfig, TransportKind};
use crate::power::{PowerManager, PowerState};
use crate::protocol::ProtocolError;
use crate::stream::{encode_stream_frame, StreamReassembler};
use crate::MAX_PACKET_BYTES;

const TOKEN: &str = "bohs_tok_0123456789abcdef0123456789abcdef";

#[test]
fn test_stream_reassembly() {
    let frame: Vec<u8> = (0..644u32).map(|i| i as u8).collect();
    let mut wire = [0u8; 2 * (MAX_PACKET_BYTES + 2)];
    let first = encode_stream_frame(&frame, &mut wire).unwrap();
    let total = first + encode_stream_frame(&frame[..3], &mut wire[first..]).unwrap();
    assert_eq!(&wire[..2], &644u16.to_be_bytes());

    let mut reassembler = StreamReassembler::new();
    let mut frames: Vec<Vec<u8>> = Vec::new();
    for chunk in wire[..total].chunks(244) {
        let mut rest = chunk;
        while !rest.is_empty() {
            if let Some(done) = reassembler.push(&mut rest).unwrap() {
                frames.push(done.to_vec());
            }
        }
    }
    assert_eq!(frames, [frame.clone(), frame[..3].to_vec()]);

    let mut oversized: &[u8] = &[0x04, 0x01, 0xAA, 0x00, 0x01, 0x02];
    assert_eq!(reassembler.push(&mut oversized), Err(ProtocolError::InvalidFrameLength));
    assert!(oversized.is_empty());
    let mut zero: &[u8] = &[0x00, 0x00];
    assert_eq!(reassembler.push(&mut zero), Err(ProtocolError::InvalidFrameLength));
    let mut resync: &[u8] = &[0x00, 0x01, 0x02];
    assert_eq!(reassembler.push(&mut resync), Ok(None));
    assert_eq!(reassembler.push(&mut resync), Ok(Some(&[0x02][..])));
    assert_eq!(encode_stream_frame(&[], &mut wire), Err(ProtocolError::InvalidFrameLength));
    assert_eq!(encode_stream_frame(&[0; MAX_PACKET_BYTES + 1], &mut wire), Err(ProtocolError::InvalidFrameLength));
}

#[test]
fn test_ble_uuids_and_power_fast_connect() {
    let chars = [BLE_CHAR_INFO_UUID, BLE_CHAR_AUDIO_UUID, BLE_CHAR_CONTROL_UUID, BLE_CHAR_PROV_UUID];
    for (n, uuid) in (1u128..).zip(chars) {
        assert_eq!(uuid, BLE_SERVICE_UUID + (n << 96));
    }

    let mut pm = PowerManager::new();
    pm.on_wake_trigger();
    pm.on_hardware_ready(TransportKind::BleRelay);
    assert_eq!(pm.state, PowerState::BleFastConnect);
    pm.on_ble_connected();
    assert_eq!((pm.state, pm.silence_ms), (PowerState::StreamFullDuplex, 0));
}

#[test]
fn test_config_debug_redacts_secrets() {
    let config = ProvisionConfig {
        gateway_url: "wss://g".try_into().unwrap(),
        device_token: TOKEN.try_into().unwrap(),
        wifi_ssid: "Atelier".try_into().unwrap(),
        wifi_psk: "hunter22".try_into().unwrap(),
        prefer_ble: false,
    };
    let printed = format!("{:?}", config);
    assert!(printed.contains("wss://g") && printed.contains("<redacted>"));
    assert!(!printed.contains(TOKEN) && !printed.contains("hunter22"));
}
