use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, State, Window};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as TokioCommand;
use uuid::Uuid;

// Server process state
pub type ServerProcess = Arc<Mutex<Option<tokio::process::Child>>>;

// Session configuration for history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    pub name: Option<String>,
    pub directory: String,
    pub model: Option<String>,
    pub temperature: f32,
    pub max_tokens: Option<u32>,
    pub role: String,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            name: None,
            directory: env::current_dir()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            model: None,
            temperature: 0.7,
            max_tokens: None,
            role: "developer".to_string(),
        }
    }
}

pub type SessionStates = Mutex<HashMap<String, SessionConfig>>;

#[tauri::command]
pub async fn start_octomind_server(
    port: Option<u16>,
    server_process: State<'_, ServerProcess>,
    window: Window,
) -> Result<u16, String> {
    let port = port.unwrap_or(8080);
    
    // Check if already running
    {
        let guard = server_process.lock().unwrap();
        if guard.is_some() {
            return Ok(port);
        }
    }
    
    // Start octomind server
    let mut cmd = TokioCommand::new("octomind");
    cmd.arg("server")
        .arg("--port").arg(port.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    
    let mut child = cmd.spawn().map_err(|e| {
        format!("Failed to start octomind server: {}. Make sure 'octomind' is in your PATH.", e)
    })?;
    
    let stdout = child.stdout.take().ok_or("Failed to get stdout")?;
    let stderr = child.stderr.take().ok_or("Failed to get stderr")?;
    
    // Store the process
    {
        let mut guard = server_process.lock().unwrap();
        *guard = Some(child);
    }
    
    // Spawn task to handle server output
    tokio::spawn(async move {
        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);
        
        let mut stdout_lines = stdout_reader.lines();
        let mut stderr_lines = stderr_reader.lines();
        
        loop {
            tokio::select! {
                Ok(Some(line)) = stdout_lines.next_line() => {
                    let _ = window.emit("server_output", serde_json::json!({
                        "type": "stdout",
                        "content": line
                    }));
                }
                Ok(Some(line)) = stderr_lines.next_line() => {
                    let _ = window.emit("server_output", serde_json::json!({
                        "type": "stderr",
                        "content": line
                    }));
                }
                _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {}
            }
        }
    });
    
    // Give server time to start
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    
    Ok(port)
}

#[tauri::command]
pub async fn stop_octomind_server(
    server_process: State<'_, ServerProcess>,
) -> Result<(), String> {
    // Take the child out of the mutex before awaiting
    let child = {
        let mut guard = server_process.lock().unwrap();
        guard.take()
    };
    
    if let Some(mut child) = child {
        child.kill().await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn is_server_running(
    server_process: State<'_, ServerProcess>,
) -> Result<bool, String> {
    let guard = server_process.lock().unwrap();
    Ok(guard.is_some())
}

#[tauri::command]
pub async fn create_session_config(
    name: Option<String>,
    directory: String,
    model: Option<String>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
    role: Option<String>,
    state: State<'_, SessionStates>,
) -> Result<String, String> {
    let session_id = Uuid::new_v4().to_string();

    let config = SessionConfig {
        name,
        model,
        temperature: temperature.unwrap_or(0.7),
        max_tokens,
        role: role.unwrap_or_else(|| "developer".to_string()),
        directory,
    };

    state.lock().unwrap().insert(session_id.clone(), config);
    Ok(session_id)
}

#[tauri::command]
pub async fn get_session_info(
    session_id: String,
    state: State<'_, SessionStates>,
) -> Result<SessionConfig, String> {
    let sessions = state.lock().unwrap();
    sessions
        .get(&session_id)
        .cloned()
        .ok_or("Session not found".to_string())
}

#[tauri::command]
pub async fn select_directory_native(app: AppHandle) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;

    let folder_path = app
        .dialog()
        .file()
        .set_directory(std::env::current_dir().unwrap_or_default())
        .blocking_pick_folder();

    match folder_path {
        Some(path) => Ok(Some(path.to_string())),
        None => Ok(None),
    }
}

#[tauri::command]
pub async fn list_directories() -> Result<Vec<String>, String> {
    let current_dir = env::current_dir().map_err(|e| e.to_string())?;
    let mut dirs = vec![current_dir.to_string_lossy().to_string()];

    if let Some(home_dir) = dirs::home_dir() {
        dirs.push(home_dir.to_string_lossy().to_string());
    }

    Ok(dirs)
}
