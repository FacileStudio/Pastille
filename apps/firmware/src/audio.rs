#![allow(dead_code, clippy::chunks_exact_to_as_chunks)]

pub const SAMPLE_RATE_HZ: u32 = 16_000;
pub const FRAME_DURATION_MS: u32 = 20;
pub const SAMPLES_PER_FRAME: usize = 320;
pub const BYTES_PER_FRAME: usize = SAMPLES_PER_FRAME * 2;
pub const DEFAULT_VAD_THRESHOLD: u16 = 450;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioError {
    BufferOverflow,
    BufferUnderflow,
    DmaError,
    HardwareFault,
}

#[derive(Clone, Copy)]
pub struct AudioFrame {
    pub samples: [i16; SAMPLES_PER_FRAME],
}

impl Default for AudioFrame {
    fn default() -> Self {
        Self {
            samples: [0; SAMPLES_PER_FRAME],
        }
    }
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

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        let ptr = self.samples.as_mut_ptr() as *mut u8;
        unsafe { core::slice::from_raw_parts_mut(ptr, BYTES_PER_FRAME) }
    }

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
        isqrt(mean) as u16
    }

    pub fn is_voice_active(&self, threshold: u16) -> bool {
        self.calculate_rms() > threshold
    }
}

pub struct I2sPipeline {
    amp_enabled: bool,
    mic_gain: u8,
    tx_frames: u32,
    rx_frames: u32,
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

    pub fn set_amp_enabled(&mut self, enabled: bool) {
        self.amp_enabled = enabled;
    }

    pub fn is_amp_enabled(&self) -> bool {
        self.amp_enabled
    }

    pub fn set_mic_gain(&mut self, gain: u8) {
        self.mic_gain = gain;
    }

    pub fn mic_gain(&self) -> u8 {
        self.mic_gain
    }

    pub fn capture_frame(&mut self, frame: &mut AudioFrame) -> Result<(), AudioError> {
        self.rx_frames = self.rx_frames.wrapping_add(1);
        for (i, sample) in frame.samples.iter_mut().enumerate() {
            let synth = ((i % 32) as i16 - 16) * 20;
            *sample = synth;
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

    pub fn tx_frame_count(&self) -> u32 {
        self.tx_frames
    }

    pub fn rx_frame_count(&self) -> u32 {
        self.rx_frames
    }
}

fn isqrt(val: u64) -> u64 {
    if val == 0 {
        return 0;
    }
    let mut x0 = val / 2;
    if x0 == 0 {
        return 1;
    }
    let mut x1 = (x0 + val / x0) / 2;
    while x1 < x0 {
        x0 = x1;
        x1 = (x0 + val / x0) / 2;
    }
    x0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_frame_bytes() {
        let mut frame = AudioFrame::new();
        frame.samples[0] = 0x1234;
        frame.samples[1] = -500;

        let bytes = frame.as_bytes();
        assert_eq!(bytes.len(), BYTES_PER_FRAME);

        let mut roundtrip = AudioFrame::new();
        roundtrip.copy_from_bytes(bytes).unwrap();
        assert_eq!(roundtrip.samples[0], 0x1234);
        assert_eq!(roundtrip.samples[1], -500);
    }

    #[test]
    fn test_rms_and_vad() {
        let mut quiet_frame = AudioFrame::new();
        for s in quiet_frame.samples.iter_mut() {
            *s = 10;
        }
        assert!(quiet_frame.calculate_rms() < DEFAULT_VAD_THRESHOLD);
        assert!(!quiet_frame.is_voice_active(DEFAULT_VAD_THRESHOLD));

        let mut loud_frame = AudioFrame::new();
        for s in loud_frame.samples.iter_mut() {
            *s = 2000;
        }
        assert!(loud_frame.calculate_rms() > DEFAULT_VAD_THRESHOLD);
        assert!(loud_frame.is_voice_active(DEFAULT_VAD_THRESHOLD));
    }

    #[test]
    fn test_pipeline_control() {
        let mut pipe = I2sPipeline::new();
        assert!(!pipe.is_amp_enabled());

        let frame = AudioFrame::new();
        assert_eq!(pipe.play_frame(&frame), Err(AudioError::HardwareFault));

        pipe.set_amp_enabled(true);
        assert!(pipe.is_amp_enabled());
        assert_eq!(pipe.play_frame(&frame), Ok(()));
        assert_eq!(pipe.tx_frame_count(), 1);

        let mut rx_frame = AudioFrame::new();
        pipe.capture_frame(&mut rx_frame).unwrap();
        assert_eq!(pipe.rx_frame_count(), 1);
    }
}
