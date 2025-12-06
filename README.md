# <img src="assets/logo.png" width="48" height="48" align="top" style="margin-right: 10px;"> GhostKeys

> **Bringing ABNT2 souls back to life on US Keyboards.** 👻

GhostKeys is a Windows System Tray application written in Rust that performs low-level keyboard interception. It allows you to type fluently in Brazilian Portuguese (ABNT2) while keeping your OS in English (US Layout) for coding shortcuts.

It's the best of both worlds: **US Layout for Code** + **ABNT2 for Text**.

## 🎥 Demo

[Link to Video Demo Here]

## 👻 Features

-   **Zero Config:** Runs in the system tray.
-   **Positional Mapping:** Intercepts physical keys. Type `;` to get `ç`. Type `[` to prepare an acute accent (`´`).
-   **Safe:** Panic-safe implementation ensures your keyboard is never stuck.
-   **Lightweight:** Built in Rust with native Windows API (`windows-rs`). <5MB RAM.

## 🚀 Installation

1.  Download `ghostkeys.exe` from [Releases](../../releases).
2.  Run it (Allow Windows Defender if it gets scared of ghosts).
3.  Look for the Ghost icon in your System Tray.
4.  **Usage:**
    -   Press `;` (next to L) → Outputs `ç`
    -   Press `[` (next to P) → Prepares Accent `´`
    -   Press `'` (next to ;) → Prepares Tilde `~`

## 🧠 How we built it (The Kiro Workflow)

This project was built for **Kiroween 2025** using a **Spec-Driven Development** approach with Kiro.

1.  **Context-First:** We defined the "Positional Mapping" logic in Markdown specs (`.kiro/specs/`) before writing a single line of Rust.
2.  **Safety & Speed:** We used `windows-rs` for safe API hooks and `cargo-xwin` to cross-compile from Linux Containers to Windows 11.
3.  **Agent Automation:** We configured Kiro Agent Hooks (`.kiro/hooks.yml`) to audit our code safety on every save.
4.  **MCP:** We utilized a static context strategy to ground the LLM in our specific ABNT2 mapping requirements.

## 🛠️ Tech Stack

-   **Language:** Rust (Stable)
-   **Core:** `windows-rs` (Win32 Hooks)
-   **UI:** `tray-icon` + `tao`
-   **Dev:** Kiro.dev

## 🔧 Development

### Prerequisites

**Using DevContainer (Recommended):**
Open in VS Code/Kiro with the Dev Containers extension. All dependencies are pre-installed.

**Manual Setup (Linux/WSL):**

```bash
# Install system dependencies
sudo apt-get update && sudo apt-get install -y \
    libx11-dev libxcb1-dev libxkbcommon-dev pkg-config \
    libxi-dev libxtst-dev libxrandr-dev libxcursor-dev \
    libgtk-3-dev clang lld libxdo-dev libayatana-appindicator3-dev

# Install Rust tools
cargo install cargo-xwin just git-cliff
```

### Task Runner

We use [just](https://github.com/casey/just) as our task runner (like npm scripts):

```bash
# Install just
cargo install just

# See all available commands
just

# Common tasks
just build              # Build debug
just build-release      # Build release
just build-windows      # Cross-compile to Windows
just test               # Run tests
just lint               # Run clippy
just fmt                # Format code
just ci                 # Run all CI checks
```

### Building

```bash
# Linux (native)
cargo build --release

# Windows (cross-compile from Linux)
cargo xwin build --target x86_64-pc-windows-msvc --release

# Output: target/x86_64-pc-windows-msvc/release/ghostkeys.exe
```

### Testing

```bash
cargo test                    # Run all tests
cargo test -- --nocapture     # With output
just test-verbose             # Same as above
```

## 📦 Release Process

Releases are automated via GitHub Actions. To create a release:

```bash
git tag v1.0.0
git push origin v1.0.0
```

This triggers the workflow that:

1. Builds `ghostkeys.exe` on Windows
2. Generates changelog from commits
3. Creates SHA256 checksum
4. Publishes GitHub Release

## 📄 License

Apache-2.0 - See [LICENSE](LICENSE) for details.
