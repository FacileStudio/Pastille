//! Converts the one bond the device keeps between trouble-host and its flash record.

use crate::store::BOND_BYTES;
use core::num::NonZeroU128;
use trouble_host::prelude::*;

/// Serializes a bond: valid flag, LTK, address kind and bytes, IRK flag and IRK, security level.
pub fn encode(bond: &BondInformation) -> [u8; BOND_BYTES] {
    let mut out = [0u8; BOND_BYTES];
    out[0] = 1;
    out[1..17].copy_from_slice(&bond.ltk.0.to_le_bytes());
    out[17] = bond.identity.addr.kind.as_raw();
    out[18..24].copy_from_slice(bond.identity.addr.addr.raw());
    if let Some(irk) = bond.identity.irk {
        out[24] = 1;
        out[25..41].copy_from_slice(&irk.0.get().to_le_bytes());
    }
    out[41] = u8::from(bond.security_level == SecurityLevel::EncryptedAuthenticated);
    out
}

/// Restores a bond written by `encode`; `None` for an empty or invalid blob.
pub fn decode(blob: &[u8; BOND_BYTES]) -> Option<BondInformation> {
    if blob[0] != 1 {
        return None;
    }
    let ltk = LongTermKey(u128::from_le_bytes(blob[1..17].try_into().ok()?));
    let addr = BdAddr::new(blob[18..24].try_into().ok()?);
    let irk = (blob[24] == 1)
        .then(|| NonZeroU128::new(u128::from_le_bytes(blob[25..41].try_into().ok()?)))
        .flatten()
        .map(IdentityResolvingKey);
    let identity = Identity { addr: Address { kind: AddrKind::new(blob[17]), addr }, irk };
    let level = if blob[41] == 1 { SecurityLevel::EncryptedAuthenticated } else { SecurityLevel::Encrypted };
    Some(BondInformation::new(identity, ltk, level, true))
}
