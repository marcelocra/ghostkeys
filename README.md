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
