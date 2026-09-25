//! Shared plugin state wrapping the audio engine.

use audio_engine::engine::AudioEngine;
use std::sync::Mutex;

/// Plugin-level state managed by Tauri's state system.
///
/// Uses `Mutex` (not in the audio thread — only in the IPC command handlers)
/// to guard the `AudioEngine` which manages its own RT-safe internal state.
pub struct AudioPluginState {
    pub engine: Mutex<AudioEngine>,
}

impl AudioPluginState {
    pub fn new() -> Self {
        Self {
            engine: Mutex::new(AudioEngine::new()),
        }
    }
}
