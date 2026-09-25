fn main() {
    let commands = &[
        "list_render_devices",
        "list_capture_devices",
        "get_engine_status",
        "start_engine",
        "stop_engine",
        "set_channel_volume",
        "set_channel_mute",
        "set_crossfade",
        "set_master_volume",
        "set_master_mute",
        "search_headphones",
        "apply_autoeq_profile",
        "update_eq_filter"
    ];
    tauri_plugin::Builder::new(commands).build();
}
