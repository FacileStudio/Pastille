use crate::audio::{AudioFrame, InvalidFrameLength, DEFAULT_VAD_THRESHOLD};
use crate::jitter::{JitterBuffer, JITTER_END_MISSES, JITTER_SLOTS};
use crate::mic::{
    decode_mic_frame, encode_speaker_frame, sph0645_slot_to_pcm16, DcBlocker, Vad, MIC_FRAME_BYTES,
    SPEAKER_FRAME_BYTES, VAD_HANGOVER_FRAMES,
};

/// Builds the slot the receiver latches for an 18-bit sample: left-justified in 32 bits, then one bit late.
fn latched_slot(sample18: i32) -> u32 {
    ((sample18 as u32) << 14) >> 1
}

fn tone(level: i16) -> AudioFrame {
    let mut frame = AudioFrame::new();
    frame.samples.fill(level);
    frame
}

#[test]
fn test_sph0645_slot_conversion() {
    assert_eq!(sph0645_slot_to_pcm16(latched_slot(0)), 0);
    assert_eq!(sph0645_slot_to_pcm16(latched_slot(0x1_FFFF)), i16::MAX);
    assert_eq!(sph0645_slot_to_pcm16(latched_slot(-0x2_0000 & 0x3_FFFF)), i16::MIN);
    assert_eq!(sph0645_slot_to_pcm16(latched_slot(-4 & 0x3_FFFF)), -1);
    assert_eq!(sph0645_slot_to_pcm16(latched_slot(400)), 100);
    assert_eq!(sph0645_slot_to_pcm16(0x8000_0000), 0);
}

#[test]
fn test_mic_decode_removes_dc_and_speaker_encode_duplicates() {
    let mut raw = [0u8; MIC_FRAME_BYTES];
    for pair in raw.as_chunks_mut::<8>().0 {
        pair[..4].copy_from_slice(&latched_slot(4_000).to_le_bytes());
        pair[4..].copy_from_slice(&0xFFFF_FFFFu32.to_le_bytes());
    }
    let (mut dc, mut frame) = (DcBlocker::default(), AudioFrame::new());
    decode_mic_frame(&raw, &mut dc, &mut frame).unwrap();
    assert_eq!(frame.samples[0], 1_000);
    assert!(frame.samples[319].abs() < 200);
    for _ in 0..20 {
        decode_mic_frame(&raw, &mut dc, &mut frame).unwrap();
    }
    assert!(frame.samples.iter().all(|s| s.abs() <= 1));
    assert_eq!(decode_mic_frame(&raw[..100], &mut dc, &mut frame), Err(InvalidFrameLength));

    let mut out = [0u8; SPEAKER_FRAME_BYTES];
    let mut played = AudioFrame::new();
    played.samples[0] = -2;
    played.samples[1] = 0x1234;
    encode_speaker_frame(&played, &mut out).unwrap();
    assert_eq!(&out[..8], &[0xFE, 0xFF, 0xFE, 0xFF, 0x34, 0x12, 0x34, 0x12]);
    assert_eq!(encode_speaker_frame(&played, &mut out[..10]), Err(InvalidFrameLength));
}

#[test]
fn test_vad_hangover() {
    let mut vad = Vad::default();
    assert!(!vad.update(&tone(0), DEFAULT_VAD_THRESHOLD));
    assert!(vad.update(&tone(2000), DEFAULT_VAD_THRESHOLD));
    for _ in 1..VAD_HANGOVER_FRAMES {
        assert!(vad.update(&tone(0), DEFAULT_VAD_THRESHOLD));
    }
    assert!(!vad.update(&tone(0), DEFAULT_VAD_THRESHOLD));
    assert!(vad.update(&tone(-2000), DEFAULT_VAD_THRESHOLD));
}

#[test]
fn test_jitter_buffer_reorders_and_never_stalls() {
    let mut jb = JitterBuffer::new();
    assert!(jb.is_idle() && jb.pop().is_none());
    jb.push(10, tone(1));
    assert!(jb.pop().is_none());
    jb.push(12, tone(3));
    jb.push(11, tone(2));
    let levels: Vec<Option<i16>> = (0..3).map(|_| jb.pop().map(|f| f.samples[0])).collect();
    assert_eq!(levels, [Some(1), Some(2), Some(3)]);
    assert!(jb.pop().is_none());
    jb.push(13, tone(4));
    jb.push(14, tone(5));
    assert_eq!(jb.pop().map(|f| f.samples[0]), Some(5));

    jb.push(14 + JITTER_SLOTS as u16 * 4, tone(9));
    jb.push(15 + JITTER_SLOTS as u16 * 4, tone(10));
    assert_eq!(jb.pop().map(|f| f.samples[0]), Some(9));
    for _ in 0..JITTER_END_MISSES + 1 {
        jb.pop();
    }
    assert!(jb.is_idle());
    jb.push(u16::MAX, tone(7));
    jb.push(0, tone(8));
    assert_eq!((jb.pop().map(|f| f.samples[0]), jb.pop().map(|f| f.samples[0])), (Some(7), Some(8)));
}
