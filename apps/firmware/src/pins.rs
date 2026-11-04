#![allow(dead_code)]

pub mod gpio {
    pub const STATUS_LED: u8 = 0;
    pub const TOUCH_WAKE: u8 = 7;
    pub const HAPTIC_PWM: u8 = 3;
    pub const VBUS_SENSE: u8 = 1;
    pub const BATT_SENSE: u8 = 2;

    pub const MIC_SCK: u8 = 4;
    pub const MIC_WS: u8 = 5;
    pub const MIC_SD: u8 = 6;

    pub const SPK_BCLK: u8 = 15;
    pub const SPK_LRCLK: u8 = 16;
    pub const SPK_DIN: u8 = 17;
    pub const SPK_SD_MODE: u8 = 18;

    pub const LCD_CS: u8 = 10;
    pub const LCD_DC: u8 = 11;
    pub const LCD_SCLK: u8 = 12;
    pub const LCD_MOSI: u8 = 13;
    pub const LCD_RST: u8 = 14;
    pub const LCD_BL: u8 = 21;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoardPins {
    pub led: u8,
    pub touch_wake: u8,
    pub haptic: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PowerPins {
    pub vbus_sense: u8,
    pub batt_sense: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioPins {
    pub mic_sck: u8,
    pub mic_ws: u8,
    pub mic_sd: u8,
    pub spk_bclk: u8,
    pub spk_lrclk: u8,
    pub spk_din: u8,
    pub spk_sd_mode: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayPins {
    pub cs: u8,
    pub dc: u8,
    pub sclk: u8,
    pub mosi: u8,
    pub rst: u8,
    pub backlight: u8,
}

pub const BOARD_PINS: BoardPins = BoardPins {
    led: gpio::STATUS_LED,
    touch_wake: gpio::TOUCH_WAKE,
    haptic: gpio::HAPTIC_PWM,
};

pub const POWER_PINS: PowerPins = PowerPins {
    vbus_sense: gpio::VBUS_SENSE,
    batt_sense: gpio::BATT_SENSE,
};

pub const AUDIO_PINS: AudioPins = AudioPins {
    mic_sck: gpio::MIC_SCK,
    mic_ws: gpio::MIC_WS,
    mic_sd: gpio::MIC_SD,
    spk_bclk: gpio::SPK_BCLK,
    spk_lrclk: gpio::SPK_LRCLK,
    spk_din: gpio::SPK_DIN,
    spk_sd_mode: gpio::SPK_SD_MODE,
};

pub const DISPLAY_PINS: DisplayPins = DisplayPins {
    cs: gpio::LCD_CS,
    dc: gpio::LCD_DC,
    sclk: gpio::LCD_SCLK,
    mosi: gpio::LCD_MOSI,
    rst: gpio::LCD_RST,
    backlight: gpio::LCD_BL,
};
