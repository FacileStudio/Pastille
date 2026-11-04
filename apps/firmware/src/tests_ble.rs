use crate::audio::{AudioFrame, BYTES_PER_FRAME};
use crate::avatar::AvatarRenderer;
use crate::ble::gatt::{
    BLE_CHAR_AUDIO_UUID, BLE_CHAR_CONTROL_UUID, BLE_CHAR_INFO_UUID, BLE_CHAR_PROV_UUID, BLE_SERVICE_UUID,
};
use crate::ble::{BleProvisioner, ProvisionConfig, ProvisionState, TransportKind, AUTH_WINDOW_MS};
use crate::power::{PowerManager, PowerState};
use crate::protocol::{ProtocolDispatcher, ProtocolError, OPCODE_CONTROL, OPCODE_PROVISION};
use crate::stream::{encode_stream_frame, StreamReassembler};
use crate::{handle_inbound_packet, AUDIO_RX_CHANNEL, MAX_PACKET_BYTES, OUTBOUND_PKT_CHANNEL};

const TOKEN: &str = "bohs_tok_0123456789abcdef0123456789abcdef";

fn configure_req(token: &str) -> std::string::String {
    format!(
        concat!(
            r#"{{"jsonrpc":"2.0","id":7,"method":"prov.configure","params":{{"#,
            r#""gateway_url":"wss://bulle.example/api/hardware/ws","device_token":"{}","prefer_ble":true}}}}"#
        ),
        token
    )
}

fn prov_call(prov: &mut BleProvisioner, req: &str) -> std::string::String {
    let mut out = [0u8; MAX_PACKET_BYTES];
    let len = prov.dispatch_provision_rpc(req, TransportKind::BleRelay, &mut out).unwrap();
    assert_eq!(out[0], OPCODE_PROVISION);
    std::string::String::from_utf8(out[1..len].to_vec()).unwrap()
}

