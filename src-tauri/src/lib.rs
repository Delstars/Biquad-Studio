//! Biquad Studio — Tauri application setup.
//!
//! Registers plugins (audio engine, global shortcuts, autostart),
//! configures the system tray, and initializes structured logging.

use tauri::{Manager, Emitter};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

/// Main application entry point called from `main.rs`.
pub fn run() {
    // Initialize structured logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "biquad_studio=info,tauri_plugin_audio=info,audio_engine=info".into()),
        )
        .init();

    tracing::info!("Starting Biquad Studio v{}", env!("CARGO_PKG_VERSION"));

    let mute_shortcut = "Ctrl+Alt+M".parse::<Shortcut>().unwrap();
    let vol_up_shortcut = "Ctrl+Alt+Up".parse::<Shortcut>().unwrap();
    let vol_down_shortcut = "Ctrl+Alt+Down".parse::<Shortcut>().unwrap();

    tauri::Builder::default()
        // Official plugins
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, shortcut, _event| {
                    if shortcut.id() == mute_shortcut.id() {
                        tracing::info!("Global Hotkey: Mute Toggle");
                        let _ = app.emit("hotkey-mute-toggle", ());
                    } else if shortcut.id() == vol_up_shortcut.id() {
                        let _ = app.emit("hotkey-volume-up", ());
                    } else if shortcut.id() == vol_down_shortcut.id() {
                        let _ = app.emit("hotkey-volume-down", ());
                    }
                })
                .build(),
        )
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        // Custom audio engine plugin
        .plugin(tauri_plugin_audio::init())
        // Application setup
        .setup(|app| {
            // Register global shortcuts
            if let Ok(shortcut_mute) = "Ctrl+Alt+M".parse::<Shortcut>() {
                let _ = app.global_shortcut().register(shortcut_mute);
            }
            if let Ok(shortcut_vol_up) = "Ctrl+Alt+Up".parse::<Shortcut>() {
                let _ = app.global_shortcut().register(shortcut_vol_up);
            }
            if let Ok(shortcut_vol_down) = "Ctrl+Alt+Down".parse::<Shortcut>() {
                let _ = app.global_shortcut().register(shortcut_vol_down);
            }

            setup_system_tray(app)?;
            tracing::info!("Biquad Studio initialized successfully");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error running Biquad Studio");
}

/// Configure the system tray icon and menu.
fn setup_system_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    use tauri::menu::{MenuBuilder, MenuItem};
    use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};

    let toggle_mute = MenuItem::with_id(app, "toggle_mute", "Toggle Mute", true, None::<&str>)?;
    let show_window = MenuItem::with_id(app, "show_window", "Show Window", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit Biquad Studio", true, None::<&str>)?;

    let menu = MenuBuilder::new(app)
        .items(&[&toggle_mute, &show_window, &quit])
        .build()?;

    TrayIconBuilder::with_id("main-tray")
        .tooltip("Biquad Studio")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                tracing::info!("Quit requested from system tray");
                app.exit(0);
            }
            "show_window" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "toggle_mute" => {
                tracing::info!("Mute toggled from system tray");
                let _ = app.emit("hotkey-mute-toggle", ());
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(())
}
