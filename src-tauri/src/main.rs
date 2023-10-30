// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod external_path;
mod ipc;

use ipc::AppScanning;
use tauri::async_runtime::Mutex;

fn main() {
    let scanning_state: AppScanning = AppScanning(Mutex::new(None));
    tauri::Builder::default()
        .manage(scanning_state)
        .invoke_handler(tauri::generate_handler![
            ipc::start_scan,
            ipc::scan_step,
            ipc::get_aggregate_data,
            ipc::open_path,
            ipc::jump,
            ipc::level_up,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
