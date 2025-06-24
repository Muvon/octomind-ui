use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, State, Window};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command as TokioCommand;
use tokio::sync::mpsc;
use uuid::Uuid;

// Session configuration that matches CLI args exactly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    pub name: Option<String>,
    pub resume: Option<String>,
    pub model: Option<String>,
    pub temperature: f32,
    pub max_tokens: Option<u32>,
    pub role: String,
    pub directory: String,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            name: None,
            resume: None,
            model: None,
            temperature: 0.7,
            max_tokens: None,
            role: "developer".to_string(),
            directory: env::current_dir()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
        }
    }
}
// Running session process
#[derive(Debug)]
pub struct SessionProcess {
    pub config: SessionConfig,
    pub stdin_tx: mpsc::UnboundedSender<String>,
    pub process_handle: tokio::task::JoinHandle<()>,
}

// Session state management
pub type SessionStates = Mutex<HashMap<String, SessionConfig>>;
pub type SessionProcesses = Arc<Mutex<HashMap<String, SessionProcess>>>;

#[tauri::command]
pub async fn get_available_sessions() -> Result<Vec<String>, String> {
    // TODO!
    // We should store it ONLY in local storage
    Ok(vec![])
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
        resume: None,
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
pub async fn resume_session_config(
    session_name: String,
    directory: String,
    model: Option<String>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
    role: Option<String>,
    state: State<'_, SessionStates>,
) -> Result<String, String> {
    let session_id = Uuid::new_v4().to_string();

    let config = SessionConfig {
        name: None,
        resume: Some(session_name),
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
pub async fn start_session_process(
    session_id: String,
    state: State<'_, SessionStates>,
    processes: State<'_, SessionProcesses>,
    window: Window,
) -> Result<(), String> {
    let session_config = {
        let sessions = state.lock().unwrap();
        sessions.get(&session_id).cloned()
    };

    let config = session_config.ok_or("Session not found")?;

    // Check if process already exists
    {
        let processes_guard = processes.lock().unwrap();
        if processes_guard.contains_key(&session_id) {
            return Ok(()); // Process already running
        }
    }

    // Change to the session directory
    let original_dir = std::env::current_dir().map_err(|e| e.to_string())?;
    std::env::set_current_dir(&config.directory).map_err(|e| e.to_string())?;

    // Build octomind session command
    let mut cmd = TokioCommand::new("octomind");
    cmd.arg("session");

    // Add session arguments based on config
    if let Some(name) = &config.name {
        cmd.arg("--name").arg(name);
    }
    if let Some(resume) = &config.resume {
        cmd.arg("--resume").arg(resume);
    }
    if let Some(model) = &config.model {
        cmd.arg("--model").arg(model);
    }
    cmd.arg("--temperature").arg(config.temperature.to_string());
    if let Some(max_tokens) = config.max_tokens {
        cmd.arg("--max-tokens").arg(max_tokens.to_string());
    }
    cmd.arg("--role").arg(&config.role);

    // Configure for interactive mode with streaming
    cmd.stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // Start the process
    let mut child = cmd.spawn().map_err(|e| {
        format!(
            "Failed to start octomind session: {}. Make sure 'octomind' is in your PATH.",
            e
        )
    })?;

    // Get stdin handle
    let stdin = child.stdin.take().ok_or("Failed to get stdin handle")?;
    let stdout = child.stdout.take().ok_or("Failed to get stdout handle")?;
    let stderr = child.stderr.take().ok_or("Failed to get stderr handle")?;

    // Create channel for sending messages to stdin
    let (stdin_tx, mut stdin_rx) = mpsc::unbounded_channel::<String>();

    // Restore original directory
    std::env::set_current_dir(original_dir).map_err(|e| e.to_string())?;

    let session_id_clone = session_id.clone();
    let window_clone = window.clone();

    // Spawn task to handle the session process
    let process_handle = tokio::spawn(async move {
        let mut stdin = stdin;
        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);

        // Handle stdin writes
        let stdin_handle = {
            tokio::spawn(async move {
                while let Some(message) = stdin_rx.recv().await {
                    if let Err(e) = stdin.write_all(format!("{}\n", message).as_bytes()).await {
                        eprintln!("Failed to write to stdin: {}", e);
                        break;
                    }
                    if let Err(e) = stdin.flush().await {
                        eprintln!("Failed to flush stdin: {}", e);
                        break;
                    }
                }
            })
        };

        // Handle stdout streaming
        let session_id_stdout = session_id_clone.clone();
        let window_stdout = window_clone.clone();
        let stdout_handle = tokio::spawn(async move {
            let mut lines = stdout_reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                // Convert ANSI colors to HTML and emit to frontend
                let html_line = ansi_to_html(&line);
                let _ = window_stdout.emit(
                    "session_output",
                    serde_json::json!({
                        "session_id": session_id_stdout,
                        "type": "stdout",
                        "content": html_line
                    }),
                );
            }
        });

        // Handle stderr streaming
        let session_id_stderr = session_id_clone.clone();
        let window_stderr = window_clone.clone();
        let stderr_handle = tokio::spawn(async move {
            let mut lines = stderr_reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                // Convert ANSI colors to HTML and emit to frontend
                let html_line = ansi_to_html(&line);
                let _ = window_stderr.emit(
                    "session_output",
                    serde_json::json!({
                        "session_id": session_id_stderr,
                        "type": "stderr",
                        "content": html_line
                    }),
                );
            }
        });

        // Wait for process to complete
        let _ = tokio::join!(stdin_handle, stdout_handle, stderr_handle);

        // Wait for child process
        if let Err(e) = child.wait().await {
            let _ = window_clone.emit(
                "session_output",
                serde_json::json!({
                    "session_id": session_id_clone,
                    "type": "error",
                    "content": format!("Process error: {}", e)
                }),
            );
        }

        // Emit session ended event
        let _ = window_clone.emit(
            "session_ended",
            serde_json::json!({
                "session_id": session_id_clone
            }),
        );
    });

    // Store the process
    let session_process = SessionProcess {
        config,
        stdin_tx,
        process_handle,
    };

    {
        let mut processes_guard = processes.lock().unwrap();
        processes_guard.insert(session_id, session_process);
    }

    Ok(())
}

