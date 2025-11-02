// Beverley - Advanced bitcrusher and harmonic distortion CLAP plugin
// Port of the Voltage Modular module to CLAP

use nih_plug::prelude::*;
use std::sync::Arc;

mod dsp;
use dsp::InterpolatedBitCrusher;

pub struct Beverley {
    params: Arc<BeverleyParams>,
    crusher_left: InterpolatedBitCrusher,
    crusher_right: InterpolatedBitCrusher,

    // Cached parameter values to avoid redundant updates
    cached_bit_depth: f32,
    cached_gamma: f32,
    cached_smoothing: f32,
    cached_gain_db: f32,
    cached_auto_gain: bool,
    cached_crush_mode: bool,
}

#[derive(Params)]
pub struct BeverleyParams {
    /// Bit depth (1-16 bits, with fractional interpolation)
    #[id = "bit_depth"]
    pub bit_depth: FloatParam,

    /// Gamma correction: redistributes amplitude levels before quantization
    /// -1.0 = compress dynamics (gamma 0.1), 0.0 = neutral (gamma 1.0), +1.0 = expand dynamics (gamma 10.0)
    #[id = "gamma"]
    pub gamma: FloatParam,

    /// Smoothing amount (0 = hard quantization, 1 = transparent)
    #[id = "smoothing"]
    pub smoothing: FloatParam,

    /// Fixed gain in dB (0-20 dB)
    #[id = "gain"]
    pub gain_db: FloatParam,

    /// Auto-gain toggle (envelope-following gain compensation)
    #[id = "auto_gain"]
    pub auto_gain: BoolParam,

    /// Crush mode (false = asymmetric, true = symmetric)
    #[id = "crush_mode"]
    pub crush_mode: BoolParam,
}

impl Default for Beverley {
    fn default() -> Self {
        Self {
            params: Arc::new(BeverleyParams::default()),
            crusher_left: InterpolatedBitCrusher::new(48000.0),
            crusher_right: InterpolatedBitCrusher::new(48000.0),
            cached_bit_depth: 4.0,
            cached_gamma: 1.0, // 10^0 = 1.0
            cached_smoothing: 0.0,
            cached_gain_db: 0.0,
            cached_auto_gain: false,
            cached_crush_mode: false,
        }
    }
}

impl Beverley {
    const EPSILON: f32 = 1e-6;

    fn update_crushers(&mut self) {
        let bit_depth = self.params.bit_depth.value();
        let gamma = self.params.gamma.value();
        let smoothing = self.params.smoothing.value();
        let gain_db: f32 = self.params.gain_db.value();
        let auto_gain = self.params.auto_gain.value();
        let crush_mode = self.params.crush_mode.value();

        self.update_crushers_if_changed(
            bit_depth,
            gamma,
            smoothing,
            gain_db,
            auto_gain,
            crush_mode
        );
    }

    #[inline]
    fn update_crushers_if_changed(
        &mut self,
        bit_depth: f32,
        gamma: f32,
        smoothing: f32,
        gain_db: f32,
        auto_gain: bool,
        crush_mode: bool,
    ) {
        let bit_depth_changed = (bit_depth - self.cached_bit_depth).abs() > Self::EPSILON;
        let gamma_changed = (gamma - self.cached_gamma).abs() > Self::EPSILON;
        let smoothing_changed = (smoothing - self.cached_smoothing).abs() > Self::EPSILON;
        let gain_db_changed = (gain_db - self.cached_gain_db).abs() > Self::EPSILON;
        let auto_gain_changed = auto_gain != self.cached_auto_gain;
        let crush_mode_changed = crush_mode != self.cached_crush_mode;

        if bit_depth_changed {
            self.crusher_left.set_depth(bit_depth);
            self.crusher_right.set_depth(bit_depth);
            self.cached_bit_depth = bit_depth;
        }

        if gamma_changed {
            let scaled_gamma: f32 = 10_f32.powf(gamma);
            let scaled_gamma_reciprocal = 1.0 / scaled_gamma; 
            self.crusher_left.set_gamma(scaled_gamma, scaled_gamma_reciprocal);
            self.crusher_right.set_gamma(scaled_gamma, scaled_gamma_reciprocal);
            self.cached_gamma = gamma;
        }

        if smoothing_changed {
            let steepness = 1.0 - smoothing;
            let power = 1.0 / (1.0 - steepness + 0.01);
            self.crusher_left.set_steepness(steepness, power);
            self.crusher_right.set_steepness(steepness, power);
            self.cached_smoothing = smoothing;
        }

        if gain_db_changed {
            let gain: f32 =  10_f32.powf(gain_db * 0.05);
            let gain_reciprocal: f32 = 1.0 / gain;
            self.crusher_left.set_gain(gain, gain_reciprocal);
            self.crusher_right.set_gain(gain, gain_reciprocal);
            self.cached_gain_db = gain_db;
        }

        if auto_gain_changed {
            self.crusher_left.set_auto_gain(auto_gain);
            self.crusher_right.set_auto_gain(auto_gain);
            self.cached_auto_gain = auto_gain;
        }

        if crush_mode_changed {
            self.crusher_left.set_crush_mode(crush_mode);
            self.crusher_right.set_crush_mode(crush_mode);
            self.cached_crush_mode = crush_mode;
        }
    }
}

