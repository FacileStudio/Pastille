//! Full-duplex audio: the SPH0645LM4H-B microphone on I2S0 and the MAX98357A amplifier on I2S1,
//! each with its own DMA stream, both clocked at 16 kHz.

use super::board_pin;
use super::net::AUDIO_TX_CHANNEL;
use crate::audio::{AudioFrame, SAMPLE_RATE_HZ};
use crate::jitter::JitterBuffer;
use crate::link::{AUDIO_RX_CHANNEL, CAPTURE_ENABLED};
use crate::mic::{decode_mic_frame, encode_speaker_frame, DcBlocker, MIC_FRAME_BYTES, SPEAKER_FRAME_BYTES};
use crate::pins::BOARD_PINS;
use core::sync::atomic::Ordering;
use embassy_time::{Duration, Timer};
use esp_hal::dma::{DmaRxStreamBuf, DmaTxStreamBuf};
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::i2s::master::{Channels, DataFormat, I2s, I2sRx, I2sRxDmaTransfer, I2sTx, TdmConfig};
use esp_hal::peripherals::{DMA_CH0, DMA_CH1, I2S0, I2S1};
use esp_hal::time::Rate;
use esp_hal::{dma_rx_stream_buffer, dma_tx_stream_buffer, Async};

/// Silent frames after the last downstream frame before the amplifier is shut down (200 ms).
const AMP_IDLE_FRAMES: u8 = 10;
const CAPTURE_POLL: Duration = Duration::from_millis(50);

fn tdm(format: DataFormat) -> TdmConfig {
    TdmConfig::new_tdm_philips()
        .with_sample_rate(Rate::from_hz(SAMPLE_RATE_HZ))
        .with_data_format(format)
        .with_channels(Channels::STEREO)
}

/// The microphone receiver: 32-bit slots, so BCLK runs at 1.024 MHz, the SPH0645 minimum.
pub fn microphone(i2s: I2S0<'static>, dma: DMA_CH0<'static>) -> Option<I2sRx<'static, Async>> {
    let pins = BOARD_PINS.general;
    let i2s = I2s::new(i2s, dma, tdm(DataFormat::Data32Channel32)).ok()?.into_async();
    Some(
        i2s.i2s_rx
            .with_bclk(board_pin(pins.mic_sck))
            .with_ws(board_pin(pins.mic_ws))
            .with_din(board_pin(pins.mic_sd))
            .build(),
    )
}

/// The amplifier transmitter (16-bit stereo slots) and its `SD_MODE` line, starting shut down.
pub fn speaker(i2s: I2S1<'static>, dma: DMA_CH1<'static>) -> Option<(I2sTx<'static, Async>, Output<'static>)> {
    let pins = BOARD_PINS.general;
    let i2s = I2s::new(i2s, dma, tdm(DataFormat::Data16Channel16)).ok()?.into_async();
    let tx = i2s
        .i2s_tx
        .with_bclk(board_pin(pins.spk_bclk))
        .with_ws(board_pin(pins.spk_lrclk))
        .with_dout(board_pin(pins.spk_din));
    let shutdown = Output::new(board_pin(pins.spk_sd_mode), Level::Low, OutputConfig::default());
    Some((tx.build(), shutdown))
}

/// Decoding state for the microphone stream.
struct Capture {
    raw: [u8; MIC_FRAME_BYTES],
    filled: usize,
    dc: DcBlocker,
    frame: AudioFrame,
}

impl Capture {
    /// Reads from the running transfer until capture is disabled or the DMA fails.
    async fn run(&mut self, transfer: &mut I2sRxDmaTransfer<'static, Async, DmaRxStreamBuf>) {
        self.filled = 0;
        while CAPTURE_ENABLED.load(Ordering::Relaxed) {
            if transfer.available_bytes() == 0 && transfer.wait_for_available_async().await.is_err() {
                return;
            }
            self.filled += transfer.pop(&mut self.raw[self.filled..]);
            if self.filled == MIC_FRAME_BYTES && decode_mic_frame(&self.raw, &mut self.dc, &mut self.frame).is_ok() {
                let _ = AUDIO_TX_CHANNEL.try_send(self.frame);
            }
            self.filled %= MIC_FRAME_BYTES;
        }
    }
}

/// Captures while a session needs the microphone and stops the clock otherwise, which puts the
/// SPH0645 into its sleep mode.
#[embassy_executor::task]
pub async fn mic_task(mut rx: I2sRx<'static, Async>) {
    let mut buffer: DmaRxStreamBuf = dma_rx_stream_buffer!(MIC_FRAME_BYTES * 4, MIC_FRAME_BYTES / 2);
    let mut capture =
        Capture { raw: [0; MIC_FRAME_BYTES], filled: 0, dc: DcBlocker::default(), frame: AudioFrame::new() };
    loop {
        while !CAPTURE_ENABLED.load(Ordering::Relaxed) {
            Timer::after(CAPTURE_POLL).await;
        }
        match rx.read(buffer) {
            Ok(mut transfer) => {
                capture.run(&mut transfer).await;
                (rx, buffer) = transfer.stop();
            }
            Err((_, idle_rx, idle_buffer)) => {
                (rx, buffer) = (idle_rx, idle_buffer);
                Timer::after(CAPTURE_POLL).await;
            }
        }
    }
}

/// Playback state: the jitter buffer, the frame being pushed and the amplifier idle count.
struct Player {
    jitter: JitterBuffer,
    out: [u8; SPEAKER_FRAME_BYTES],
    pending: usize,
    idle: u8,
}

impl Player {
    /// Takes the next frame or silence and switches the amplifier on or off.
    fn refill(&mut self, shutdown: &mut Output<'static>) {
        while let Ok((seq, frame)) = AUDIO_RX_CHANNEL.try_receive() {
            self.jitter.push(seq, frame);
        }
        let frame = self.jitter.pop();
        let active = frame.is_some() || !self.jitter.is_idle();
        self.idle = if active { 0 } else { self.idle.saturating_add(1) };
        let _ = encode_speaker_frame(&frame.unwrap_or(AudioFrame::new()), &mut self.out);
        shutdown.set_level(if self.idle < AMP_IDLE_FRAMES { Level::High } else { Level::Low });
        self.pending = SPEAKER_FRAME_BYTES;
    }
}

/// Plays downstream frames at the I2S clock, silence when none is due, and drives `SD_MODE`
/// high only while there is audio to play.
#[embassy_executor::task]
pub async fn speaker_task(tx: I2sTx<'static, Async>, mut shutdown: Output<'static>) {
    let buffer: DmaTxStreamBuf = dma_tx_stream_buffer!(SPEAKER_FRAME_BYTES * 4, SPEAKER_FRAME_BYTES);
    let Ok(mut transfer) = tx.write(buffer) else { return };
    let mut player =
        Player { jitter: JitterBuffer::new(), out: [0; SPEAKER_FRAME_BYTES], pending: 0, idle: AMP_IDLE_FRAMES };
    loop {
        if player.pending == 0 {
            player.refill(&mut shutdown);
        }
        if transfer.available_bytes() == 0 && transfer.wait_for_available_async().await.is_err() {
            return;
        }
        player.pending -= transfer.push(&player.out[SPEAKER_FRAME_BYTES - player.pending..]);
    }
}
