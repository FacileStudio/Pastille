//! The BOHS-01 GATT service: `B001` info, then the `B002` audio, `B003` control and `B004`
//! provisioning streams. `B004` writes and subscriptions need an encrypted link.

use trouble_host::prelude::*;

/// Largest chunk one ATT notification, indication or write carries at the 247-byte MTU.
pub const CHUNK_BYTES: usize = 244;

#[gatt_server]
pub struct Server {
    pub bohs: BohsService,
}

#[gatt_service(uuid = "b0010000-f4c1-4e00-b001-000000000001")]
pub struct BohsService {
    #[characteristic(uuid = "b0010001-f4c1-4e00-b001-000000000001", read)]
    pub info: [u8; 8],
    #[characteristic(
        uuid = "b0010002-f4c1-4e00-b001-000000000001",
        notify,
        write_without_response,
        value = [0; CHUNK_BYTES]
    )]
    pub audio: [u8; CHUNK_BYTES],
    #[characteristic(uuid = "b0010003-f4c1-4e00-b001-000000000001", indicate, write, value = [0; CHUNK_BYTES])]
    pub control: [u8; CHUNK_BYTES],
    #[characteristic(
        uuid = "b0010004-f4c1-4e00-b001-000000000001",
        indicate,
        write,
        value = [0; CHUNK_BYTES],
        permissions(write = encrypted, cccd = encrypted)
    )]
    pub prov: [u8; CHUNK_BYTES],
}
