use gpiocdev::Request;
use gpiocdev::line::Value;

/// A load the switch drives, such as a relay on a GPIO line.
pub trait Output {
    /// Switches the load and answers an error a person can read when the hardware refuses.
    fn set(&mut self, on: bool) -> Result<(), String>;
    /// Answers whether the load is on.
    fn is_on(&self) -> bool;
}

/// One GPIO line requested as an output through the kernel's character device.
pub struct Gpio {
    request: Request,
    on: bool,
}

impl Gpio {
    /// Requests `line` on `chip` and drives it to `initial` straight away.
    pub fn open(chip: &str, line: u32, active_low: bool, initial: bool) -> Result<Self, String> {
        let mut builder = Request::builder();
        builder
            .on_chip(chip)
            .with_consumer("bohs-switch")
            .with_line(line)
            .as_output(value(initial));
        if active_low {
            builder.as_active_low();
        }
        let request = builder
            .request()
            .map_err(|e| format!("cannot open GPIO {line} on {chip}: {e}"))?;
        Ok(Self { request, on: initial })
    }
}

impl Output for Gpio {
    fn set(&mut self, on: bool) -> Result<(), String> {
        self.request
            .set_lone_value(value(on))
            .map_err(|e| format!("the GPIO line refused: {e}"))?;
        self.on = on;
        Ok(())
    }

    fn is_on(&self) -> bool {
        self.on
    }
}

fn value(on: bool) -> Value {
    if on { Value::Active } else { Value::Inactive }
}

/// A load that only lives in memory, for `--dry-run` and tests.
#[derive(Default)]
pub struct Memory {
    on: bool,
}

impl Output for Memory {
    fn set(&mut self, on: bool) -> Result<(), String> {
        self.on = on;
        println!("[dry-run] load {}", if on { "on" } else { "off" });
        Ok(())
    }

    fn is_on(&self) -> bool {
        self.on
    }
}
