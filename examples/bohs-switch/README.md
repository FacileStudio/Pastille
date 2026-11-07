# bohs-switch

A BOHS-01 field device on a Raspberry Pi: a lamp that Bulle switches when you ask it to.

You plug a lamp into a relay, wire the relay to the Pi, and pair the Pi with a Bulle account by typing a code. From then on, "could you turn on the light?" in any Bulle chat (web, desktop or mobile) switches the lamp. The Pi and the Bulle user can be in different cities: the Pi dials out to `bulle.facile.studio`, so no port forwarding is needed.

It implements the field profile of [BOHS-01-ARCH.md](../../docs/spec/BOHS-01-ARCH.md): code pairing, the `switch.set` and `switch.get` tools, and the `bohs.v1` socket.

## Hardware

- Raspberry Pi 3 Model B+ (any Pi with a 40-pin header and Raspberry Pi OS works; on a Pi 5 set `chip = "/dev/gpiochip4"`).
- A relay the Pi can drive at 3.3 V, and a lamp.

Keep mains voltage away from the Pi and from your hands. Use one of these, in this order:

1. **An enclosed relay power box with a low-voltage control input**, such as the Digital Loggers "IoT Power Relay". The lamp plugs into its socket and two wires go to the Pi. Check that the version you buy has sockets for your country.
2. **A low-voltage lamp**: a USB or 12 V LED lamp on a small relay or MOSFET module. Nothing on the desk carries mains.

Do not wire a bare relay module to 230 V. Cheap boards often leave the mains traces exposed underneath.

Wiring, with the default config:

| Pi header pin | Signal | Relay input |
| :--- | :--- | :--- |
| 11 | GPIO17 | `+` / `IN` |
| 9 | Ground | `-` / `GND` |

Many relay boards switch on when their input is LOW. If the lamp is on when it should be off, set `active_low = true`.

The daemon owns the GPIO line only while it runs. Before it starts (during boot) and for the 5 seconds after a crash, the pin takes the Pi's default, a pull-down input, and an active-low relay board then switches the lamp **on**. To fix the boot state, add this line to `/boot/firmware/config.txt` (`dh` drives the pin high, which is off for an active-low board; use `dl` for an active-high board that must stay off):

```
gpio=17=op,dh
```

The state is not saved: after a reboot the lamp takes `initial_on`.

## Build the binary

On any Linux machine with Docker:

```sh
./build.sh
```

It writes `dist/bohs-switch-arm64` (64-bit Raspberry Pi OS) and `dist/bohs-switch-armhf` (32-bit), linked against Debian Bookworm's glibc, so they run on Bookworm and Trixie.

## Path A: stock Raspberry Pi OS (simplest)

1. Flash Raspberry Pi OS Lite with Raspberry Pi Imager. In its customisation screen, set the Wi-Fi network and turn on SSH.
2. Copy this directory, with `dist/`, to the Pi, then install:

   ```sh
   scp -r examples/bohs-switch pi@raspberrypi.local:
   ssh pi@raspberrypi.local 'cd bohs-switch && sudo BOHS_NAME="Living Room Light" sh install.sh'
   ```

3. The install prints the pairing code. Later, read it with `journalctl -u bohs-switch -f`.

`install.sh` installs `/usr/local/bin/bohs-switch`, `/etc/bohs-switch.toml` (kept on reinstall), a udev rule that gives the `gpio` group the GPIO chips, and the `bohs-switch` systemd service. The service runs as a dynamic user with only the `gpio` and `tty` groups. Its token lives in `/var/lib/bohs-switch`, mode 0700.

## Path B: a dedicated image

`image/` builds a minimal Debian Trixie image for the Pi 3 with [rpi-image-gen](https://github.com/raspberrypi/rpi-image-gen). It boots straight into the switch and draws the pairing code full screen on the HDMI output, so a camera can film it.

1. Run `./build.sh` first. The image takes `dist/bohs-switch-arm64`.
2. For Wi-Fi, write an iwd profile named after your network, such as `image/wifi/Home.psk` (this directory is git-ignored):

   ```ini
   [Security]
   Passphrase=your-wifi-password
   ```

   Ethernet works with no profile.

3. On a 64-bit Debian or Raspberry Pi OS host (rpi-image-gen runs best on an arm64 Pi):

   ```sh
   git clone https://github.com/raspberrypi/rpi-image-gen && cd rpi-image-gen && sudo ./install_deps.sh
   ./rpi-image-gen build -S ../Pastille/examples/bohs-switch/image -c bohs-switch.yaml \
     -- IGconf_iwd_profile=$PWD/../Pastille/examples/bohs-switch/image/wifi/Home.psk
   ```

4. Flash `work/image-bohs-switch-pi3/bohs-switch-pi3.img` with Raspberry Pi Imager ("Use custom").

The image has no login password and no SSH. Everything it does is in the systemd unit. The layer is written against rpi-image-gen `fb44a4c` (2026-11-05) but has not been built yet. Build it once before the demo.

## Pair it

1. On the Pi's screen or in its journal:

   ```
   In Bulle, open Settings > Devices > Add a remote device
   and enter this code:

       KX7Q-42MN
   ```

2. In Bulle: Settings, Devices, "Add a remote device". Type the code, and a name if you want another one. Bulle finds the device by this name.
3. Within 3 seconds the Pi says `Online as Living Room Light`.

The code lives 10 minutes, then the Pi shows a new one. If the Pi shows a new code right after you entered the old one, the reply was lost on the way (the gateway hands the token out once): remove the device in Bulle and enter the new code. Removing the device in Bulle makes the Pi forget its token and show a new code.

## Demo runbook

The day before:

- Pair the Pi to your own account and ask Bulle to switch the lamp. This checks the wiring, the relay polarity and the network. Then remove it from your account.
- Prefer Ethernet. The 3B+ Wi-Fi is the weakest link.
- Make sure the guest's Bulle account is on `bulle.facile.studio` and can run the agent.

Live:

1. Power the Pi on camera. The code appears.
2. The guest adds the device in Settings, Devices, with the code.
3. The guest types "could you turn on the light please?" in a new chat.
4. The lamp turns on. "Turn it off" works too, as does "is the light on?".

If it fails: `journalctl -u bohs-switch -f` on the Pi shows every state change. "Offline" means the network or the clock: TLS needs the time, which the Pi gets from the network after boot.

## Try it without a Pi

```sh
cp bohs-switch.toml.example /tmp/switch.toml
echo 'state_dir = "/tmp/bohs-switch"' >> /tmp/switch.toml
cargo run -- --dry-run --config /tmp/switch.toml
```

`--dry-run` replaces the GPIO line with a log line. Pair it like a real Pi; set `gateway` to a local Bulle to test against your own server.

## Develop

```sh
cargo test
cargo clippy --all-targets -- -D warnings
```

The tests cover the JSON-RPC rules of [BOHS-01-WIRE.md](../../docs/spec/BOHS-01-WIRE.md), the switch tools and the hardware ID. The device name in the switch's answers is the one set when it paired; a rename in Bulle shows after the next pairing. The daemon was checked end to end against Bulle's real hardware module, with pairing, claim, reconnect with the saved token and keepalive pings. It has not run on a Pi yet.
