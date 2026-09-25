use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct MeterFrame {
    pub peaks: [f32; 4],
    pub rms: [f32; 4],
    pub sample_rate: u32,
    pub buffer_frames: u32,
}

impl MeterFrame {
    pub fn silent() -> Self {
        Self {
            peaks: [0.0; 4],
            rms: [0.0; 4],
            sample_rate: 48000,
            buffer_frames: 0,
        }
    }
}

pub fn compute_meters(buffer: &[f32], channels: usize) -> (Vec<f32>, Vec<f32>) {
    let mut peaks = vec![0.0; channels];
    let mut sum_sq = vec![0.0; channels];

    let frames = buffer.len() / channels;
    if frames == 0 {
        return (peaks, sum_sq);
    }

    for (i, &sample) in buffer.iter().enumerate() {
        let ch = i % channels;
        let abs_sample = sample.abs();
        if abs_sample > peaks[ch] {
            peaks[ch] = abs_sample;
        }
        sum_sq[ch] += sample * sample;
    }

    let rms = sum_sq
        .into_iter()
        .map(|s| (s / frames as f32).sqrt())
        .collect();
    (peaks, rms)
}
