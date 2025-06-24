# Development Guide - Octomind UI

## 🛠 Tauri v2 Development Reference

### **Version Information**
- **Tauri**: `2.0` (latest stable)
- **Tauri CLI**: `2.0`
- **Rust**: `1.87+` (latest stable)
- **Architecture**: Native macOS with glassmorphism UI

### **Key Dependencies**
```toml
[dependencies]
tauri = { version = "2.0", features = ["macos-private-api"] }
tauri-plugin-shell = "2.0"
tauri-plugin-dialog = "2.0"
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
```

## 📁 Project Structure Deep Dive

### **Frontend Architecture**
```
src/
├── index.html          # Main UI with embedded CSS/JS
├── index.css          # Modern macOS styling (if separated)
└── index.js           # Session management logic (if separated)
```

**Key Frontend Components:**
- **Session Manager**: Handles CLI subprocess communication
- **ANSI Parser**: Converts terminal colors to HTML
- **Storage Manager**: localStorage for session persistence
- **UI Controller**: DOM manipulation and event handling

### **Backend Architecture**
```
src-tauri/src/
├── main.rs            # App initialization with plugins
├── lib.rs             # Public API exports
└── commands.rs        # Tauri command implementations
```

**Core Backend Systems:**
- **Session Process Management**: Real subprocess handling
- **Event Streaming**: stdout/stderr to frontend events
- **State Management**: In-memory session configurations
- **Dialog Integration**: Native file/folder selection

## 🔧 Development Workflow

### **Setup & Installation**
```bash
# Install Tauri CLI v2
cargo install tauri-cli@^2.0

# Verify installation
cargo tauri --version

# Clone and setup
git clone <repo>
cd octomind-ui
cargo check
```

### **Development Commands**
```bash
# Development mode (hot reload)
cargo tauri dev

# Production build
cargo tauri build

# Check compilation without running
cargo check

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy
```

### **Debugging**
```bash
# Debug mode with console
cargo tauri dev --debug

# Verbose logging
RUST_LOG=debug cargo tauri dev

# Check configuration
cargo tauri info
```

## 🏗 Architecture Patterns

### **Command Pattern**
All backend functions use the `#[tauri::command]` attribute:

```rust
#[tauri::command]
pub async fn command_name(
    app: AppHandle,           // For accessing app context
    state: State<'_, Type>,   // For shared state
    param: String             // Parameters from frontend
) -> Result<ReturnType, String> {
    // Implementation
    Ok(result)
}
```

### **Event Streaming Pattern**
For real-time communication:

```rust
// Backend: Emit events
window.emit("event_name", payload)?;

// Frontend: Listen for events
await listen('event_name', (event) => {
    handleEvent(event.payload);
});
```

### **State Management Pattern**
```rust
// Define state types
pub type SessionStates = Arc<Mutex<HashMap<String, SessionConfig>>>;
pub type SessionProcesses = Arc<Mutex<HashMap<String, SessionProcess>>>;

// Register in main.rs
.manage(SessionStates::new(HashMap::new()))
.manage(SessionProcesses::new(Mutex::new(HashMap::new())))

// Use in commands
state: State<'_, SessionStates>
```

## 🎨 UI Development Guidelines

### **CSS Architecture**
- **Base Styles**: Reset, typography, colors
- **Component Styles**: Buttons, forms, cards
- **Layout Styles**: Grid, flexbox, spacing
- **Effect Styles**: Glassmorphism, animations

### **Design Tokens**
```css
:root {
  /* Colors */
  --primary: #8bc34a;
  --bg-primary: rgba(26, 26, 26, 0.85);
  --text-primary: #e5e5e5;

  /* Effects */
  --blur-light: blur(15px);
  --blur-heavy: blur(30px);
  --transition: 0.2s ease;

  /* Spacing */
  --space-xs: 4px;
  --space-sm: 8px;
  --space-md: 16px;
  --space-lg: 24px;
  --space-xl: 32px;
}
```

