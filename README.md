# Octomind UI

> **⚠️ ACTIVE DEVELOPMENT: NOT READY FOR PRODUCTION**
> 
> This feature is currently under active development and is not yet ready for production use. It may contain bugs, incomplete functionality, or undergo significant changes before final release.
> 
> Use at your own risk in development environments only.

A modern native macOS desktop application built with **Tauri v2** that provides a beautiful GUI wrapper for Octomind CLI sessions.

## ✨ Features

✅ **Native macOS Design**: Glassmorphism effects, transparency, and native styling
✅ **Real-time Session Management**: Live CLI subprocess integration with streaming I/O
✅ **Directory Selection**: Native file dialogs for working directory selection
✅ **Role Configuration**: Developer/assistant roles with custom model support
✅ **Session Persistence**: localStorage-backed session history with cost tracking
✅ **Interactive Chat**: Real-time messaging with ANSI color support
✅ **Modern UI**: SF Pro fonts, backdrop blur, and smooth animations
✅ **Cross-platform**: Built on Tauri v2 for excellent performance

## 🚀 Quick Start

### Prerequisites
- **Rust** (latest stable)
- **Node.js** (for frontend development)
- **Tauri CLI v2**: `cargo install tauri-cli@^2.0`

### Development
```bash
# Clone and navigate
cd octomind-ui

# Install dependencies (auto-handled by Cargo)
cargo check

# Run in development mode
cargo tauri dev

# Build for production
cargo tauri build
```

### Usage
1. **Create Session**: Fill form in sidebar, select directory, configure settings
2. **Live Chat**: Type messages, see real-time responses with syntax highlighting
3. **Resume Sessions**: Click any saved session to continue where you left off
4. **Session Management**: Delete old sessions, track costs, auto-naming

## 🏗 Architecture

### **Tauri v2 Stack**
- **Frontend**: HTML/CSS/JavaScript with modern glassmorphism design
- **Backend**: Rust with Tauri v2 commands and plugin system
- **CLI Integration**: Real subprocess management with stdin/stdout streaming
- **State Management**: Persistent session storage with cost tracking

### **Key Components**
```rust
// Core session management
pub struct SessionProcess {
    pub config: SessionConfig,
    pub stdin_tx: mpsc::UnboundedSender<String>,
    pub process_handle: tokio::task::JoinHandle<()>,
}

// Real-time CLI integration
start_session_process() -> subprocess with streaming I/O
send_message_to_session() -> live stdin communication
stop_session_process() -> clean termination
```

### **Modern UI Features**
- **Window Transparency**: Native macOS vibrancy effects
- **Backdrop Blur**: 30px blur with 180% saturation for glassmorphism
- **SF Pro Typography**: Native macOS font stack with proper spacing
- **ANSI Support**: Terminal color codes converted to HTML
- **Responsive Design**: Smooth 0.2s animations throughout

## 📁 File Structure

```
octomind-ui/
├── src/
│   ├── index.html              # Frontend UI with modern styling
│   ├── index.css              # Glassmorphism and native macOS design
│   └── index.js               # Session management and real-time I/O
├── src-tauri/
│   ├── src/
│   │   ├── main.rs            # Tauri v2 app with plugins
│   │   ├── lib.rs             # Library exports
│   │   └── commands.rs        # CLI subprocess commands
│   ├── Cargo.toml             # Tauri v2 dependencies
│   └── tauri.conf.json        # Tauri v2 configuration
├── icons/                     # App icons (auto-generated)
├── tests/                     # Integration tests
└── README.md                  # This file
```

## 🛠 Development Guide

### **Tauri v2 Key Changes**
- **Plugin System**: Uses `tauri-plugin-shell` and `tauri-plugin-dialog`
- **Configuration**: New v2 format in `tauri.conf.json`
- **Commands**: Enhanced with proper async/await support
- **Window Management**: Improved transparency and vibrancy options

### **Important Development Notes**

