use std::f32::consts::PI;

/// Calculate crossfade gains for an equal-power crossfade.
/// position 0.0 = full A, 0.5 = equal, 1.0 = full B.
pub fn crossfade_gains(position: f32) -> (f32, f32) {
    let clamped = position.clamp(0.0, 1.0);
    let angle = clamped * (PI / 2.0);
    (angle.cos(), angle.sin())
}

/// A struct to apply crossfading to interleaved buffers
pub struct Crossfader {
    pub position: f32,
}

impl Crossfader {
    pub fn new(position: f32) -> Self {
        Self { position: position.clamp(0.0, 1.0) }
    }

    pub fn set_position(&mut self, position: f32) {
        self.position = position.clamp(0.0, 1.0);
    }

    /// Process in-place crossfade from buffer_b into buffer_a.
    /// buffer_a will contain the crossfaded output.
    #[inline(always)]
    pub fn process_interleaved(&self, buffer_a: &mut [f32], buffer_b: &[f32]) {
        let (gain_a, gain_b) = crossfade_gains(self.position);
        let len = buffer_a.len().min(buffer_b.len());
        
        for i in 0..len {
            buffer_a[i] = buffer_a[i] * gain_a + buffer_b[i] * gain_b;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_crossfade_gains() {
        let (a, b) = crossfade_gains(0.0);
        assert_relative_eq!(a, 1.0);
        assert_relative_eq!(b, 0.0);

        let (a, b) = crossfade_gains(1.0);
        assert_relative_eq!(a, 0.0, epsilon = 1e-6);
        assert_relative_eq!(b, 1.0);

        let (a, b) = crossfade_gains(0.5);
        assert_relative_eq!(a, (0.5_f32).sqrt());
        assert_relative_eq!(b, (0.5_f32).sqrt());
    }
}