### **Component Guidelines**
1. **Consistent Blur**: Use `backdrop-filter: blur(20-30px)`
2. **Smooth Animations**: Always `0.2s ease` transitions
3. **Proper Contrast**: Ensure accessibility with transparency
4. **Native Fonts**: Use SF Pro font stack on macOS

## 🔌 Plugin Integration

### **Dialog Plugin Usage**
```rust
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
pub async fn select_folder(app: AppHandle) -> Result<Option<String>, String> {
    let folder = app.dialog()
        .file()
        .set_directory(env::current_dir().unwrap_or_default())
        .blocking_pick_folder(); // Use blocking_ for commands

    Ok(folder.map(|p| p.to_string()))
}
```

### **Shell Plugin Usage**
```rust
use tauri_plugin_shell::ShellExt;

#[tauri::command]
pub async fn run_command(app: AppHandle, cmd: String) -> Result<String, String> {
    let output = app.shell()
        .command("sh")
        .args(["-c", &cmd])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
```

## 🐛 Common Issues & Solutions

### **Build Issues**

**Problem**: `failed to open icon`
```bash
# Solution: Create minimal icon
mkdir -p icons
echo "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==" | base64 -d > icons/icon.png
```

**Problem**: Plugin not found
```bash
# Solution: Check Cargo.toml dependencies and main.rs plugins
.plugin(tauri_plugin_dialog::init())
.plugin(tauri_plugin_shell::init())
```

### **Runtime Issues**

**Problem**: Command not working
```rust
// Check: Command is registered in main.rs
.invoke_handler(tauri::generate_handler![
    commands::your_command,
])

// Check: Proper async/await usage
#[tauri::command]
pub async fn your_command() -> Result<String, String> {
    // Implementation
}
```

**Problem**: Events not received
```javascript
// Check: Event name matches exactly
// Backend
window.emit("session_output", data)?;

// Frontend
await listen('session_output', handler);
```

### **UI Issues**

**Problem**: Transparency not working
```json
// Check tauri.conf.json
{
  "app": {
    "windows": [{
      "transparent": true
    }]
  }
}
```

**Problem**: Fonts not loading
```css
/* Use proper macOS font stack */
font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Display', sans-serif;
```

## 📚 Reference Links

### **Official Documentation**
- [Tauri v2 Guide](https://v2.tauri.app/)
- [Configuration Reference](https://v2.tauri.app/reference/config/)
- [Plugin System](https://v2.tauri.app/plugin/)
- [Command System](https://v2.tauri.app/develop/calling-rust/)

### **Plugin Documentation**
- [Dialog Plugin](https://v2.tauri.app/plugin/dialog/)
- [Shell Plugin](https://v2.tauri.app/plugin/shell/)
- [File System Plugin](https://v2.tauri.app/plugin/file-system/)

### **Design Resources**
- [macOS Human Interface Guidelines](https://developer.apple.com/design/human-interface-guidelines/macos)
- [SF Pro Fonts](https://developer.apple.com/fonts/)
- [Glassmorphism Design](https://uxdesign.cc/glassmorphism-in-user-interfaces-1f39bb1308c9)

## 🔄 Migration Notes

### **From Tauri v1 to v2**
Key changes made during upgrade:

1. **Configuration Format**: Complete restructure of `tauri.conf.json`
2. **Plugin System**: Moved from allowlist to dedicated plugins
3. **Event System**: Enhanced with better typing and performance
4. **Window Management**: Improved transparency and vibrancy support
5. **Command System**: Better async/await integration

### **Breaking Changes Handled**
- ✅ Updated configuration schema
- ✅ Migrated to plugin-based architecture
- ✅ Fixed command signatures and imports
- ✅ Updated dialog API usage
- ✅ Enhanced event emission patterns

This guide should help maintain and extend the Octomind UI with confidence in the Tauri v2 ecosystem.
