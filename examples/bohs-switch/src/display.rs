use crate::config::Config;
use std::fs::OpenOptions;
use std::io::Write;

/// Shows the pairing code in the journal and, when configured, full screen on a console.
pub fn code(code: &str, hardware_id: &str, cfg: &Config) {
    let name = &cfg.name;
    let banner = [
        format!("\n  BOHS switch \"{name}\" (hardware id {hardware_id})\n"),
        "  In Bulle, open Settings > Devices > Add a remote device".into(),
        "  and enter this code:\n".into(),
        format!("      {code}\n"),
        "  It expires in 10 minutes.\n".into(),
    ]
    .join("\n");
    println!("{banner}");
    screen(cfg, &banner);
}

/// Shows a one-line state on the console, such as "online".
pub fn status(cfg: &Config, line: &str) {
    println!("{line}");
    screen(cfg, &format!("\n  BOHS switch \"{}\"\n\n  {line}\n", cfg.name));
}

fn screen(cfg: &Config, text: &str) {
    let Some(tty) = &cfg.tty else { return };
    let written = OpenOptions::new()
        .write(true)
        .open(tty)
        .and_then(|mut out| write!(out, "\x1b[2J\x1b[H\x1b[1m{text}\x1b[0m"));
    if let Err(e) = written {
        eprintln!("cannot draw on {}: {e}", tty.display());
    }
}