#[test]
fn test_provisioner_rejects_unauthorized_configure() {
    let mut prov = BleProvisioner::new();
    assert_eq!(prov.state, ProvisionState::Unclaimed);
    assert_eq!(prov.advertisement_service_data([1, 2, 3, 4]), [0x00, 0x0F, 1, 2, 3, 4]);
    assert!(prov_call(&mut prov, &configure_req(TOKEN)).contains(r#""code":-32001,"message":"unauthorized""#));

    let cfg = ProvisionConfig {
        gateway_url: "wss://g".try_into().unwrap(),
        device_token: TOKEN.try_into().unwrap(),
        wifi_ssid: Default::default(),
        wifi_psk: Default::default(),
        prefer_ble: true,
    };
    assert_eq!(prov.apply_config(cfg.clone()), Err(ProtocolError::Unauthorized));

    prov_call(&mut prov, r#"{"jsonrpc":"2.0","id":1,"method":"prov.identify"}"#);
    prov.on_physical_tap();
    assert_eq!(prov.state, ProvisionState::Authorized);
    assert_eq!(prov.apply_config(cfg), Err(ProtocolError::Unauthorized));
    assert!(prov.config.is_none());
}

#[test]
fn test_provisioning_happy_path() {
    let mut prov = BleProvisioner::new();
    let identify = prov_call(&mut prov, r#"{"jsonrpc":"2.0","id":1,"method":"prov.identify"}"#);
    assert!(identify.contains(r#""state":"awaiting_tap""#));
    prov.on_physical_tap();
    prov.on_link_encrypted(true);

    let bad_token = &TOKEN[..TOKEN.len() - 1];
    assert!(prov_call(&mut prov, &configure_req(bad_token)).contains("-32602"));
    assert!(prov_call(&mut prov, &configure_req(&TOKEN.to_uppercase())).contains("-32602"));
    let no_pref = configure_req(TOKEN).replace(r#","prefer_ble":true"#, "");
    assert!(prov_call(&mut prov, &no_pref).contains("-32602"));

    let done = prov_call(&mut prov, &configure_req(TOKEN));
    assert_eq!(done, r#"{"jsonrpc":"2.0","id":7,"result":{"status":"provisioned"}}"#);
    let status = prov_call(&mut prov, r#"{"jsonrpc":"2.0","id":8,"method":"prov.status"}"#);
    assert!(status.contains(r#""state":"provisioned","transport":"ble","link_encrypted":true"#));
    assert_eq!(prov.state, ProvisionState::Provisioned);
    assert_eq!(prov.transport, TransportKind::BleRelay);
    assert_eq!(prov.config.as_ref().unwrap().device_token.as_str(), TOKEN);
    assert!(prov.config.as_ref().unwrap().wifi_psk.is_empty());
    assert!(!done.contains(TOKEN));
}

#[test]
fn test_auth_window_timeout_resets() {
    let mut prov = BleProvisioner::new();
    prov.on_identify_request();
    prov.tick(AUTH_WINDOW_MS - 1);
    assert_eq!(prov.state, ProvisionState::AwaitingTap);
    prov.tick(1);
    assert_eq!(prov.state, ProvisionState::Unclaimed);

    prov.on_identify_request();
    prov.on_physical_tap();
    prov.tick(AUTH_WINDOW_MS);
    assert_eq!(prov.state, ProvisionState::Unclaimed);
    assert_eq!(prov.auth_timer_ms, 0);
    prov.on_physical_tap();
    assert_eq!(prov.state, ProvisionState::Unclaimed);

    prov.on_identify_request();
    prov.on_physical_tap();
    prov.on_link_encrypted(false);
    assert_eq!(prov.state, ProvisionState::Unclaimed);
}

#[test]
fn test_stream_reassembly() {
    let frame: std::vec::Vec<u8> = (0..644u32).map(|i| i as u8).collect();
    let mut wire = [0u8; 2 * (MAX_PACKET_BYTES + 2)];
    let first = encode_stream_frame(&frame, &mut wire).unwrap();
    let total = first + encode_stream_frame(&frame[..3], &mut wire[first..]).unwrap();
    assert_eq!(&wire[..2], &644u16.to_be_bytes());

    let mut reassembler = StreamReassembler::new();
    let mut frames: std::vec::Vec<std::vec::Vec<u8>> = std::vec::Vec::new();
    for chunk in wire[..total].chunks(244) {
        let mut rest = chunk;
        while !rest.is_empty() {
            if let Some(done) = reassembler.push(&mut rest).unwrap() {
                frames.push(done.to_vec());
            }
        }
    }
    assert_eq!(frames, [frame.clone(), frame[..3].to_vec()]);

    let mut oversized: &[u8] = &[0x04, 0x01, 0xAA];
    assert_eq!(reassembler.push(&mut oversized), Err(ProtocolError::InvalidFrameLength));
    let mut zero: &[u8] = &[0x00, 0x00];
    assert_eq!(reassembler.push(&mut zero), Err(ProtocolError::InvalidFrameLength));
    assert_eq!(encode_stream_frame(&[], &mut wire), Err(ProtocolError::InvalidFrameLength));
    assert_eq!(encode_stream_frame(&[0; MAX_PACKET_BYTES + 1], &mut wire), Err(ProtocolError::InvalidFrameLength));

    let list_req = r#"{"jsonrpc":"2.0","id":"00000000-0000-0000-0000-000000000000","method":"tools/list"}"#;
    let mut list = [0u8; MAX_PACKET_BYTES];
    let mut avatar = AvatarRenderer::new();
    assert!(ProtocolDispatcher::new().dispatch_json_rpc(list_req, &mut avatar, &mut list).is_ok());
}

#[test]
fn test_inbound_routing_and_outbound_queue() {
    let mut dispatcher = ProtocolDispatcher::new();
    let mut avatar = AvatarRenderer::new();
    let mut prov = BleProvisioner::new();
    let mut audio = [0u8; 4 + BYTES_PER_FRAME];
    let payload = AudioFrame::new();
    let len = ProtocolDispatcher::encode_audio_frame(0, 9, payload.as_bytes(), &mut audio).unwrap();
    assert_eq!(len, 644);
    handle_inbound_packet(&mut dispatcher, &mut avatar, &mut prov, TransportKind::WifiDirect, &audio);
    assert!(AUDIO_RX_CHANNEL.try_receive().is_ok());

    let mut call = std::vec::Vec::from([OPCODE_CONTROL]);
    let telemetry = br#"{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"charm.get_telemetry"}}"#;
    call.extend_from_slice(telemetry);
    handle_inbound_packet(&mut dispatcher, &mut avatar, &mut prov, TransportKind::WifiDirect, &call);
    let (pkt, pkt_len) = OUTBOUND_PKT_CHANNEL.try_receive().unwrap();
    assert_eq!(pkt[0], OPCODE_CONTROL);
    assert!(core::str::from_utf8(&pkt[1..pkt_len]).unwrap().contains(r#""id":5"#));

    let mut identify = std::vec::Vec::from([OPCODE_PROVISION]);
    identify.extend_from_slice(br#"{"jsonrpc":"2.0","id":6,"method":"prov.identify"}"#);
    handle_inbound_packet(&mut dispatcher, &mut avatar, &mut prov, TransportKind::WifiDirect, &identify);
    let (pkt, pkt_len) = OUTBOUND_PKT_CHANNEL.try_receive().unwrap();
    assert!(core::str::from_utf8(&pkt[1..pkt_len]).unwrap().contains("-32001"));
    assert_eq!(prov.state, ProvisionState::Unclaimed);
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
    assert_eq!(pm.state, PowerState::StreamFullDuplex);
    assert_eq!(pm.silence_ms, 0);
}
