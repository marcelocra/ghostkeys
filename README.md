# GhostKeys

A Windows System Tray application that enables Brazilian Portuguese (ABNT2) typing on US-International keyboards.

## Overview

GhostKeys intercepts keyboard input to simulate ABNT2 layout behavior, allowing seamless typing of Portuguese characters like `ç`, `ã`, `õ` using dead key sequences on a US-International keyboard.

## Features

- **Dead Key Remapping**: Converts US-International dead key sequences to ABNT2 characters
  - `'` + `c` → `ç` / `'` + `C` → `Ç`
  - `~` + `a` → `ã` / `~` + `A` → `Ã`
  - `~` + `o` → `õ` / `~` + `O` → `Õ`

- **Context-Aware Mode**: Automatically disables remapping in specific applications (e.g., VSCode) to preserve native shortcuts

- **System Tray Integration**: Runs quietly in the background with a context menu for Pause/Resume/Exit

- **Low Latency**: Processes keystrokes in under 10ms with minimal resource usage

- **Panic Safety**: Automatically releases the keyboard hook on crash to prevent keyboard lockup

## Requirements

- Windows 10/11
- Rust toolchain (for building from source)

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/ghostkeys.git
cd ghostkeys

# Build release binary
cargo build --release

# The executable will be at target/release/ghostkeys.exe
```

## Usage

1. Run `ghostkeys.exe`
2. The application will appear in the System Tray
3. Right-click the tray icon for options:
   - **Pause**: Disable remapping globally
   - **Resume**: Re-enable context-aware remapping
   - **Exit**: Close the application

## Configuration

By default, GhostKeys operates in:
- **Passthrough Mode** for: Visual Studio Code (`Code.exe`)
- **Active Mode** for: Slack and all other applications

## How It Works

GhostKeys uses a state machine to handle dead key sequences:

1. **Idle State**: Waiting for input
2. **PendingAccent State**: Dead key pressed, waiting for next character
3. If a combinable character follows, outputs the combined character
4. If a non-combinable character follows, outputs both characters separately
5. If 500ms timeout elapses, outputs the dead key alone

## License

MIT

## Contributing

Contributions are welcome! Please read the requirements specification in `.kiro/specs/ghostkeys/requirements.md` before submitting changes.
