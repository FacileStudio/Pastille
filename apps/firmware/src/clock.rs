use core::cell::Cell;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::blocking_mutex::Mutex;

/// Wall time below which a clock reading is rejected: 2026-11-01T00:00:00Z, the v0.4.0 build
/// floor. Raise it at each release.
pub const TIME_FLOOR_UNIX_S: u64 = 1_793_491_200;

/// Unix time in milliseconds at uptime zero, once SNTP has answered.
static BOOT_UNIX_MS: Mutex<CriticalSectionRawMutex, Cell<Option<u64>>> = Mutex::new(Cell::new(None));

/// Records the wall time `unix_ms` observed at `uptime_ms`; readings before the floor are refused.
pub fn set_wall_time(unix_ms: u64, uptime_ms: u64) -> bool {
    let valid = unix_ms / 1000 >= TIME_FLOOR_UNIX_S && unix_ms >= uptime_ms;
    if valid {
        BOOT_UNIX_MS.lock(|cell| cell.set(Some(unix_ms - uptime_ms)));
    }
    valid
}

/// Wall time in milliseconds at `uptime_ms`, or `None` before the first SNTP answer.
pub fn wall_ms(uptime_ms: u64) -> Option<u64> {
    BOOT_UNIX_MS.lock(Cell::get).map(|boot| boot + uptime_ms)
}

/// Milliseconds since boot from the device timer. Host builds have no timer and report zero.
pub fn uptime_ms() -> u64 {
    #[cfg(target_arch = "xtensa")]
    return embassy_time::Instant::now().as_millis();
    #[cfg(not(target_arch = "xtensa"))]
    return 0;
}

/// The clock embedded-tls checks certificate validity against: wall time once known.
pub struct WallClock;

impl embedded_tls::TlsClock for WallClock {
    fn now() -> Option<u64> {
        wall_ms(uptime_ms()).map(|ms| ms / 1000)
    }
}
