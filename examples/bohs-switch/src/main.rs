mod config;
mod creds;
mod display;
mod identity;
mod output;
mod pairing;
mod rpc;
mod session;
mod tools;

#[cfg(test)]
mod tests;

use config::Config;
use output::{Gpio, Memory, Output};
use rpc::Switch;
use session::End;
use std::path::PathBuf;
use std::process::ExitCode;
use std::thread::sleep;
use std::time::Duration;

const RETRY: Duration = Duration::from_secs(5);

struct Args {
    config: PathBuf,
    dry_run: bool,
}

fn args() -> Result<Args, String> {
    let mut out = Args {
        config: config::DEFAULT_PATH.into(),
        dry_run: false,
    };
    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--config" => out.config = it.next().ok_or("--config needs a path")?.into(),
            "--dry-run" => out.dry_run = true,
            _ => return Err("usage: bohs-switch [--config PATH] [--dry-run]".into()),
        }
    }
    Ok(out)
}

fn main() -> ExitCode {
    let started = args().and_then(|a| {
        let cfg = Config::load(&a.config)?;
        let out: Box<dyn Output> = if a.dry_run {
            Box::new(Memory::default())
        } else {
            Box::new(Gpio::open(&cfg.chip, cfg.line, cfg.active_low, cfg.initial_on)?)
        };
        Ok((cfg, out, identity::hardware_id()?))
    });
    match started {
        Ok((cfg, mut out, hardware_id)) => serve(&cfg, out.as_mut(), &hardware_id),
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

fn serve(cfg: &Config, out: &mut dyn Output, hardware_id: &str) -> ExitCode {
    loop {
        let Some(creds) = creds::load(&cfg.state_dir).or_else(|| paired(cfg, hardware_id)) else {
            continue;
        };
        let mut sw = Switch {
            name: &creds.name,
            out: &mut *out,
        };
        let mut online = || display::status(cfg, &format!("Online as {} on {}", creds.name, cfg.base()));
        match session::run(&creds, &mut sw, &mut online) {
            End::Revoked => {
                creds::forget(&cfg.state_dir);
                display::status(cfg, "This device was removed from Bulle. Pairing again.");
            }
            End::Dropped(reason) => {
                display::status(cfg, &format!("Offline ({reason}). Retrying."));
                sleep(RETRY);
            }
        }
    }
}

fn paired(cfg: &Config, hardware_id: &str) -> Option<creds::Credentials> {
    match pairing::pair(cfg, hardware_id) {
        Ok(Some(creds)) => {
            if let Err(e) = creds::save(&cfg.state_dir, &creds) {
                display::status(cfg, &format!("Paired, but the token is lost at the next reboot: {e}"));
            }
            Some(creds)
        }
        Ok(None) => None,
        Err(e) => {
            display::status(cfg, &format!("{e}. Retrying."));
            sleep(RETRY);
            None
        }
    }
}