impl Default for BeverleyParams {
    fn default() -> Self {
        Self {
            bit_depth: FloatParam::new(
                "Bit Depth",
                4.0,
                FloatRange::Linear { min: 1.0, max: 16.0 },
            )
            .with_step_size(0.01)
            .with_value_to_string(formatters::v2s_f32_rounded(2)),

            gamma: FloatParam::new(
                "Gamma",
                0.0,
                FloatRange::Linear { min: -1.0, max: 1.0 },
            )
            .with_step_size(0.01)
            .with_value_to_string(formatters::v2s_f32_rounded(2)),

            smoothing: FloatParam::new(
                "Smoothing",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_step_size(0.01)
            .with_value_to_string(formatters::v2s_f32_percentage(0)),

            gain_db: FloatParam::new(
                "Gain",
                0.0,
                FloatRange::Linear { min: 0.0, max: 20.0 },
            )
            .with_step_size(0.1)
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_rounded(1)),

            auto_gain: BoolParam::new("Auto Gain", false),

            crush_mode: BoolParam::new("Symmetric", false),
        }
    }
}

impl Plugin for Beverley {
    const NAME: &'static str = "Beverley";
    const VENDOR: &'static str = "Vulpus Labs";
    const URL: &'static str = "https://vulpuslabs.com";
    const EMAIL: &'static str = "info@vulpuslabs.com";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
    ];

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> std::sync::Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Reinitialize crushers with correct sample rate
        let sample_rate = buffer_config.sample_rate;
        self.crusher_left = InterpolatedBitCrusher::new(sample_rate);
        self.crusher_right = InterpolatedBitCrusher::new(sample_rate);

        // Set initial parameters
        self.update_crushers();

        true
    }

    fn reset(&mut self) {
        self.crusher_left.reset();
        self.crusher_right.reset();
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let num_samples = buffer.samples();
        let channels = buffer.as_slice();

        if channels.len() == 1 {
            // Mono input - process left channel only
            let left_channel = &mut channels[0];
            for sample_idx in 0..num_samples {
                self.update_crushers();
                let sample = &mut left_channel[sample_idx];
                *sample = self.crusher_left.apply(*sample);
            }
        } else {
            // Stereo - process both channels
            let (left_channel, rest) = channels.split_first_mut().unwrap();
            let right_channel = &mut rest[0];

            for sample_idx in 0..num_samples {
                self.update_crushers();

                let left_sample = &mut left_channel[sample_idx];
                *left_sample = self.crusher_left.apply(*left_sample);

                let right_sample = &mut right_channel[sample_idx];
                *right_sample = self.crusher_right.apply(*right_sample);
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for Beverley {
    const CLAP_ID: &'static str = "com.vulpuslabs.beverley";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Advanced bitcrusher and harmonic distortion");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Distortion,
    ];
}

nih_export_clap!(Beverley);

// Export VST3 wrapper for the CLAP plugin
clap_wrapper::export_vst3!();
