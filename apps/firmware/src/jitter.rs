use crate::audio::AudioFrame;

/// Frames the buffer holds: 80 ms.
pub const JITTER_SLOTS: usize = 4;
/// Frames buffered before playback starts: 40 ms.
pub const JITTER_PREFILL: usize = 2;
/// Consecutive empty slots after which the stream counts as ended.
pub const JITTER_END_MISSES: u8 = 5;

/// Reorders downstream frames by sequence number and plays them at the I2S clock. A frame that
/// is not there when its turn comes plays as silence and is dropped if it arrives later, so a
/// late frame never stalls playback.
#[derive(Clone)]
pub struct JitterBuffer {
    slots: [Option<(u16, AudioFrame)>; JITTER_SLOTS],
    next: Option<u16>,
    playing: bool,
    misses: u8,
}

impl JitterBuffer {
    pub const fn new() -> Self {
        Self { slots: [None; JITTER_SLOTS], next: None, playing: false, misses: 0 }
    }

    fn buffered(&self) -> usize {
        self.slots.iter().flatten().count()
    }

    /// Stores a received frame; frames older than the play position are dropped, and a frame far
    /// ahead of it restarts the stream there.
    pub fn push(&mut self, seq: u16, frame: AudioFrame) {
        let next = *self.next.get_or_insert(seq);
        let ahead = seq.wrapping_sub(next) as i16;
        if ahead < 0 {
            return;
        }
        if ahead as usize >= JITTER_SLOTS {
            *self = Self::new();
            self.next = Some(seq);
        }
        self.slots[usize::from(seq) % JITTER_SLOTS] = Some((seq, frame));
    }

    /// Returns the frame due now, or `None` for silence.
    pub fn pop(&mut self) -> Option<AudioFrame> {
        if !self.playing {
            self.playing = self.buffered() >= JITTER_PREFILL;
            if !self.playing {
                return None;
            }
        }
        let next = self.next?;
        self.next = Some(next.wrapping_add(1));
        let slot = &mut self.slots[usize::from(next) % JITTER_SLOTS];
        match slot.take().filter(|(seq, _)| *seq == next) {
            Some((_, frame)) => {
                self.misses = 0;
                Some(frame)
            }
            None => {
                self.misses += 1;
                if self.misses >= JITTER_END_MISSES && self.buffered() == 0 {
                    *self = Self::new();
                }
                None
            }
        }
    }

    /// Whether nothing is playing or waiting to play.
    pub fn is_idle(&self) -> bool {
        !self.playing && self.buffered() == 0
    }
}
