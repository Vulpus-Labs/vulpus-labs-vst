# Changelog

All notable changes to Beverley will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

Nothing yet.

## [0.1.0] - 2024-11-02

### Added
- **Interpolated bit depth reduction** (1-16 bits)
  - Fractional bit depth with smooth crossfading between integer depths
  - Eliminates zipper noise during parameter automation
- **Gamma correction** (-1.0 to +1.0)
  - Power-law amplitude redistribution before quantization
  - Negative values compress dynamics (gamma 0.1-1.0)
  - Positive values expand dynamics (gamma 1.0-10.0)
- **Smoothing control** (0-100%)
  - Adjustable interpolation between quantization levels
  - 0% = hard quantization with classic stair-step artifacts
  - 50% = smoothstep interpolation for musicality
  - 100% = nearly transparent processing
- **Dual gain modes**
  - Fixed gain (0-20 dB) for pre-quantization boost
  - Auto-gain with envelope follower (0.5ms attack, 50ms release)
  - Gate threshold at -46 dBFS prevents over-compensation
- **Crush modes**
  - Asymmetric: Full bipolar quantization (default)
  - Symmetric: Independent positive/negative quantization
- **Plugin format support**
  - CLAP native plugin format
  - VST3 via clap-wrapper (fat binary, no dynamic loading)
- **Platform support**
  - Windows (x86_64)
  - macOS ARM64 (Apple Silicon)
- **Audio I/O configurations**
  - Stereo (2in/2out)
  - Mono-to-Stereo (1in/2out)
- **Sample-accurate automation** for all parameters
- **Independent stereo processing** (separate L/R channels)

### Technical
- Built with nih-plug framework (ISC license)
- VST3 support via clap-wrapper (MIT/Apache-2.0 dual license)
- Uses VST3 SDK v3.8+ (MIT license)
- MIT licensed project
- Continuous integration with GitHub Actions
- Automatic releases on version tags

---

## Version History

[Unreleased]: https://github.com/vulpuslabs/beverley/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/vulpuslabs/beverley/releases/tag/v0.1.0
