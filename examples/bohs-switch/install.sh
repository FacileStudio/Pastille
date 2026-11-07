#!/bin/sh
# Installs bohs-switch on Raspberry Pi OS (Bookworm or Trixie). Run as root on the Pi:
#   sudo sh install.sh                  uses bohs-switch-<arch> next to this script
#   sudo BOHS_NAME="Desk Lamp" sh install.sh
set -eu

here=$(cd "$(dirname "$0")" && pwd)
case "$(uname -m)" in
  aarch64) arch=arm64 ;;
  armv7l) arch=armhf ;;
  *) echo "bohs-switch needs a 64-bit or ARMv7 Raspberry Pi OS, not $(uname -m)" >&2; exit 1 ;;
esac
[ "$(id -u)" -eq 0 ] || { echo "run as root: sudo sh $0" >&2; exit 1; }

binary="$here/bohs-switch-$arch"
[ -f "$binary" ] || binary="$here/dist/bohs-switch-$arch"
[ -f "$binary" ] || { echo "missing $binary: build it with build.sh and copy it next to this script" >&2; exit 1; }

install -m 0755 "$binary" /usr/local/bin/bohs-switch
getent group gpio >/dev/null || groupadd --system gpio
install -m 0644 "$here/systemd/60-bohs-gpio.rules" /etc/udev/rules.d/60-bohs-gpio.rules
udevadm control --reload && udevadm trigger --subsystem-match=gpio

case "${BOHS_NAME:-}" in
  *[!A-Za-z0-9\ _.\'-]*) echo "BOHS_NAME takes letters, digits, spaces and . _ ' - only" >&2; exit 1 ;;
esac
case "${BOHS_LINE:-17}" in
  ''|*[!0-9]*) echo "BOHS_LINE must be a BCM GPIO number such as 17" >&2; exit 1 ;;
esac
case "${BOHS_ACTIVE_LOW:-false}" in
  true|false) ;;
  *) echo "BOHS_ACTIVE_LOW must be true or false" >&2; exit 1 ;;
esac

if [ ! -f /etc/bohs-switch.toml ]; then
  sed -e "s/^name = .*/name = \"${BOHS_NAME:-Living Room Light}\"/" \
      -e "s/^line = .*/line = ${BOHS_LINE:-17}/" \
      -e "s/^active_low = .*/active_low = ${BOHS_ACTIVE_LOW:-false}/" \
      "$here/bohs-switch.toml.example" > /etc/bohs-switch.toml
fi

install -m 0644 "$here/systemd/bohs-switch.service" /etc/systemd/system/bohs-switch.service
systemctl daemon-reload
systemctl enable bohs-switch
systemctl restart bohs-switch
sleep 3
journalctl -u bohs-switch -n 20 --no-pager -o cat
