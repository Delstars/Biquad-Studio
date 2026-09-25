/// Convert decibels to linear scale
#[inline(always)]
pub fn db_to_linear(db: f32) -> f32 {
    10.0_f32.powf(db / 20.0)
}

/// Convert linear scale to decibels
#[inline(always)]
pub fn linear_to_db(linear: f32) -> f32 {
    if linear <= 1e-5 {
        -100.0
    } else {
        20.0 * linear.log10()
    }
}

/// Soft clip a single sample
#[inline(always)]
pub fn soft_clip(sample: f32, threshold_linear: f32) -> f32 {
    if sample.abs() <= threshold_linear {
        sample
    } else {
        let sign = sample.signum();
        let excess = sample.abs() - threshold_linear;
        sign * (threshold_linear + excess.tanh())
    }
}

/// A limiter to softly saturate signals
pub struct Limiter {
    pub threshold_linear: f32,
}

impl Limiter {
    pub fn new(threshold_db: f32) -> Self {
        Self {
            threshold_linear: db_to_linear(threshold_db),
        }
    }

    pub fn set_threshold(&mut self, threshold_db: f32) {
        self.threshold_linear = db_to_linear(threshold_db);
    }

    #[inline(always)]
    pub fn process_buffer_interleaved(&self, buffer: &mut [f32]) {
        for sample in buffer.iter_mut() {
            *sample = soft_clip(*sample, self.threshold_linear);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_db_linear_roundtrip() {
        let dbs = [-60.0, -12.0, 0.0, 6.0];
        for &db in &dbs {
            let lin = db_to_linear(db);
            let out = linear_to_db(lin);
            assert_relative_eq!(db, out, epsilon = 1e-4);
        }
    }

    #[test]
    fn test_soft_clip() {
        let th = 0.5;
        assert_eq!(soft_clip(0.1, th), 0.1);
        assert_eq!(soft_clip(0.5, th), 0.5);
        
        let clipped = soft_clip(2.0, th);
        assert!(clipped > 0.5 && clipped < 1.5); // tanh(1.5) + 0.5 ~ 1.405
    }
}
