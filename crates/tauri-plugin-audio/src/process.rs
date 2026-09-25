use serde::Serialize;
use sysinfo::System;

#[derive(Debug, Serialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub memory: u64,
}

#[tauri::command]
pub fn list_running_apps() -> Vec<ProcessInfo> {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    let mut processes: Vec<ProcessInfo> = sys
        .processes()
        .iter()
        .filter(|(_pid, process)| {
            // Filter out system processes and tiny background tasks
            process.memory() > 50_000_000 // Only >50MB RAM
        })
        .map(|(pid, process)| ProcessInfo {
            pid: pid.as_u32(),
            name: process.name().to_string_lossy().into_owned(),
            memory: process.memory(),
        })
        .collect();

    // Sort by memory usage descending
    processes.sort_by(|a, b| b.memory.cmp(&a.memory));
    
    // Deduplicate by name (only keep the main process of Chrome/Discord/etc)
    let mut seen = std::collections::HashSet::new();
    processes.retain(|p| seen.insert(p.name.clone()));

    processes.into_iter().take(20).collect()
}
