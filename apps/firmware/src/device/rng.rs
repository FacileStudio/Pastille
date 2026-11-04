//! The cryptographic RNG behind TLS.

use esp_hal::rng::Rng;

/// The hardware RNG. It produces true random numbers while the radio is on, which it always is
/// when a TLS session runs over Wi-Fi.
pub struct RadioRng(pub Rng);

impl rand_core::RngCore for RadioRng {
    fn next_u32(&mut self) -> u32 {
        self.0.random()
    }

    fn next_u64(&mut self) -> u64 {
        u64::from(self.0.random()) << 32 | u64::from(self.0.random())
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.read(dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.0.read(dest);
        Ok(())
    }
}

impl rand_core::CryptoRng for RadioRng {}
