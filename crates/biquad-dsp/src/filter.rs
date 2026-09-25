use crate::coefficients::BiquadCoefficients;
use crate::limiter::db_to_linear;

/// Transposed Direct Form II biquad filter state
#[derive(Debug, Clone, Copy, Default)]
pub struct BiquadState {
    pub s1: f32,
    pub s2: f32,
}

impl BiquadState {
    #[inline(always)]
    pub fn process_sample(&mut self, sample: f32, coeffs: &BiquadCoefficients) -> f32 {
        let y = coeffs.b0 * sample + self.s1;
        self.s1 = self.s2 + coeffs.b1 * sample - coeffs.a1 * y;
        self.s2 = coeffs.b2 * sample - coeffs.a2 * y;
        y
    }
}

use crate::coefficients::SmoothedCoefficients;

/// MultiBandEqualizer holding state and coefficients
pub struct MultiBandEqualizer {
    pub num_channels: usize,
    pub filters: Vec<SmoothedCoefficients>,
    pub states: Vec<Vec<BiquadState>>,
    pub preamp_linear: f32,
}

impl MultiBandEqualizer {
    pub fn new(num_channels: usize, num_filters: usize, preamp_db: f32, sample_rate: f32) -> Self {
        let default_coeffs = BiquadCoefficients::identity();
        let filters = vec![SmoothedCoefficients::new(default_coeffs, sample_rate, 30.0); num_filters];
        let states = vec![vec![BiquadState::default(); num_filters]; num_channels];
        Self {
            num_channels,
            filters,
            states,
            preamp_linear: db_to_linear(preamp_db),
        }
    }

    pub fn update_filter(&mut self, index: usize, coeffs: BiquadCoefficients) {
        if index < self.filters.len() {
            self.filters[index].set_target(coeffs);
        }
    }

    pub fn set_preamp(&mut self, preamp_db: f32) {
        self.preamp_linear = db_to_linear(preamp_db);
    }

    #[inline(always)]
    pub fn process_interleaved(&mut self, buffer: &mut [f32], channels: usize) {
        let preamp = self.preamp_linear;
        
        // We only tick the smoothing once per frame, not per channel sample
        for frame_idx in 0..(buffer.len() / channels) {
            // Tick filters once per frame to avoid updating coefficients mid-frame
            // This is actually safer, but for simplicity, we can tick them per frame
            
            for f_idx in 0..self.filters.len() {
                self.filters[f_idx].tick();
            }
            
            for ch in 0..channels {
                if ch >= self.num_channels { continue; }
                let sample_idx = frame_idx * channels + ch;
                
                let mut y = buffer[sample_idx] * preamp;
                for f_idx in 0..self.filters.len() {
                    let coeffs = &self.filters[f_idx].current;
                    y = self.states[ch][f_idx].process_sample(y, coeffs);
                }
                buffer[sample_idx] = y;
            }
        }
    }

    pub fn reset(&mut self) {
        for ch_states in &mut self.states {
            for state in ch_states.iter_mut() {
                *state = BiquadState::default();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn test_identity_filter() {
        let mut state = BiquadState::default();
        let coeffs = BiquadCoefficients::identity();
        
        let out = state.process_sample(0.5, &coeffs);
        assert_relative_eq!(out, 0.5);
    }

    #[test]
    fn test_process_silence() {
        let mut eq = MultiBandEqualizer::new(2, 4, 0.0, 48000.0);
        let mut buffer = vec![0.0; 10000];
        eq.process_interleaved(&mut buffer, 2);
        assert!(buffer.iter().all(|&s| s == 0.0 && !s.is_nan() && !s.is_infinite()));
    }
}
