use crate::audio::{AudioFrame, InvalidFrameLength, SAMPLES_PER_FRAME};

/// Bytes the I2S receiver writes per 20 ms frame: 320 stereo pairs of 32-bit slots.
pub const MIC_FRAME_BYTES: usize = SAMPLES_PER_FRAME * 2 * 4;
/// Bytes the I2S transmitter reads per 20 ms frame: 320 stereo pairs of 16-bit samples.
pub const SPEAKER_FRAME_BYTES: usize = SAMPLES_PER_FRAME * 2 * 2;
/// Frames the VAD stays active after the last loud frame (300 ms), so pauses inside a sentence
/// do not end the utterance.
pub const VAD_HANGOVER_FRAMES: u8 = 15;
/// DC-blocker pole in Q15 (0.995): a high-pass corner near 13 Hz at 16 kHz.
const DC_POLE_Q15: i32 = 32_604;

/// Converts one SPH0645LM4H-B slot, as the ESP32-S3 I2S receiver captures it, to PCM16.
///
/// The microphone outputs 18-bit two's complement data, MSB first, zero-filled to 24 bits and
/// left-justified in the 32-bit slot. It changes DATA 66 ns after the rising BCLK edge (datasheet
/// t_DC), the edge on which a standard I2S receiver samples, so the receiver latches every bit one
/// clock late: the word arrives shifted right by one and its top bit is the pulled-down idle line.
/// Shifting left by one restores the MSB; the top 16 bits are then the PCM16 sample.
pub fn sph0645_slot_to_pcm16(slot: u32) -> i16 {
    ((slot << 1) >> 16) as u16 as i16
}

/// One-pole high-pass filter that removes the microphone's DC offset.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DcBlocker {
    last_in: i32,
    last_out: i32,
}

impl DcBlocker {
    pub fn process(&mut self, sample: i16) -> i16 {
        let x = i32::from(sample);
        let y = x - self.last_in + ((self.last_out * DC_POLE_Q15) >> 15);
        self.last_in = x;
        self.last_out = y.clamp(i32::from(i16::MIN), i32::from(i16::MAX));
        self.last_out as i16
    }
}

/// Decodes one DMA frame of little-endian 32-bit slots, keeping the left channel.
pub fn decode_mic_frame(raw: &[u8], dc: &mut DcBlocker, frame: &mut AudioFrame) -> Result<(), InvalidFrameLength> {
    let raw = raw.get(..MIC_FRAME_BYTES).ok_or(InvalidFrameLength)?;
    for (sample, pair) in frame.samples.iter_mut().zip(raw.as_chunks::<8>().0) {
        let left = u32::from_le_bytes([pair[0], pair[1], pair[2], pair[3]]);
        *sample = dc.process(sph0645_slot_to_pcm16(left));
    }
    Ok(())
}

/// Encodes a frame for the MAX98357A: the same 16-bit sample in the left and right slots.
pub fn encode_speaker_frame(frame: &AudioFrame, out: &mut [u8]) -> Result<(), InvalidFrameLength> {
    let out = out.get_mut(..SPEAKER_FRAME_BYTES).ok_or(InvalidFrameLength)?;
    for (pair, sample) in out.as_chunks_mut::<4>().0.iter_mut().zip(frame.samples) {
        let [lo, hi] = sample.to_le_bytes();
        *pair = [lo, hi, lo, hi];
    }
    Ok(())
}

/// RMS voice activity detector with a hangover.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Vad {
    hangover: u8,
}

impl Vad {
    /// Returns whether the frame counts as speech, holding true for `VAD_HANGOVER_FRAMES` after the last loud one.
    pub fn update(&mut self, frame: &AudioFrame, threshold: u16) -> bool {
        if frame.is_voice_active(threshold) {
            self.hangover = VAD_HANGOVER_FRAMES;
            return true;
        }
        self.hangover = self.hangover.saturating_sub(1);
        self.hangover > 0
    }
}
