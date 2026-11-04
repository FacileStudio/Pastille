//! The crypto provider for gateway TLS sessions.

use super::rng::RadioRng;
use crate::trust::{AnchorVerifier, Suite};
use embedded_tls::{CryptoProvider, CryptoRngCore, TlsError, TlsVerifier};
use esp_hal::rng::Rng;

/// Crypto for one gateway session: the radio RNG and the pinned-root verifier.
pub struct GatewayProvider {
    rng: RadioRng,
    verifier: AnchorVerifier,
}

impl GatewayProvider {
    pub fn new() -> Self {
        Self { rng: RadioRng(Rng::new()), verifier: AnchorVerifier::new() }
    }
}

impl CryptoProvider for GatewayProvider {
    type CipherSuite = Suite;
    type Signature = [u8; 0];

    fn rng(&mut self) -> impl CryptoRngCore {
        &mut self.rng
    }

    fn verifier(&mut self) -> Result<&mut impl TlsVerifier<Suite>, TlsError> {
        Ok(&mut self.verifier)
    }
}
