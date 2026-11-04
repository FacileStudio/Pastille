pub const SAMPLE_RATE_HZ: u32 = 16_000;
pub const FRAME_DURATION_MS: u32 = 20;
pub const SAMPLES_PER_FRAME: usize = (SAMPLE_RATE_HZ * FRAME_DURATION_MS / 1000) as usize;
pub const BYTES_PER_FRAME: usize = SAMPLES_PER_FRAME * 2;
pub const DEFAULT_VAD_THRESHOLD: u16 = 450;

/// A payload that is not exactly `BYTES_PER_FRAME` long.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidFrameLength;

/// One 20 ms frame of 16 kHz mono PCM16.
#[derive(Clone, Copy)]
pub struct AudioFrame {
    pub samples: [i16; SAMPLES_PER_FRAME],
}

impl AudioFrame {
    pub const fn new() -> Self {
        Self { samples: [0; SAMPLES_PER_FRAME] }
    }

    /// Encodes the samples as little-endian PCM16 into the first `BYTES_PER_FRAME` bytes of `out`.
    pub fn write_le_bytes(&self, out: &mut [u8]) -> Result<(), InvalidFrameLength> {
        let out = out.get_mut(..BYTES_PER_FRAME).ok_or(InvalidFrameLength)?;
        for (chunk, sample) in out.as_chunks_mut::<2>().0.iter_mut().zip(self.samples) {
            *chunk = sample.to_le_bytes();
        }
        Ok(())
    }

    /// Decodes a little-endian PCM16 payload of exactly `BYTES_PER_FRAME` bytes.
    pub fn copy_from_bytes(&mut self, bytes: &[u8]) -> Result<(), InvalidFrameLength> {
        if bytes.len() != BYTES_PER_FRAME {
            return Err(InvalidFrameLength);
        }
        for (sample, chunk) in self.samples.iter_mut().zip(bytes.as_chunks::<2>().0) {
            *sample = i16::from_le_bytes(*chunk);
        }
        Ok(())
    }

    pub fn calculate_rms(&self) -> u16 {
        let sum_squares: u64 = self.samples.iter().map(|&s| (s as i64 * s as i64) as u64).sum();
        (sum_squares / SAMPLES_PER_FRAME as u64).isqrt() as u16
    }

    pub fn is_voice_active(&self, threshold: u16) -> bool {
        self.calculate_rms() > threshold
    }
}
