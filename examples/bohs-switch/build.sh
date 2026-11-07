#!/bin/sh
set -eu
cd "$(dirname "$0")"
docker run --rm -v "$PWD:/src" -w /src -e CARGO_HOME=/src/target/cargo-home rust:1.98-bookworm sh -eu -c '
  apt-get update -qq
  apt-get install -qq -y gcc-aarch64-linux-gnu gcc-arm-linux-gnueabihf >/dev/null
  rustup target add aarch64-unknown-linux-gnu armv7-unknown-linux-gnueabihf
  export CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc CC_armv7_unknown_linux_gnueabihf=arm-linux-gnueabihf-gcc
  for target in aarch64-unknown-linux-gnu armv7-unknown-linux-gnueabihf; do
    cargo build --locked --release --target "$target"
  done
  chown -R "$(stat -c %u:%g /src)" target
'
mkdir -p dist
cp target/aarch64-unknown-linux-gnu/release/bohs-switch dist/bohs-switch-arm64
cp target/armv7-unknown-linux-gnueabihf/release/bohs-switch dist/bohs-switch-armhf
ls -l dist
