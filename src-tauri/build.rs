fn main() {
    let mut attrs = tauri_build::Attributes::new();
    
    #[cfg(target_os = "windows")]
    {
        attrs = attrs.windows_attributes(tauri_build::WindowsAttributes::new());
    }

    tauri_build::try_build(attrs).expect("Failed to build Tauri resources");
}
