//! BLE peripheral on trouble-host over the esp-radio controller: advertising with the BOHS-01
//! service data, the GATT server, LE Secure Connections pairing and one persisted bond.

mod bond;
mod gatt;
mod session;

use crate::ble::gatt::BLE_SERVICE_UUID;
use crate::ble_link::{info_record, service_data};
use crate::link::{device_info, INFO_CHANGED};
use crate::store::BOND_BYTES;
use embassy_futures::join::join;
use embassy_futures::select::{select, Either};
use embassy_time::{Duration, Timer};
use esp_radio::ble::controller::BleConnector;
use gatt::Server;
use trouble_host::prelude::*;

const CONNECTIONS: usize = 1;
const CHANNELS: usize = 3;
/// Name in the scan response.
const DEVICE_NAME: &str = "Pastille";
/// AD type "Service Data - 128-bit UUID".
const AD_SERVICE_DATA_128: u8 = 0x21;

pub(super) type Ble = ExternalController<BleConnector<'static>, 20>;

/// Builds the advertising payload: flags, then the service UUID with the 6-byte service data.
fn advertising_data(hardware_id: [u8; 4], out: &mut [u8; 31]) -> Option<usize> {
    let info = device_info();
    let mut data = [0u8; 22];
    data[..16].copy_from_slice(&BLE_SERVICE_UUID.to_le_bytes());
    data[16..].copy_from_slice(&service_data(info.state, hardware_id));
    let service = AdStructure::Unknown { ty: AD_SERVICE_DATA_128, data: &data };
    AdStructure::encode_slice(&[AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED), service], out).ok()
}

async fn advertise_once(
    peripheral: &mut Peripheral<'_, Ble, DefaultPacketPool>,
    server: &Server<'_>,
    stack: &Stack<'_, Ble, DefaultPacketPool>,
    hardware_id: [u8; 4],
) {
    let _ = server.set(&server.bohs.info, &info_record(&device_info()));
    let (mut adv, mut scan) = ([0u8; 31], [0u8; 31]);
    let Some(adv_len) = advertising_data(hardware_id, &mut adv) else { return };
    let Ok(scan_len) = AdStructure::encode_slice(&[AdStructure::CompleteLocalName(DEVICE_NAME.as_bytes())], &mut scan)
    else {
        return;
    };
    let kind =
        Advertisement::ConnectableScannableUndirected { adv_data: &adv[..adv_len], scan_data: &scan[..scan_len] };
    let Ok(advertiser) = peripheral.advertise(&AdvertisementParameters::default(), kind).await else {
        Timer::after(Duration::from_secs(1)).await;
        return;
    };
    INFO_CHANGED.reset();
    if let Either::First(Ok(conn)) = select(advertiser.accept(), INFO_CHANGED.wait()).await {
        if let Ok(gatt) = conn.with_attribute_server(server) {
            session::serve(server, &gatt, stack).await;
        }
    }
}

/// Runs the BLE host: restores the stored bond, then advertises and serves one central at a time.
#[embassy_executor::task]
pub async fn ble_task(
    connector: BleConnector<'static>,
    hardware_id: [u8; 4],
    address: [u8; 6],
    bond: Option<[u8; BOND_BYTES]>,
) {
    let mut resources: HostResources<Ble, DefaultPacketPool, CONNECTIONS, CHANNELS> = HostResources::new();
    let stack = trouble_host::new(ExternalController::new(connector), &mut resources)
        .set_random_address(Address::random(address))
        .set_io_capabilities(IoCapabilities::DisplayYesNo)
        .build();
    if let Some(bond) = bond.as_ref().and_then(bond::decode) {
        let _ = stack.add_bond_information(bond);
    }
    let config = PeripheralConfig { name: DEVICE_NAME, appearance: &appearance::watch::GENERIC_WATCH };
    let Ok(server) = Server::new_with_config(GapConfig::Peripheral(config)) else { return };
    let mut peripheral = stack.peripheral();
    let mut runner = stack.runner();
    let _ = join(runner.run(), async {
        loop {
            advertise_once(&mut peripheral, &server, &stack, hardware_id).await;
        }
    })
    .await;
}
