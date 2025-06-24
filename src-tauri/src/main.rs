#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod commands;

use commands::{SessionProcesses, SessionStates};
use std::collections::HashMap;
use std::sync::Mutex;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(SessionStates::new(HashMap::new()))
        .manage(SessionProcesses::new(Mutex::new(HashMap::new())))
        .invoke_handler(tauri::generate_handler![
            commands::get_available_sessions,
            commands::create_session_config,
            commands::resume_session_config,
            commands::start_session_process,
            commands::send_message_to_session,
            commands::stop_session_process,
            commands::get_session_info,
            commands::list_directories,
            commands::select_directory_native,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
