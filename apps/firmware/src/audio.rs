pub const SAMPLE_RATE_HZ: u32 = 16_000;
pub const FRAME_DURATION_MS: u32 = 20;
pub const SAMPLES_PER_FRAME: usize = (SAMPLE_RATE_HZ * FRAME_DURATION_MS / 1000) as usize;
pub const BYTES_PER_FRAME: usize = SAMPLES_PER_FRAME * 2;
pub const DEFAULT_VAD_THRESHOLD: u16 = 450;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioError {
    BufferUnderflow,
    HardwareFault,
}

#[derive(Clone, Copy)]
pub struct AudioFrame {
    pub samples: [i16; SAMPLES_PER_FRAME],
}

impl AudioFrame {
    pub const fn new() -> Self {
        Self {
            samples: [0; SAMPLES_PER_FRAME],
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        let ptr = self.samples.as_ptr() as *const u8;
        unsafe { core::slice::from_raw_parts(ptr, BYTES_PER_FRAME) }
    }

    #[allow(clippy::chunks_exact_to_as_chunks)]
    pub fn copy_from_bytes(&mut self, bytes: &[u8]) -> Result<(), AudioError> {
        if bytes.len() < BYTES_PER_FRAME {
            return Err(AudioError::BufferUnderflow);
        }
        for (i, chunk) in bytes[..BYTES_PER_FRAME].chunks_exact(2).enumerate() {
            self.samples[i] = i16::from_le_bytes([chunk[0], chunk[1]]);
        }
        Ok(())
    }

    pub fn calculate_rms(&self) -> u16 {
        let mut sum_squares: u64 = 0;
        for &sample in &self.samples {
            let val = sample as i64;
            sum_squares += (val * val) as u64;
        }
        let mean = sum_squares / (SAMPLES_PER_FRAME as u64);
        mean.isqrt() as u16
    }

    pub fn is_voice_active(&self, threshold: u16) -> bool {
        self.calculate_rms() > threshold
    }
}

pub struct I2sPipeline {
    pub amp_enabled: bool,
    pub mic_gain: u8,
    pub tx_frames: u32,
    pub rx_frames: u32,
}

impl I2sPipeline {
    pub const fn new() -> Self {
        Self {
            amp_enabled: false,
            mic_gain: 100,
            tx_frames: 0,
            rx_frames: 0,
        }
    }

    pub fn capture_frame(&mut self, frame: &mut AudioFrame) -> Result<(), AudioError> {
        self.rx_frames = self.rx_frames.wrapping_add(1);
        let gain = self.mic_gain as i32;
        for (i, sample) in frame.samples.iter_mut().enumerate() {
            let raw = ((i % 32) as i32 - 16) * 20;
            let scaled = (raw * gain) / 100;
            *sample = scaled.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
        }
        Ok(())
    }

    pub fn play_frame(&mut self, _frame: &AudioFrame) -> Result<(), AudioError> {
        if !self.amp_enabled {
            return Err(AudioError::HardwareFault);
        }
        self.tx_frames = self.tx_frames.wrapping_add(1);
        Ok(())
    }
}
