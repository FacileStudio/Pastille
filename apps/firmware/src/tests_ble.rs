use crate::ble::{BleProvisioner, ProvisionState, TransportKind, AUTH_WINDOW_MS};
use crate::protocol::OPCODE_PROVISION;
use crate::MAX_PACKET_BYTES;

const TOKEN: &str = "bohs_tok_0123456789abcdef0123456789abcdef";
const HW_ID: [u8; 4] = [0x4F, 0x2A, 0x91, 0xC0];
const IDENTIFY: &str = r#"{"jsonrpc":"2.0","id":1,"method":"prov.identify"}"#;
const UNAUTHORIZED: &str = r#""error":{"code":-32001,"message":"unauthorized"}"#;

fn configure_req(token: &str) -> String {
    format!(
        concat!(
            r#"{{"jsonrpc":"2.0","id":7,"method":"prov.configure","params":{{"#,
            r#""gateway_url":"wss://bulle.example/api/hardware/ws","device_token":"{}","prefer_ble":true}}}}"#
        ),
        token
    )
}

fn prov_call(prov: &mut BleProvisioner, req: &str) -> String {
    let mut out = [0u8; MAX_PACKET_BYTES];
    let len = prov.dispatch_provision_rpc(req, TransportKind::BleRelay, &mut out).unwrap();
    assert_eq!(out[0], OPCODE_PROVISION);
    String::from_utf8(out[1..len].to_vec()).unwrap()
}

fn secure_link(prov: &mut BleProvisioner, handle: u16) {
    prov.on_connected(handle);
    prov.on_link_encrypted(true);
}

#[test]
fn test_provisioner_rejects_unencrypted_links() {
    let mut prov = BleProvisioner::new(HW_ID);
    assert_eq!(prov.advertisement_service_data(), [0x00, 0x0F, 0x4F, 0x2A, 0x91, 0xC0]);
    assert!(prov_call(&mut prov, IDENTIFY).contains(UNAUTHORIZED));
    prov.on_connected(1);
    assert!(prov_call(&mut prov, IDENTIFY).contains(UNAUTHORIZED));
    assert!(prov_call(&mut prov, r#"{"jsonrpc":"2.0","id":2,"method":"prov.status"}"#).contains(UNAUTHORIZED));
    assert_eq!(prov.state, ProvisionState::Unclaimed);

    prov.on_link_encrypted(true);
    prov_call(&mut prov, IDENTIFY);
    assert!(prov_call(&mut prov, &configure_req(TOKEN)).contains(UNAUTHORIZED));
    prov.on_physical_tap();
    prov.on_link_encrypted(false);
    assert_eq!(prov.state, ProvisionState::Unclaimed);
    assert!(prov_call(&mut prov, &configure_req(TOKEN)).contains(UNAUTHORIZED));
    let mut out = [0u8; MAX_PACKET_BYTES];
    let notification = r#"{"jsonrpc":"2.0","method":"prov.identify"}"#;
    assert_eq!(prov.dispatch_provision_rpc(notification, TransportKind::BleRelay, &mut out), None);
}

#[test]
fn test_provisioning_happy_path() {
    let mut prov = BleProvisioner::new(HW_ID);
    secure_link(&mut prov, 1);
    let identify = prov_call(&mut prov, IDENTIFY);
    let expected = r#""state":"awaiting_tap","transport":"ble","link_encrypted":true,"hardware_id":"4f2a91c0"}}"#;
    assert!(identify.ends_with(expected));
    prov.on_physical_tap();

    let invalid = [
        configure_req(&TOKEN[..TOKEN.len() - 1]),
        configure_req(&TOKEN.to_uppercase()),
        configure_req(TOKEN).replace(r#","prefer_ble":true"#, ""),
        configure_req(TOKEN).replace("wss://bulle.example/api/hardware/ws", "wss://"),
        configure_req(TOKEN).replace("wss://bulle.example/api/hardware/ws", "wss:///api"),
        configure_req(TOKEN).replace("wss://bulle.example", "https://bulle.example"),
        configure_req(TOKEN).replace(r#""prefer_ble":true"#, r#""prefer_ble":true,"wifi_ssid":123"#),
    ];
    for req in invalid {
        assert!(prov_call(&mut prov, &req).contains(r#""code":-32602"#), "{req}");
    }

    let done = prov_call(&mut prov, &configure_req(TOKEN));
    assert_eq!(done, r#"{"jsonrpc":"2.0","id":7,"result":{"status":"provisioned"}}"#);
    let status = prov_call(&mut prov, r#"{"jsonrpc":"2.0","id":8,"method":"prov.status"}"#);
    assert!(status.contains(r#""state":"provisioned","transport":"ble","link_encrypted":true"#));
    assert_eq!((prov.state, prov.transport), (ProvisionState::Provisioned, TransportKind::BleRelay));
    assert_eq!(prov.config.as_ref().unwrap().device_token.as_str(), TOKEN);
    assert!(!done.contains(TOKEN));
}

#[test]
fn test_auth_window_cannot_be_extended() {
    let mut prov = BleProvisioner::new([0xFF; 4]);
    prov.transport = TransportKind::WifiDirect;
    secure_link(&mut prov, 3);
    let id = "x".repeat(64);
    let req = format!(r#"{{"jsonrpc":"2.0","id":"{}","method":"prov.identify"}}"#, id);
    let expected = r#""transport":"wifi","link_encrypted":true,"hardware_id":"ffffffff"}}"#;
    assert!(prov_call(&mut prov, &req).ends_with(expected));
    prov.tick(AUTH_WINDOW_MS - 10);
    prov_call(&mut prov, IDENTIFY);
    prov.on_physical_tap();
    prov_call(&mut prov, IDENTIFY);
    assert_eq!((prov.state, prov.auth_timer_ms), (ProvisionState::Authorized, 10));
    prov.tick(10);
    assert_eq!(prov.state, ProvisionState::Unclaimed);
    prov.on_physical_tap();
    assert_eq!(prov.state, ProvisionState::Unclaimed);
    assert!(prov_call(&mut prov, &configure_req(TOKEN)).contains(UNAUTHORIZED));
}

#[test]
fn test_repair_keeps_old_config_until_configured() {
    let mut prov = BleProvisioner::new(HW_ID);
    secure_link(&mut prov, 1);
    prov_call(&mut prov, IDENTIFY);
    prov.on_physical_tap();
    prov_call(&mut prov, &configure_req(TOKEN));
    let old = prov.config.clone();

    prov_call(&mut prov, IDENTIFY);
    assert_eq!(prov.state, ProvisionState::AwaitingTap);
    prov.on_physical_tap();
    prov.tick(AUTH_WINDOW_MS);
    assert_eq!((prov.state, &prov.config), (ProvisionState::Provisioned, &old));
    assert!(prov_call(&mut prov, &configure_req(TOKEN)).contains(UNAUTHORIZED));
}

#[test]
fn test_tap_authorization_is_bound_to_its_connection() {
    let mut prov = BleProvisioner::new(HW_ID);
    secure_link(&mut prov, 1);
    prov_call(&mut prov, IDENTIFY);
    prov.on_physical_tap();
    prov.on_disconnect();
    assert_eq!((prov.state, prov.conn), (ProvisionState::Unclaimed, None));

    secure_link(&mut prov, 2);
    assert!(prov_call(&mut prov, &configure_req(TOKEN)).contains(UNAUTHORIZED));
    prov.state = ProvisionState::Authorized;
    prov.auth_conn = Some(1);
    assert!(prov_call(&mut prov, &configure_req(TOKEN)).contains(UNAUTHORIZED));
    assert!(prov.config.is_none());
}
