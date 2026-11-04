# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-11-04

### Added
- **Bulle Open Hardware Standard (BOHS-01)**: Comprehensive formal specifications in `docs/spec/` defining low-latency audio streaming (Opus 16kHz, 20ms frames) and bidirectional Reverse MCP tool execution over WebSocket (`bohs.v1`).
- **Physical Circuit Design**: Declarative 38mm 4-layer circular PCB designed with `tscircuit` and Fusible, featuring an ESP32-S3 (8MB PSRAM), USB-C charging with TP4056, ultra-low quiescent 3.3V LDO, I2S INMP441 MEMS microphone, I2S MAX98357A amplifier, GC9A01 1.28" round LCD connector, and capacitive touch wake pad.
- **Electrical Rule Checking**: Verified with Fusible's fail-closed Fuse Contract with zero DRC violations, zero strapping pin collisions, and verified decoupling capacitor bypass rules.
- **Bare-Metal Embassy Firmware**: Asynchronous Rust runtime for the ESP32-S3 with integer trigonometric vector rendering for Bulle's circular avatar states (`Idle`, `Listening`, `Thinking`, `Speaking`, `Error`), I2S DMA audio pipeline, and BOHS-01 protocol dispatcher.
- **Mechanical Enclosures**: Generated snap-fit 3D case models (`bodies/pastille_case.{step,stl,glb,brep}`) and export presets.
