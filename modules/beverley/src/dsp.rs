// DSP components for Beverley bitcrusher
use std::f32::consts::E;

/// Simple bit crusher that quantizes input to a specified number of levels
/// with optional smoothing between quantization levels
pub struct BitCrusher {
    levels: i32,
    step: f32,
    step_reciprocal: f32,
    steepness: f32,
    power: f32,
}

impl BitCrusher {
    pub fn new() -> Self {
        let mut crusher = Self {
            levels: 1,
            step: 1.0,
            step_reciprocal: 1.0,
            steepness: 0.0,
            power: 1.0,
        };
        crusher.set_levels(15); // Default to 4-bit (16 levels - 1)
        crusher
    }

    pub fn set_levels(&mut self, levels: i32) {
        self.levels = levels;
        self.step = 1.0 / levels as f32;
        self.step_reciprocal = levels as f32;
    }

    pub fn set_steepness(&mut self, steepness: f32, power: f32) {
        self.steepness = steepness;
        self.power = power;
    }

    pub fn crush(&self, input: f32) -> f32 {
        let scaled = input * self.step_reciprocal;

        let low = scaled.floor() as i32;
        let high = scaled.ceil() as i32;

        let quantized_low = low as f32 * self.step;
        if low == high {
            return quantized_low;
        }

        let interval_fraction = (input - quantized_low) * self.step_reciprocal;
        let smoothed_fraction = self.sigmoidish(interval_fraction);

        quantized_low + (smoothed_fraction * self.step)
    }

    fn sigmoidish(&self, x: f32) -> f32 {
        if self.steepness == 0.0 {
            return x;
        }

        // Smoothstep function: 3x^2 - 2x^3
        let smooth = x * x * (3.0 - 2.0 * x);

        if self.steepness < 0.5 {
            // Blend between linear and smooth
            let blend = self.steepness * 2.0;
            return (1.0 - blend) * x + blend * smooth;
        }

        // Blend between smooth and hard step
        let step = if x >= 0.5 { 1.0 } else { 0.0 };
        let blend = (self.steepness - 0.5) * 2.0;
        (1.0 - blend) * smooth + blend * step
    }
}

/// Envelope follower for automatic gain compensation
pub struct ExponentialPeak {
    attack_coeff: f32,
    inverse_attack_coeff: f32,
    release_coeff: f32,
    peak_level: f32,
}

impl ExponentialPeak {
    const GATE_THRESHOLD: f32 = 0.005; // Below this, apply maximum gain
    const TARGET_LEVEL: f32 = 1.0;
    const GATE_THRESHOLD_RECIPROCAL: f32 = Self::TARGET_LEVEL / Self::GATE_THRESHOLD;

    pub fn new(sample_rate: f32) -> Self {
        let attack_coeff = Self::time_constant_to_coeff(0.5, sample_rate);
        let release_coeff = Self::time_constant_to_coeff(50.0, sample_rate);

        Self {
            attack_coeff,
            inverse_attack_coeff: 1.0 - attack_coeff,
            release_coeff,
            peak_level: 0.0,
        }
    }

    fn time_constant_to_coeff(time_constant_ms: f32, sample_rate: f32) -> f32 {
        let time_constant_samples = (time_constant_ms / 1000.0) * sample_rate;
        E.powf(-1.0 / time_constant_samples)
    }

    pub fn gain_compensation(&mut self, abs_input: f32) -> f32 {
        // Update peak level with attack/release envelope
        self.peak_level = if abs_input > self.peak_level {
            self.attack_coeff * self.peak_level + self.inverse_attack_coeff * abs_input
        } else {
            self.release_coeff * self.peak_level
        };

        // Calculate gain compensation
        if self.peak_level < Self::GATE_THRESHOLD {
            Self::GATE_THRESHOLD_RECIPROCAL
        } else {
            Self::TARGET_LEVEL / self.peak_level
        }
    }

    pub fn reset(&mut self) {
        self.peak_level = 0.0;
    }
}

/// Interpolated bit crusher with gamma correction and auto-gain
pub struct InterpolatedBitCrusher {
    low: BitCrusher,
    high: BitCrusher,
    fraction: f32,
    gamma: f32,
    gamma_reciprocal: f32,
    gain: f32,
    gain_reciprocal: f32,
    auto_gain: bool,
    symmetric_crush: bool,
    peak: ExponentialPeak,
}

