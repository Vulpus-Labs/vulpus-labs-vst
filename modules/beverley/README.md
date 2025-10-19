# Beverley VST3

Advanced bitcrusher and harmonic distortion VST3 plugin.

## Overview

Beverley is a port of the Voltage Modular bitcrusher module to VST3 format. It combines intelligent gain control, gamma correction, and smoothed quantization to create harmonically rich distortion suitable for guitars, drums, synths, and other audio sources.

## Features

- **Interpolated Bit Depth**: Smooth parameter sweeps between 1-16 bits without zipper noise
- **Gamma Correction**: Power-law transform that shifts amplitude distribution for different harmonic characteristics
- **Smoothing Control**: Adjustable transitions from hard quantization to smooth reconstruction
- **Auto-Gain**: Envelope-following gain compensation for consistent quantization range
- **Crush Modes**: Asymmetric (full signal range) or Symmetric (independent positive/negative quantization)
- **CV Modulation**: Real-time modulation of bit depth and gamma parameters

## Building

### Prerequisites

- Rust (latest stable) - install from https://rustup.rs
- A C compiler (for native dependencies)

### Build Commands

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# The plugin will be in target/release/
```

### Installation

After building, copy the VST3 bundle to your system's VST3 folder:

- **macOS**: `~/Library/Audio/Plug-Ins/VST3/`
- **Windows**: `C:\Program Files\Common Files\VST3\`
- **Linux**: `~/.vst3/`

## Technology Stack

- **nih-plug**: Modern Rust framework for VST3/CLAP plugins
- **nih-plug-vizia**: GUI framework built on VIZIA for rich user interfaces
- **atomic_float**: Thread-safe parameter handling

## Parameters

### Main Controls
- **Bit Depth** (1-16): Number of quantization levels
- **Gamma** (-1 to +1): Power-law transform amount
- **Smoothing** (0-1): Transition smoothness (0=hard, 0.5=smoothstep, 1=transparent)
- **Gain** (0-20dB): Fixed input gain
- **Auto Gain**: Toggle for envelope-following gain correction

### Modulation
- **Bit Depth Mod**: CV modulation amount for bit depth
- **Gamma Mod**: CV modulation amount for gamma

### Mode
- **Crush Mode**: Asymmetric/Symmetric quantization

## Development Status

This is a work in progress. The project structure is set up but implementation is pending.

## License

Copyright © 2025 Vulpus Labs
