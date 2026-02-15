#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod commands;

use commands::{ServerProcess, SessionStates};
use std::collections::HashMap;
use std::sync::Mutex;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(SessionStates::new(HashMap::new()))
        .manage(ServerProcess::new(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            commands::start_octomind_server,
            commands::stop_octomind_server,
            commands::is_server_running,
            commands::create_session_config,
            commands::get_session_info,
            commands::list_directories,
            commands::select_directory_native,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
