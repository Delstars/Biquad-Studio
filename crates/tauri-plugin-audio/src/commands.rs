//! Tauri IPC command handlers for the audio plugin.
//!
//! These functions are called from the React frontend via `invoke("plugin:audio|command_name")`.
//! They run on Tauri's async runtime (Tokio), NOT on the audio thread.

use crate::state::AudioPluginState;
use audio_engine::device::AudioDeviceInfo;
use audio_engine::params;
use serde::{Deserialize, Serialize};
use std::sync::atomic::Ordering;
use tauri::State;

/// Engine status reported to the frontend.
#[derive(Debug, Serialize)]
pub struct EngineStatus {
    pub running: bool,
    pub sample_rate: u32,
    pub buffer_frames: u32,
    pub output_device: Option<String>,
}

/// Parameters for updating a single EQ filter band.
#[derive(Debug, Deserialize)]
pub struct EqFilterUpdate {
    pub band_index: usize,
    pub frequency_hz: f32,
    pub gain_db: f32,
    pub q: f32,
    pub filter_type: String, // "peaking", "low_shelf", "high_shelf", "low_pass", "high_pass"
}

// ─── Device Enumeration ───────────────────────────────────────────

#[tauri::command]
pub fn list_render_devices() -> Result<Vec<AudioDeviceInfo>, String> {
    audio_engine::device::enumerate_render_devices().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_capture_devices() -> Result<Vec<AudioDeviceInfo>, String> {
    audio_engine::device::enumerate_capture_devices().map_err(|e| e.to_string())
}

// ─── Engine Lifecycle ─────────────────────────────────────────────

#[tauri::command]
pub fn get_engine_status(state: State<'_, AudioPluginState>) -> Result<EngineStatus, String> {
    let engine = state.engine.lock().map_err(|e| e.to_string())?;
    Ok(EngineStatus {
        running: engine.is_running(),
        sample_rate: 48000, // TODO: query from engine
        buffer_frames: 128, // TODO: query from engine
        output_device: None, // TODO: track selected device
    })
}

#[tauri::command]
pub fn start_engine(
    device_id: Option<String>,
    state: State<'_, AudioPluginState>,
) -> Result<(), String> {
    let mut engine = state.engine.lock().map_err(|e| e.to_string())?;
    engine
        .start(device_id.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn stop_engine(state: State<'_, AudioPluginState>) -> Result<(), String> {
    let mut engine = state.engine.lock().map_err(|e| e.to_string())?;
    engine.stop().map_err(|e| e.to_string())
}

// ─── Parameter Controls ───────────────────────────────────────────

#[tauri::command]
pub fn set_channel_volume(
    channel: String,
    volume: f32,
    state: State<'_, AudioPluginState>,
) -> Result<(), String> {
    let engine = state.engine.lock().map_err(|e| e.to_string())?;
    let channel_index = channel_name_to_index(&channel)?;
    engine
        .params()
        .channel_volumes[channel_index]
        .store(volume.clamp(0.0, 1.0), Ordering::Relaxed);
    Ok(())
}

#[tauri::command]
pub fn set_channel_mute(
    channel: String,
    muted: bool,
    state: State<'_, AudioPluginState>,
) -> Result<(), String> {
    let engine = state.engine.lock().map_err(|e| e.to_string())?;
    let channel_index = channel_name_to_index(&channel)?;
    engine.params().channel_mutes[channel_index].store(muted, Ordering::Relaxed);
    Ok(())
}

#[tauri::command]
pub fn set_crossfade(
    position: f32,
    state: State<'_, AudioPluginState>,
) -> Result<(), String> {
    let engine = state.engine.lock().map_err(|e| e.to_string())?;
    engine
        .params()
        .crossfade_position
        .store(position.clamp(0.0, 1.0), Ordering::Relaxed);
    Ok(())
}

#[tauri::command]
pub fn set_master_volume(
    volume: f32,
    state: State<'_, AudioPluginState>,
) -> Result<(), String> {
    let engine = state.engine.lock().map_err(|e| e.to_string())?;
    engine
        .params()
        .master_volume
        .store(volume.clamp(0.0, 1.0), Ordering::Relaxed);
    Ok(())
}

#[tauri::command]
pub fn set_master_mute(
    muted: bool,
    state: State<'_, AudioPluginState>,
) -> Result<(), String> {
    let engine = state.engine.lock().map_err(|e| e.to_string())?;
    engine
        .params()
        .master_mute
        .store(muted, Ordering::Relaxed);
    Ok(())
}

// ─── AutoEQ / Headphone Calibration ──────────────────────────────

#[tauri::command]
pub fn search_headphones(query: String) -> Result<Vec<String>, String> {
    // Phase 1 stub — will integrate with autoeq-parser database in Phase 2
    let _ = query;
    Ok(vec![
        "Sennheiser HD 600".to_string(),
        "Beyerdynamic DT 990 Pro".to_string(),
        "AKG K712 Pro".to_string(),
    ])
}

#[tauri::command]
pub fn apply_autoeq_profile(
    headphone_name: String,
    _state: State<'_, AudioPluginState>,
) -> Result<(), String> {
    // Phase 2 stub — will load from bundled database and apply to Stage 1 DSP
    tracing::info!("AutoEQ profile requested for: {}", headphone_name);
    Ok(())
}

#[tauri::command]
pub fn update_eq_filter(
    params: EqFilterUpdate,
    _state: State<'_, AudioPluginState>,
) -> Result<(), String> {
    // Phase 2 stub — will compute BiquadCoefficients and push to atomic store
    tracing::info!(
        "EQ filter update: band={}, freq={}, gain={}, Q={}, type={}",
        params.band_index,
        params.frequency_hz,
        params.gain_db,
        params.q,
        params.filter_type
    );
    Ok(())
}

// ─── Helpers ──────────────────────────────────────────────────────

/// Map a channel name string to its index constant.
fn channel_name_to_index(name: &str) -> Result<usize, String> {
    match name.to_lowercase().as_str() {
        "game" => Ok(params::CHANNEL_GAME),
        "chat" => Ok(params::CHANNEL_CHAT),
        "media" => Ok(params::CHANNEL_MEDIA),
        "mic" => Ok(params::CHANNEL_MIC),
        _ => Err(format!("Unknown channel: {name}")),
    }
}
