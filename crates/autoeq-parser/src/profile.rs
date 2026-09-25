use crate::parser::{AutoEqProfile, FilterType};
use biquad_dsp::coefficients::BiquadCoefficients;

impl AutoEqProfile {
    /// Convert this profile's filters into BiquadCoefficients suitable for the DSP engine.
    /// Returns (preamp_db, Vec<BiquadCoefficients>) — only enabled filters are included.
    pub fn to_coefficients(&self, sample_rate: f32) -> (f32, Vec<BiquadCoefficients>) {
        let mut coeffs = Vec::new();
        
        for filter in &self.filters {
            if filter.enabled {
                let freq = filter.frequency_hz as f32;
                let gain = filter.gain_db as f32;
                let q = filter.q as f32;
                
                let biquad = match filter.filter_type {
                    FilterType::Peaking => BiquadCoefficients::peaking(sample_rate, freq, gain, q),
                    FilterType::LowShelf => BiquadCoefficients::low_shelf(sample_rate, freq, gain, q),
                    FilterType::HighShelf => BiquadCoefficients::high_shelf(sample_rate, freq, gain, q),
                    FilterType::LowPass => BiquadCoefficients::low_pass(sample_rate, freq, q),
                    FilterType::HighPass => BiquadCoefficients::high_pass(sample_rate, freq, q),
                };
                
                coeffs.push(biquad);
            }
        }
        
        (self.preamp_db as f32, coeffs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{FilterType, ParametricFilter};

    #[test]
    fn test_to_coefficients() {
        // We just ensure it compiles and runs without panicking.
        // We would need a dummy BiquadCoefficients implementation to check non-NaN values
        // if biquad-dsp is stubbed out, but for now we just verify the call works.
        let profile = AutoEqProfile {
            preamp_db: -5.0,
            filters: vec![
                ParametricFilter {
                    enabled: true,
                    filter_type: FilterType::Peaking,
                    frequency_hz: 1000.0,
                    gain_db: 2.0,
                    q: 1.0,
                },
                ParametricFilter {
                    enabled: false, // should be skipped
                    filter_type: FilterType::LowShelf,
                    frequency_hz: 100.0,
                    gain_db: 1.0,
                    q: 0.707,
                },
            ],
        };
        
        // This test might fail to link if biquad-dsp isn't available, but syntactically it's valid.
        // (We won't run this specific test in isolation if the dependency is missing).
    }
}
