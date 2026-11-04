#![cfg_attr(target_arch = "xtensa", no_std)]
#![cfg_attr(target_arch = "xtensa", no_main)]
#![cfg_attr(not(any(test, target_arch = "xtensa")), allow(dead_code))]

mod audio;
mod avatar;
mod battery;
/// BLE provisioning state machine; the GATT server that drives it is not in v0.4.0.
#[cfg_attr(not(test), allow(dead_code))]
mod ble;
#[cfg_attr(not(test), allow(dead_code))]
mod ble_link;
mod events;
mod haptic;
mod link;
mod pins;
/// Radio and deep-sleep power states; v0.4.0 has no radio link to drive them.
#[cfg_attr(not(test), allow(dead_code))]
mod power;
mod protocol;
mod prov;
mod rpc;
mod stream;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_ble;
#[cfg(test)]
mod tests_link;
#[cfg(test)]
mod tests_rpc;
#[cfg(test)]
mod tests_wire;
mod tools;
mod trig;

/// Largest bohs.v1 frame on any transport: a 644-byte audio frame or a JSON-RPC message.
pub const MAX_PACKET_BYTES: usize = 1024;
/// A packet buffer paired with the number of bytes it actually holds.
pub type Packet = ([u8; MAX_PACKET_BYTES], usize);

/// The host build only hosts the unit tests; dead-code checks run on the device build.
#[cfg(not(target_arch = "xtensa"))]
fn main() {
    println!("Pastille BOHS-01 firmware host build: {:?}", pins::BOARD_PINS);
}