#### **Adding New Commands**
```rust
// In src-tauri/src/commands.rs
#[tauri::command]
pub async fn your_command(app: AppHandle, param: String) -> Result<String, String> {
    // Implementation
    Ok("result".to_string())
}

// Register in src-tauri/src/main.rs
.invoke_handler(tauri::generate_handler![
    commands::your_command,
    // ... other commands
])
```

#### **Frontend-Backend Communication**
```javascript
// Invoke Tauri commands
const result = await invoke('your_command', { param: 'value' });

// Listen to events
await listen('session_output', (event) => {
    console.log('Received:', event.payload);
});
```

#### **Session Management**
- **CLI Integration**: Real subprocess via `tokio::process::Command`
- **Streaming I/O**: `mpsc::UnboundedSender` for stdin, event emission for stdout
- **State Persistence**: localStorage for session history and metadata
- **Process Cleanup**: Proper termination handling in `stop_session_process`

#### **UI Styling Guidelines**
- **Colors**: Primary green `#8bc34a`, dark backgrounds with transparency
- **Animations**: Consistent `0.2s ease` transitions
- **Blur Effects**: `backdrop-filter: blur(20-30px)` for glassmorphism
- **Typography**: SF Pro fonts with `-0.01em` to `-0.02em` letter spacing
- **Spacing**: 8px base unit, 24px container padding

### **Debugging & Troubleshooting**

#### **Common Issues**
1. **Build Errors**: Check Tauri v2 dependency versions in `Cargo.toml`
2. **Dialog Issues**: Ensure `tauri-plugin-dialog` is properly configured
3. **Window Transparency**: Verify macOS permissions for window effects
4. **CLI Integration**: Check subprocess spawning and I/O handling

#### **Useful Commands**
```bash
# Check for compilation errors
cargo check

# Run tests
cargo test

# Build for specific target
cargo tauri build --target universal-apple-darwin

# Debug mode with console
cargo tauri dev --debug
```

#### **Configuration Reference**
- **Tauri v2 Docs**: https://v2.tauri.app/
- **Plugin Documentation**: https://v2.tauri.app/plugin/
- **Configuration Schema**: https://schema.tauri.app/config/2

## 🎨 Design System

### **Colors**
- **Primary**: `#8bc34a` (Octomind green)
- **Background**: `rgba(26, 26, 26, 0.85)` with blur
- **Text**: `#e5e5e5` (high contrast white)
- **Accents**: `rgba(139, 195, 74, 0.1-0.3)` (green variants)

### **Typography**
- **Font Stack**: `-apple-system, BlinkMacSystemFont, 'SF Pro Display', 'SF Pro Text'`
- **Weights**: 400 (regular), 500 (medium), 600 (semibold)
- **Letter Spacing**: Tight spacing for modern look

### **Effects**
- **Glassmorphism**: `backdrop-filter: blur(30px) saturate(180%)`
- **Shadows**: Subtle `rgba(0, 0, 0, 0.3)` for depth
- **Borders**: `rgba(139, 195, 74, 0.1-0.3)` for definition

## 📊 Current Status

🟢 **Completed**:
- ✅ Tauri v2 upgrade and modern architecture
- ✅ Real-time CLI subprocess integration
- ✅ Native macOS UI with glassmorphism effects
- ✅ Session persistence and cost tracking
- ✅ ANSI color support and streaming I/O
- ✅ Native file dialogs and directory selection

🟡 **In Progress**:
- 🔄 Enhanced error handling and recovery
- 🔄 Advanced session management features
- 🔄 Performance optimizations

🔮 **Future Enhancements**:
- 📁 File drag-and-drop support
- 🎨 Theme customization options
- 📱 Multi-window session management
- 🔍 Advanced search and filtering
- 📊 Enhanced analytics and reporting

## 🤝 Contributing

When contributing to the UI:
1. **Follow the design system** outlined above
2. **Test on macOS** for native appearance
3. **Maintain Tauri v2 compatibility**
4. **Update this README** for significant changes

This provides a **production-ready native macOS application** with modern design and full CLI integration.
