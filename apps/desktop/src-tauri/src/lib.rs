//! KataBox desktop shell: wires Tauri commands to the core crates.
//! Keep business logic out of this crate; it belongs in `crates/`.

use kb_core::Language;
use serde::Serialize;

#[derive(Serialize)]
struct AppInfo {
    version: &'static str,
    languages: Vec<&'static str>,
}

/// First IPC command: proves the UI <-> Rust bridge works end to end.
#[tauri::command]
fn app_info() -> AppInfo {
    AppInfo {
        version: env!("CARGO_PKG_VERSION"),
        languages: Language::ALL.iter().map(|l| l.id()).collect(),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![app_info])
        .run(tauri::generate_context!())
        .expect("error while running KataBox");
}
