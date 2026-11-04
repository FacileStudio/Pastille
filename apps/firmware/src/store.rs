use crate::ble::ProvisionConfig;
use heapless::String;

/// Magic number that starts every stored record ("PST1").
pub const RECORD_MAGIC: u32 = 0x5053_5431;
/// Opaque size of the serialized BLE bond the device keeps next to its config.
pub const BOND_BYTES: usize = 48;
/// Largest record: header, the four strings with their length bytes, flags, bond and CRC.
pub const RECORD_BYTES: usize = 12 + (1 + 96) + (1 + 48) + (1 + 32) + (1 + 64) + 2 + BOND_BYTES + 4;
const HEADER_BYTES: usize = 10;

/// What the device keeps in flash across reboots.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Stored {
    pub config: Option<ProvisionConfig>,
    pub bond: Option<[u8; BOND_BYTES]>,
}

/// CRC-32 (IEEE 802.3, reflected, as used by zlib).
pub fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = !0u32;
    for &byte in bytes {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xEDB8_8320 & (crc & 1).wrapping_neg());
        }
    }
    !crc
}

/// Serializes `stored` as record number `seq`: magic, seq, payload length, payload, CRC of all of it.
pub fn encode_record(seq: u32, stored: &Stored, out: &mut [u8; RECORD_BYTES]) -> usize {
    let mut len = HEADER_BYTES;
    let mut push = |bytes: &[u8]| {
        out[len..len + bytes.len()].copy_from_slice(bytes);
        len += bytes.len();
    };
    match &stored.config {
        Some(cfg) => {
            push(&[1 | (u8::from(cfg.prefer_ble) << 1)]);
            for field in [cfg.gateway_url.as_str(), &cfg.device_token, &cfg.wifi_ssid, &cfg.wifi_psk] {
                push(&[field.len() as u8]);
                push(field.as_bytes());
            }
        }
        None => push(&[0]),
    }
    match &stored.bond {
        Some(bond) => {
            push(&[1]);
            push(bond);
        }
        None => push(&[0]),
    }
    out[..4].copy_from_slice(&RECORD_MAGIC.to_le_bytes());
    out[4..8].copy_from_slice(&seq.to_le_bytes());
    out[8..10].copy_from_slice(&((len - HEADER_BYTES) as u16).to_le_bytes());
    let crc = crc32(&out[..len]);
    out[len..len + 4].copy_from_slice(&crc.to_le_bytes());
    len + 4
}

/// Parses a record; `None` for erased flash, a torn write or a corrupted payload.
pub fn decode_record(bytes: &[u8]) -> Option<(u32, Stored)> {
    let header = bytes.get(..HEADER_BYTES)?;
    if u32::from_le_bytes(header[..4].try_into().ok()?) != RECORD_MAGIC {
        return None;
    }
    let payload_len = usize::from(u16::from_le_bytes([header[8], header[9]]));
    let end = HEADER_BYTES + payload_len;
    let crc = u32::from_le_bytes(bytes.get(end..end + 4)?.try_into().ok()?);
    if crc32(&bytes[..end]) != crc {
        return None;
    }
    let seq = u32::from_le_bytes(header[4..8].try_into().ok()?);
    let mut reader = Reader { bytes: &bytes[HEADER_BYTES..end] };
    Some((seq, reader.stored()?))
}

/// Picks the newest valid record of the two flash slots.
pub fn newest(a: Option<(u32, Stored)>, b: Option<(u32, Stored)>) -> Option<(u32, Stored)> {
    match (a, b) {
        (Some(a), Some(b)) => Some(if b.0.wrapping_sub(a.0) as i32 > 0 { b } else { a }),
        (a, b) => a.or(b),
    }
}

struct Reader<'a> {
    bytes: &'a [u8],
}

impl Reader<'_> {
    fn take(&mut self, n: usize) -> Option<&[u8]> {
        let (head, rest) = self.bytes.split_at_checked(n)?;
        self.bytes = rest;
        Some(head)
    }

    fn string<const N: usize>(&mut self) -> Option<String<N>> {
        let len = usize::from(*self.take(1)?.first()?);
        String::try_from(core::str::from_utf8(self.take(len)?).ok()?).ok()
    }

    fn stored(&mut self) -> Option<Stored> {
        let mut stored = Stored::default();
        let flags = *self.take(1)?.first()?;
        if flags & 1 == 1 {
            let (gateway_url, device_token) = (self.string()?, self.string()?);
            let (wifi_ssid, wifi_psk) = (self.string()?, self.string()?);
            let prefer_ble = flags & 2 == 2;
            stored.config = Some(ProvisionConfig { gateway_url, device_token, wifi_ssid, wifi_psk, prefer_ble });
        }
        if *self.take(1)?.first()? == 1 {
            stored.bond = Some(self.take(BOND_BYTES)?.try_into().ok()?);
        }
        Some(stored)
    }
}
