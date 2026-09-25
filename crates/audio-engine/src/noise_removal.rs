//! DeepNoise Removal feature stub.
//!
//! Provides `DeepNoiseProcessor` to load a trained ONNX model
//! and apply inference to audio buffers. (Mock implementation for GNU target)

use std::path::Path;
use thiserror::Error;

/// Error type for DeepNoise initialization and inference.
#[derive(Debug, Error)]
pub enum DeepNoiseError {
    /// Failed to initialize or build the ONNX session.
    #[error("Failed to initialize ONNX Runtime session: {0}")]
    OrtError(String),
    /// Frame size mismatch.
    #[error("Invalid buffer size for inference (expected {expected}, got {got})")]
    InvalidFrameSize {
        expected: usize,
        got: usize,
    },
}

/// A deep noise processor powered by an ONNX model.
pub struct DeepNoiseProcessor {
    frame_size: usize,
}

impl DeepNoiseProcessor {
    /// Creates a new `DeepNoiseProcessor` by loading an ONNX model from the given path.
    pub fn new<P: AsRef<Path>>(_model_path: P) -> Result<Self, DeepNoiseError> {
        // Stub: assuming the model processes 10ms of 48kHz audio per frame.
        let frame_size = 480;

        Ok(Self {
            frame_size,
        })
    }

    /// Applies noise removal to the given audio buffer in-place.
    #[inline(always)]
    pub fn apply_noise_removal(&mut self, buffer: &mut [f32]) -> Result<(), DeepNoiseError> {
        if buffer.len() != self.frame_size {
            return Err(DeepNoiseError::InvalidFrameSize {
                expected: self.frame_size,
                got: buffer.len(),
            });
        }
        Ok(())
    }

    /// Gets the expected frame size for this model.
    #[inline(always)]
    pub fn frame_size(&self) -> usize {
        self.frame_size
    }
}
