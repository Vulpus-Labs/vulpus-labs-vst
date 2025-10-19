# Beverley VST3 - Implementation Notes

## Overview
This is a first-pass implementation of the Beverley bitcrusher DSP engine, ported from the Java/Voltage Modular version to Rust/VST3.

## Completed Components

### 1. DSP Engine (`src/dsp.rs`)

#### `BitCrusher`
- Quantizes audio to a specified number of levels
- Implements smoothing via a "sigmoidish" function that blends between:
  - Linear response (steepness = 0)
  - Smoothstep interpolation (steepness = 0.5)
  - Hard step quantization (steepness = 1.0)
- Direct port from Java implementation

#### `ExponentialPeak`
- Envelope follower for automatic gain compensation
- Attack time: 0.5ms (fast response to transients)
- Release time: 50ms (smooth gain reduction)
- Gate threshold: 0.005 (below this applies max gain)
- Target level: 1.0 (normalized to ±1 range)

#### `InterpolatedBitCrusher`
- Main DSP processor combining all components
- Features:
  - **Fractional bit depth**: Interpolates between adjacent integer bit depths for smooth parameter changes
  - **Gamma correction**: Power-law transform (10^gamma) to shift amplitude distribution
  - **Two crush modes**:
    - Asymmetric: Quantizes full signal range together
    - Symmetric: Independently quantizes positive/negative halves
  - **Auto-gain**: Optional envelope-following gain compensation
  - **Manual gain**: 0-20dB fixed gain control

### 2. Plugin Parameters (`src/lib.rs`)

All parameters include:
- Smooth parameter interpolation (10ms linear smoothing)
- Proper value formatters for display
- VST3/CLAP automation support

#### Main Parameters
- **Bit Depth** (1.0-16.0): Fractional bit depth with 0.01 step resolution
- **Gamma** (-1.0 to +1.0): Maps exponentially to 0.1-10 gamma values
- **Smoothing** (0.0-1.0): Quantization step smoothness
- **Gain** (0-20 dB): Fixed input gain
- **Auto Gain** (bool): Enable envelope-following gain
- **Crush Mode** (bool): Asymmetric/Symmetric quantization

#### Modulation Parameters (Not Yet Implemented)
- **Depth Mod Amount** (-1.0 to +1.0): For CV modulation
- **Gamma Mod Amount** (-1.0 to +1.0): For CV modulation

### 3. Audio Processing

The `process()` method:
1. Extracts smoothed parameter values per-sample
2. Updates both left/right crusher instances
3. Processes stereo audio with independent left/right processing
4. Handles mono→stereo conversion (mono input copied to both channels)

**Note**: Currently processes at ±1.0 range (standard VST3), not ±5V like Voltage Modular. The 0.2/5.0 scaling factors in the code are legacy from the port and should be reviewed.

## Differences from Original

### Improvements
1. **Type safety**: Rust's type system prevents many runtime errors
2. **Memory safety**: No garbage collection overhead, deterministic performance
3. **Sample-accurate automation**: Parameters update per-sample, not per-buffer
4. **Modern parameter handling**: Built-in smoothing and formatting

### Pending Features
1. **CV Modulation inputs**: The mod amount parameters exist but aren't connected to CV inputs yet (VST3 doesn't have CV like modular systems)
2. **GUI**: No graphical interface implemented yet
3. **Presets**: No preset system beyond DAW automation

## Testing Needed

1. **Audio correctness**: Compare output with original Voltage Modular module
2. **Parameter ranges**: Verify gamma mapping (10^x) produces expected results
3. **Auto-gain behavior**: Check envelope follower response times
4. **Crush modes**: Verify asymmetric vs symmetric quantization
5. **Edge cases**:
   - Bit depth = 1 (maximum crush)
   - Bit depth = 16 (minimal crush)
   - Extreme gamma values (±1.0)
   - Zero/silent input with auto-gain

## Known Issues

1. **Performance**: Setting parameters per-sample may be inefficient
   - Consider checking if parameters changed before updating
   - Could cache parameter values per-buffer instead

2. **Scaling**: The ±5V to ±1 scaling is currently hardcoded
   - VST3 standard is ±1.0, not ±5V
   - Consider removing the 0.2/5.0 scaling factors

3. **Modulation**: CV modulation parameters defined but not used
   - In VST3, this would typically be handled by:
     - MIDI CC mapping
     - DAW automation
     - MPE (MIDI Polyphonic Expression)
     - Or a modulation matrix in the GUI

## Next Steps

### High Priority
1. **Build and test** - Verify it compiles and produces audio
2. **Listen test** - Compare with original module
3. **Fix audio scaling** - Confirm ±1.0 range is correct

### Medium Priority
4. **Performance optimization** - Reduce per-sample parameter updates
5. **Add GUI** - Implement VIZIA interface matching original layout
6. **Add presets** - Create factory presets

### Low Priority
7. **Add modulation** - Design modulation routing system
8. **Add oversampling** - Reduce aliasing from harsh quantization
9. **Add dithering** - Optional noise shaping for lower bit depths

## File Structure

```
modules/beverley/
├── Cargo.toml           # Dependencies and build config
├── README.md            # User documentation
├── IMPLEMENTATION.md    # This file
├── .gitignore          # Git ignore patterns
└── src/
    ├── lib.rs          # Plugin entry point and parameters
    └── dsp.rs          # DSP algorithms
```

## Build Instructions

```bash
# Install Rust if needed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build debug version
cd modules/beverley
cargo build

# Build optimized release version
cargo build --release

# Run tests
cargo test
```

The VST3 bundle will be in `target/bundled/Beverley.vst3/`
