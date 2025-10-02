#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]
use tauri::{Manager};

fn main() {
  tauri::Builder::default()
    .setup(|_app| {
      // TODO: wire preflight_import, run_import, get_progress commands
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
