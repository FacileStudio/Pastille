#![cfg_attr(target_arch = "xtensa", no_std)]
#![cfg_attr(target_arch = "xtensa", no_main)]
#![cfg_attr(not(target_arch = "xtensa"), allow(dead_code))]

mod audio;
mod avatar;
mod battery;
mod ble;
mod ble_link;
#[cfg(target_arch = "xtensa")]
mod device;
mod events;
mod haptic;
mod link;
mod node;
mod node_effects;
mod overlay;
mod pins;
mod power;
mod protocol;
mod prov;
mod rpc;
mod store;
mod stream;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_ble;
#[cfg(test)]
mod tests_link;
#[cfg(test)]
mod tests_node;
#[cfg(test)]
mod tests_rpc;
#[cfg(test)]
mod tests_store;
#[cfg(test)]
mod tests_touch;
#[cfg(test)]
mod tests_wire;
mod tools;
mod touch;
mod trig;

#[cfg(target_arch = "xtensa")]
extern crate alloc;

/// Largest bohs.v1 frame on any transport: a 644-byte audio frame or a JSON-RPC message.
pub const MAX_PACKET_BYTES: usize = 1024;
/// A packet buffer paired with the number of bytes it actually holds.
pub type Packet = ([u8; MAX_PACKET_BYTES], usize);

/// The host build only hosts the unit tests; the device build is where dead code is reported.
#[cfg(not(target_arch = "xtensa"))]
fn main() {
    println!("Pastille BOHS-01 firmware host build: {:?}", pins::BOARD_PINS);
}
