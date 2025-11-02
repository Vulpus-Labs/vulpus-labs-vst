# Beverley

Advanced bitcrusher with gamma correction and intelligent gain control.

## Overview

Beverley is a sophisticated bitcrusher plugin that combines smooth fractional bit depth reduction with gamma-based dynamic redistribution and envelope-following gain compensation. Originally inspired by the Voltage Modular bitcrusher module, it provides precise control over quantization artifacts for creative distortion and lo-fi effects.

## Features

### Core Processing
- **Interpolated Bit Depth** (1-16 bits): Fractional bit depth with smooth crossfading between integer depths eliminates zipper noise during automation
- **Gamma Correction** (-1.0 to +1.0): Power-law amplitude redistribution before quantization, reshaping the harmonic content
  - Negative values: Compress dynamics (gamma 0.1-1.0), preserving quiet details
  - Positive values: Expand dynamics (gamma 1.0-10.0), emphasizing louder signals
- **Smoothing** (0-100%): Adjustable interpolation between quantization levels
  - 0%: Hard quantization with classic stair-step artifacts
  - 50%: Smoothstep interpolation for musicality
  - 100%: Nearly transparent, preserving original waveform shape

### Gain Control
- **Fixed Gain** (0-20 dB): Pre-quantization boost for driving the effect harder
- **Auto Gain**: Envelope-following gain compensation (0.5ms attack, 50ms release)
  - Maintains consistent quantization depth regardless of input level
  - Applies automatic makeup gain after processing
  - Gate threshold: 0.005 (-46 dBFS) prevents over-compensation on silence

### Crush Modes
- **Asymmetric** (default): Full bipolar quantization using entire signal range
  - Applies gentle smoothing near zero-crossings to reduce artifacts
  - Best for material with wide dynamic range
- **Symmetric**: Independent quantization of positive and negative waveform halves
  - Creates different harmonic characteristics
  - Useful for asymmetric source material

## Audio Characteristics

- **Stereo Processing**: Independent left/right channel processing
- **Sample-Accurate Automation**: All parameters respond immediately without latency
- **Format Support**: CLAP and VST3 (via clap-wrapper)
- **I/O Configurations**: Stereo (2in/2out) and Mono-to-Stereo (1in/2out)

## Parameters

| Parameter | Range | Default | Description |
|-----------|-------|---------|-------------|
| Bit Depth | 1.00 - 16.00 | 4.00 | Fractional bit depth with 0.01 precision |
| Gamma | -1.00 - 1.00 | 0.00 | Amplitude redistribution (-1=compress, 0=neutral, +1=expand) |
| Smoothing | 0% - 100% | 0% | Quantization level interpolation amount |
| Gain | 0.0 - 20.0 dB | 0.0 dB | Fixed pre-quantization gain |
| Auto Gain | Off / On | Off | Envelope-following gain compensation |
| Symmetric | Off / On | Off | Crush mode (Off=asymmetric, On=symmetric) |

## Building

### Prerequisites
- Rust (latest stable) - https://rustup.rs
- VST3 SDK (for VST3 builds) - downloaded automatically in CI

### Build Commands

```bash
# Build both CLAP and VST3 (requires VST3 SDK)
CLAP_WRAPPER_VST3_SDK=/path/to/vst3sdk cargo run -p xtask -- bundle beverley --release

# Build CLAP only (no VST3 SDK required)
cargo run -p xtask -- bundle beverley --release
```

The bundled plugins will be in `target/bundled/`:
- `beverley.clap` - CLAP plugin
- `beverley.vst3` - VST3 plugin (if VST3 SDK is available)

### Installation

**macOS:**
```bash
cp -r target/bundled/beverley.clap ~/Library/Audio/Plug-Ins/CLAP/
cp -r target/bundled/beverley.vst3 ~/Library/Audio/Plug-Ins/VST3/
```

**Windows:**
```powershell
Copy-Item target\bundled\beverley.clap "C:\Program Files\Common Files\CLAP\"
Copy-Item target\bundled\beverley.vst3 "C:\Program Files\Common Files\VST3\"
```

**Linux:**
```bash
cp -r target/bundled/beverley.clap ~/.clap/
cp -r target/bundled/beverley.vst3 ~/.vst3/
```

## Technology Stack

- **nih-plug** (ISC license): Modern Rust plugin framework with CLAP support
- **clap-wrapper** (MIT/Apache-2.0): VST3 wrapper for CLAP plugins
- **atomic_float** (MIT/Apache-2.0/Unlicense): Lock-free parameter handling

All dependencies use permissive licenses. See `THIRD-PARTY-LICENSES.md` in the repository root.

## Algorithm Details

### Fractional Bit Depth
Bit depth values between integers (e.g., 4.5 bits) crossfade between two quantizers:
- Low quantizer: floor(4.5) = 4 bits (15 levels)
- High quantizer: ceil(4.5) = 5 bits (31 levels)
- Mix: 50% low + 50% high

### Gamma Transform
Applied as a power function before and after quantization:
- Forward: `x^(1/gamma)` - redistributes amplitude before quantization
- Inverse: `x^gamma` - restores amplitude curve after quantization
- Gamma value: `10^(parameter)` where parameter ranges from -1 to +1
  - At -1.0: gamma = 0.1 (strong compression)
  - At 0.0: gamma = 1.0 (no change)
  - At +1.0: gamma = 10.0 (strong expansion)

### Smoothing Function
- 0-50%: Linear blend from hard quantization to smoothstep
- 50-100%: Blend from smoothstep to hard step function
- Smoothstep formula: `3x² - 2x³`

## License

MIT License - see LICENSE file in repository root.

Copyright (c) 2024 Vulpus Labs
