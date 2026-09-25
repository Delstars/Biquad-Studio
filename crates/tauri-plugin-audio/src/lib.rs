//! Tauri v2 plugin bridging the Biquad Studio audio engine with the webview frontend.
//!
//! This plugin exposes Tauri IPC commands for device enumeration, engine lifecycle,
//! parameter control, and AutoEQ profile management.

mod commands;
mod state;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

/// Initialize the audio plugin with default state.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("audio")
        .invoke_handler(tauri::generate_handler![
            commands::list_render_devices,
            commands::list_capture_devices,
            commands::get_engine_status,
            commands::start_engine,
            commands::stop_engine,
            commands::set_channel_volume,
            commands::set_channel_mute,
            commands::set_crossfade,
            commands::set_master_volume,
            commands::set_master_mute,
            commands::search_headphones,
            commands::apply_autoeq_profile,
            commands::update_eq_filter,
        ])
        .setup(|app, _api| {
            let plugin_state = state::AudioPluginState::new();
            app.manage(plugin_state);
            tracing::info!("Biquad Studio audio plugin initialized");
            Ok(())
        })
        .build()
}