impl InterpolatedBitCrusher {
    const THRESHOLD: f32 = 0.05;
    const THRESHOLD_RECIPROCAL: f32 = 1.0 / Self::THRESHOLD;

    pub fn new(sample_rate: f32) -> Self {
        Self {
            low: BitCrusher::new(),
            high: BitCrusher::new(),
            fraction: 0.0,
            gamma: 1.0,
            gamma_reciprocal: 1.0,
            gain: 1.0,
            gain_reciprocal: 1.0,
            auto_gain: false,
            symmetric_crush: false,
            peak: ExponentialPeak::new(sample_rate),
        }
    }

    pub fn set_depth(&mut self, bit_depth: f32) {
        let bit_floor = bit_depth.floor() as u32;
        let bit_ceil = bit_depth.ceil() as u32;
        self.fraction = bit_depth - bit_floor as f32;

        self.low.set_levels((1 << bit_floor) - 1);
        self.high.set_levels((1 << bit_ceil) - 1);
    }

    pub fn set_auto_gain(&mut self, auto_gain: bool) {
        self.auto_gain = auto_gain;
    }

    pub fn set_gain(&mut self, gain: f32, gain_reciprocal: f32) {
        self.gain = gain;
        self.gain_reciprocal = gain_reciprocal;
    }

    pub fn set_gamma(&mut self, gamma: f32, gamma_reciprocal: f32) {
        self.gamma = gamma;
        self.gamma_reciprocal = gamma_reciprocal;
    }

    pub fn set_steepness(&mut self, steepness: f32, power: f32) { 
        self.low.set_steepness(steepness, power);
        self.high.set_steepness(steepness, power);
    }

    pub fn set_crush_mode(&mut self, symmetric: bool) {
        self.symmetric_crush = symmetric;
    }

    pub fn apply(&mut self, input: f32) -> f32 {
        let abs_input = input.abs();
        let sign_input = input.signum();

        // Get gain (auto or fixed)
        let gain = if self.auto_gain {
            self.peak.gain_compensation(abs_input)
        } else {
            self.gain
        };

        // Apply gain and clamp to ±1
        let clamped = (gain * abs_input).min(1.0);

        // Forward gamma transform
        let gamma_transformed = clamped.powf(self.gamma_reciprocal);

        // Quantize
        let quantized = if self.symmetric_crush {
            self.symmetric_quantize(gamma_transformed)
        } else {
            self.asymmetric_quantize(sign_input, gamma_transformed)
        };

        // Inverse gamma transform and gain compensation
        if self.auto_gain {
            sign_input * quantized.powf(self.gamma) / gain
        } else {
            sign_input * self.gain_reciprocal * quantized.powf(self.gamma)
        }
    }

    fn asymmetric_quantize(&self, sign_input: f32, gamma_transformed: f32) -> f32 {
        // Map signed input to 0-1 range
        let crush_input = ((gamma_transformed * sign_input) + 1.0) * 0.5;

        let mut quantized = self.symmetric_quantize(crush_input);

        // Smooth out very small values to avoid artifacts
        if gamma_transformed < Self::THRESHOLD {
            let frac = gamma_transformed * Self::THRESHOLD_RECIPROCAL;
            quantized = quantized * frac + (crush_input * (1.0 - frac));
        }

        // Map back to signed range
        sign_input * ((quantized * 2.0) - 1.0)
    }

    fn symmetric_quantize(&self, gamma_transformed: f32) -> f32 {
        // Interpolate between two bit depths
        let crushed_high = self.high.crush(gamma_transformed);
        let crushed_low = self.low.crush(gamma_transformed);
        crushed_low + self.fraction * (crushed_high - crushed_low)
    }

    pub fn reset(&mut self) {
        self.peak.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_crusher_basic() {
        let mut crusher = BitCrusher::new();
        crusher.set_levels(3); // 2-bit (4 levels)
        crusher.set_steepness(1.0); // Hard quantization

        let output = crusher.crush(0.5);
        assert!((output - 0.333).abs() < 0.01 || (output - 0.666).abs() < 0.01);
    }

    #[test]
    fn test_interpolated_crusher() {
        let mut crusher = InterpolatedBitCrusher::new(48000.0);
        crusher.set_depth(4.0); // 4-bit
        crusher.set_gamma(1.0); // No gamma
        crusher.set_smoothing(0.0); // Hard quantization

        let output = crusher.apply(0.5);
        assert!(output > 0.0 && output < 1.0);
    }
}