#[tauri::command]
pub async fn send_message_to_session(
    session_id: String,
    message: String,
    processes: State<'_, SessionProcesses>,
) -> Result<(), String> {
    let processes_guard = processes.lock().unwrap();
    let session_process = processes_guard
        .get(&session_id)
        .ok_or("Session process not found. Please start the session first.")?;

    session_process
        .stdin_tx
        .send(message)
        .map_err(|e| format!("Failed to send message to session: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn stop_session_process(
    session_id: String,
    processes: State<'_, SessionProcesses>,
) -> Result<(), String> {
    let session_process = {
        let mut processes_guard = processes.lock().unwrap();
        processes_guard.remove(&session_id)
    };

    if let Some(session_process) = session_process {
        // Send exit command
        let _ = session_process.stdin_tx.send("/exit".to_string());

        // Wait a bit for graceful shutdown, then abort if needed
        tokio::time::timeout(
            std::time::Duration::from_secs(5),
            session_process.process_handle,
        )
        .await
        .ok();
    }

    Ok(())
}

// Convert ANSI escape codes to HTML
fn ansi_to_html(text: &str) -> String {
    // Simple ANSI to HTML conversion
    let mut result = text.to_string();

    // Remove ANSI escape sequences and replace with HTML
    // Reset codes
    result = result.replace("\x1b[0m", "</span>");
    result = result.replace("\x1b[m", "</span>");

    // Colors
    result = result.replace("\x1b[30m", "<span style=\"color: #000000\">"); // Black
    result = result.replace("\x1b[31m", "<span style=\"color: #ff0000\">"); // Red
    result = result.replace("\x1b[32m", "<span style=\"color: #00ff00\">"); // Green
    result = result.replace("\x1b[33m", "<span style=\"color: #ffff00\">"); // Yellow
    result = result.replace("\x1b[34m", "<span style=\"color: #0000ff\">"); // Blue
    result = result.replace("\x1b[35m", "<span style=\"color: #ff00ff\">"); // Magenta
    result = result.replace("\x1b[36m", "<span style=\"color: #00ffff\">"); // Cyan
    result = result.replace("\x1b[37m", "<span style=\"color: #ffffff\">"); // White

    // Bright colors
    result = result.replace("\x1b[90m", "<span style=\"color: #808080\">"); // Bright Black
    result = result.replace("\x1b[91m", "<span style=\"color: #ff8080\">"); // Bright Red
    result = result.replace("\x1b[92m", "<span style=\"color: #80ff80\">"); // Bright Green
    result = result.replace("\x1b[93m", "<span style=\"color: #ffff80\">"); // Bright Yellow
    result = result.replace("\x1b[94m", "<span style=\"color: #8080ff\">"); // Bright Blue
    result = result.replace("\x1b[95m", "<span style=\"color: #ff80ff\">"); // Bright Magenta
    result = result.replace("\x1b[96m", "<span style=\"color: #80ffff\">"); // Bright Cyan
    result = result.replace("\x1b[97m", "<span style=\"color: #ffffff\">"); // Bright White

    // Bold
    result = result.replace("\x1b[1m", "<span style=\"font-weight: bold\">");

    // Clean up any remaining ANSI codes with a simple pattern
    // Note: This is a basic implementation, for production you might want to use a proper regex library
    let chars: Vec<char> = result.chars().collect();
    let mut cleaned = String::new();
    let mut i = 0;

    while i < chars.len() {
        if i < chars.len() - 1 && chars[i] == '\x1b' && chars[i + 1] == '[' {
            // Skip ANSI escape sequence
            i += 2;
            while i < chars.len() && chars[i] != 'm' {
                i += 1;
            }
            if i < chars.len() {
                i += 1; // Skip the 'm'
            }
        } else {
            cleaned.push(chars[i]);
            i += 1;
        }
    }

    cleaned
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

    // Use blocking version since we're in a command context
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

    // Add some common directories
    if let Some(home_dir) = dirs::home_dir() {
        dirs.push(home_dir.to_string_lossy().to_string());
    }

    Ok(dirs)
}
