# Changelog - Octomind UI

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-28

### 🚀 Major Release - Tauri v2 Upgrade & Modern macOS UI

#### Added
- **Tauri v2 Support**: Complete upgrade from Tauri v1 to v2 with modern plugin architecture
- **Native macOS Design**: Glassmorphism effects with window transparency and backdrop blur
- **Real-time CLI Integration**: Live subprocess management with streaming stdin/stdout
- **Session Persistence**: localStorage-backed session history with cost tracking
- **ANSI Color Support**: Terminal color codes converted to HTML for rich output
- **Native File Dialogs**: macOS-native directory selection using tauri-plugin-dialog
- **Modern Typography**: SF Pro font family with proper letter spacing
- **Smooth Animations**: Consistent 0.2s ease transitions throughout the interface
- **Enhanced Session Management**: Auto-naming, deletion, and resume functionality
- **Cost Tracking**: Per-session cost monitoring with automatic parsing

#### Changed
- **Architecture**: Migrated from Tauri v1 allowlist to v2 plugin system
- **Configuration**: Updated `tauri.conf.json` to v2 format with proper window settings
- **Dependencies**: Upgraded all Tauri-related dependencies to v2.0
- **UI Framework**: Implemented modern glassmorphism design system
- **Command System**: Enhanced with proper async/await and error handling
- **Event System**: Improved real-time communication between frontend and backend

#### Technical Details

##### **Tauri v2 Migration**
- **Dependencies Updated**:
  - `tauri: 1.8 → 2.0`
  - `tauri-build: 1.5 → 2.0`
  - Added `tauri-plugin-shell: 2.0`
  - Added `tauri-plugin-dialog: 2.0`

- **Configuration Changes**:
  - Migrated `tauri.conf.json` from v1 to v2 format
  - Enabled window transparency and native effects
  - Updated plugin system configuration
  - Removed deprecated allowlist configuration

- **Code Changes**:
  - Updated imports to use `tauri::Emitter` trait
  - Migrated dialog API to use `blocking_pick_folder()`
  - Enhanced command signatures with proper error handling
  - Added plugin initialization in `main.rs`

##### **UI/UX Enhancements**
- **Glassmorphism Effects**:
  - `backdrop-filter: blur(30px) saturate(180%)`
  - Transparent window background
  - Layered blur effects for depth

- **Typography System**:
  - SF Pro Display/Text font stack
  - Optimized letter spacing (-0.01em to -0.02em)
  - Consistent font weights (400, 500, 600)

- **Color System**:
  - Primary: `#8bc34a` (Octomind green)
  - Background: `rgba(26, 26, 26, 0.85)` with blur
  - Text: `#e5e5e5` (high contrast)
  - Accents: Green variants with transparency

- **Animation System**:
  - Consistent `0.2s ease` transitions
  - Hover effects with `translateY(-1px)`
  - Smooth color and opacity changes

##### **Architecture Improvements**
- **Session Management**:
  ```rust
  pub struct SessionProcess {
      pub config: SessionConfig,
      pub stdin_tx: mpsc::UnboundedSender<String>,
      pub process_handle: tokio::task::JoinHandle<()>,
  }
  ```

- **Real-time Communication**:
  - Subprocess stdin/stdout streaming
  - Event-based frontend updates
  - ANSI-to-HTML color conversion

- **State Management**:
  - In-memory session configurations
  - Persistent localStorage for history
  - Cost tracking and analytics

#### Files Modified
- `Cargo.toml` - Updated dependencies and metadata
- `tauri.conf.json` - Migrated to v2 format
- `src-tauri/src/main.rs` - Added plugin initialization
- `src-tauri/src/commands.rs` - Updated APIs and imports
- `src/index.html` - Complete UI redesign with modern styling
- `README.md` - Comprehensive documentation update
- `DEVELOPMENT.md` - New development guide
- `CHANGELOG.md` - This changelog

#### Breaking Changes
- **Tauri v1 Compatibility**: No longer compatible with Tauri v1
- **Configuration Format**: `tauri.conf.json` requires v2 format
- **Plugin Dependencies**: Requires separate plugin installations
- **Command Signatures**: Updated for v2 async patterns

#### Migration Notes
For developers upgrading from previous versions:

1. **Update Dependencies**: Use Tauri v2 packages
2. **Configuration**: Migrate `tauri.conf.json` to v2 format
3. **Plugins**: Install and configure required plugins
4. **Commands**: Update command signatures for v2 patterns
5. **Events**: Use new event emission patterns

#### Known Issues
- Icon files need to be generated for production builds
- Some advanced macOS vibrancy effects may require additional permissions

#### Future Roadmap
- Enhanced error handling and recovery
- Multi-window session management
- Advanced theme customization
- File drag-and-drop support
- Performance optimizations

---

## Development Guidelines

### Version Numbering
- **Major**: Breaking changes, architecture updates
- **Minor**: New features, UI improvements
- **Patch**: Bug fixes, small enhancements

### Changelog Format
- **Added**: New features
- **Changed**: Changes in existing functionality
- **Deprecated**: Soon-to-be removed features
- **Removed**: Now removed features
- **Fixed**: Bug fixes
- **Security**: Vulnerability fixes

### Release Process
1. Update version in `Cargo.toml`
2. Update this changelog
3. Test all functionality
4. Create release tag
5. Build and distribute

---

*This changelog is maintained manually and reflects all significant changes to the Octomind UI project.*
