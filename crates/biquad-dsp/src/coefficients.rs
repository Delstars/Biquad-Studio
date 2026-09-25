use std::f32::consts::PI;

/// Robert Bristow-Johnson (RBJ) Audio-EQ-Cookbook biquad coefficients
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct BiquadCoefficients {
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
    pub a1: f32,
    pub a2: f32,
}

impl BiquadCoefficients {
    /// Unity passthrough (b0=1, rest=0)
    pub fn identity() -> Self {
        Self { b0: 1.0, b1: 0.0, b2: 0.0, a1: 0.0, a2: 0.0 }
    }

    /// Bell/PK filter
    pub fn peaking(sample_rate: f32, freq_hz: f32, gain_db: f32, q: f32) -> Self {
        let w0 = 2.0 * PI * (freq_hz / sample_rate);
        let alpha = w0.sin() / (2.0 * q);
        let a = 10.0_f32.powf(gain_db / 40.0);
        let cos_w0 = w0.cos();

        let b0 = 1.0 + alpha * a;
        let b1 = -2.0 * cos_w0;
        let b2 = 1.0 - alpha * a;
        let a0 = 1.0 + alpha / a;
        let a1 = -2.0 * cos_w0;
        let a2 = 1.0 - alpha / a;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
        }
    }

    /// Low shelf filter
    pub fn low_shelf(sample_rate: f32, freq_hz: f32, gain_db: f32, q: f32) -> Self {
        let w0 = 2.0 * PI * (freq_hz / sample_rate);
        let alpha = w0.sin() / (2.0 * q);
        let a = 10.0_f32.powf(gain_db / 40.0);
        let cos_w0 = w0.cos();
        let sqrt_a_alpha = 2.0 * a.sqrt() * alpha;

        let b0 = a * ((a + 1.0) - (a - 1.0) * cos_w0 + sqrt_a_alpha);
        let b1 = 2.0 * a * ((a - 1.0) - (a + 1.0) * cos_w0);
        let b2 = a * ((a + 1.0) - (a - 1.0) * cos_w0 - sqrt_a_alpha);
        
        let a0 = (a + 1.0) + (a - 1.0) * cos_w0 + sqrt_a_alpha;
        let a1 = -2.0 * ((a - 1.0) + (a + 1.0) * cos_w0);
        let a2 = (a + 1.0) + (a - 1.0) * cos_w0 - sqrt_a_alpha;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
        }
    }

    /// High shelf filter
    pub fn high_shelf(sample_rate: f32, freq_hz: f32, gain_db: f32, q: f32) -> Self {
        let w0 = 2.0 * PI * (freq_hz / sample_rate);
        let alpha = w0.sin() / (2.0 * q);
        let a = 10.0_f32.powf(gain_db / 40.0);
        let cos_w0 = w0.cos();
        let sqrt_a_alpha = 2.0 * a.sqrt() * alpha;

        let b0 = a * ((a + 1.0) + (a - 1.0) * cos_w0 + sqrt_a_alpha);
        let b1 = -2.0 * a * ((a - 1.0) + (a + 1.0) * cos_w0);
        let b2 = a * ((a + 1.0) + (a - 1.0) * cos_w0 - sqrt_a_alpha);
        
        let a0 = (a + 1.0) - (a - 1.0) * cos_w0 + sqrt_a_alpha;
        let a1 = 2.0 * ((a - 1.0) - (a + 1.0) * cos_w0);
        let a2 = (a + 1.0) - (a - 1.0) * cos_w0 - sqrt_a_alpha;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
        }
    }

    /// Low pass filter
    pub fn low_pass(sample_rate: f32, freq_hz: f32, q: f32) -> Self {
        let w0 = 2.0 * PI * (freq_hz / sample_rate);
        let alpha = w0.sin() / (2.0 * q);
        let cos_w0 = w0.cos();

        let b0 = (1.0 - cos_w0) / 2.0;
        let b1 = 1.0 - cos_w0;
        let b2 = (1.0 - cos_w0) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_w0;
        let a2 = 1.0 - alpha;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
        }
    }

    /// High pass filter
    pub fn high_pass(sample_rate: f32, freq_hz: f32, q: f32) -> Self {
        let w0 = 2.0 * PI * (freq_hz / sample_rate);
        let alpha = w0.sin() / (2.0 * q);
        let cos_w0 = w0.cos();

        let b0 = (1.0 + cos_w0) / 2.0;
        let b1 = -(1.0 + cos_w0);
        let b2 = (1.0 + cos_w0) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_w0;
        let a2 = 1.0 - alpha;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
        }
    }
}

/// Coefficient smoothing to prevent audio artifacts (zipper noise) when parameters change.
#[derive(Debug, Clone, PartialEq)]
pub struct SmoothedCoefficients {
    pub current: BiquadCoefficients,
    pub target: BiquadCoefficients,
    smoothing_factor: f32,
}

impl SmoothedCoefficients {
    /// Create a new smoothed coefficient manager.
    /// `smoothing_time_ms` is the time it takes to reach ~63% of the target.
    /// For EQ dragging, 20-50ms is usually good.
    pub fn new(initial: BiquadCoefficients, sample_rate: f32, smoothing_time_ms: f32) -> Self {
        // time constant lambda
        // lambda = 1.0 - exp(-1.0 / (tau * fs))
        // or a simpler approximation:
        let tau_samples = (smoothing_time_ms / 1000.0) * sample_rate;
        let smoothing_factor = if tau_samples > 0.0 { 1.0 / tau_samples } else { 1.0 };
        
        Self {
            current: initial,
            target: initial,
            smoothing_factor: smoothing_factor.clamp(0.0, 1.0),
        }
    }

    /// Set a new target to smooth towards.
    pub fn set_target(&mut self, target: BiquadCoefficients) {
        self.target = target;
    }

    /// Snap immediately to target without smoothing.
    pub fn snap_to_target(&mut self, target: BiquadCoefficients) {
        self.target = target;
        self.current = target;
    }

    /// Advance the smoothing by one sample and return the current coefficients.
    #[inline(always)]
    pub fn tick(&mut self) -> &BiquadCoefficients {
        // Fast path: if we are very close to target, just snap and return
        // (This saves branches/math in the steady state, though the smoothing math itself is fast)
        // For simplicity and constant execution time, we can just always do the math.
        
        self.current.b0 += self.smoothing_factor * (self.target.b0 - self.current.b0);
        self.current.b1 += self.smoothing_factor * (self.target.b1 - self.current.b1);
        self.current.b2 += self.smoothing_factor * (self.target.b2 - self.current.b2);
        self.current.a1 += self.smoothing_factor * (self.target.a1 - self.current.a1);
        self.current.a2 += self.smoothing_factor * (self.target.a2 - self.current.a2);
        
        &self.current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let c = BiquadCoefficients::identity();
        assert_eq!(c.b0, 1.0);
        assert_eq!(c.b1, 0.0);
        assert_eq!(c.b2, 0.0);
        assert_eq!(c.a1, 0.0);
        assert_eq!(c.a2, 0.0);
    }
    
    #[test]
    fn test_peaking_filter() {
        let fs = 44100.0;
        let fc = 1000.0;
        let gain = 6.0;
        let q = 0.707;
        
        let c = BiquadCoefficients::peaking(fs, fc, gain, q);
        // We just ensure it generates valid numbers (no NaNs)
        assert!(!c.b0.is_nan());
        assert!(!c.b1.is_nan());
        assert!(!c.a1.is_nan());
    }
}
