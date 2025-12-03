# GhostKeys Library Research

Date: December 3, 2025

## Overview

This document summarizes research into the safest library choices for GhostKeys, a Windows System Tray application that intercepts keyboard input to provide ABNT2 keyboard layout emulation on US keyboards.

## Original Libraries from Design Document

| Library | Purpose | Safety Assessment |
|---------|---------|-------------------|
| tao | Event loop and window management | Safe - Maintained by Tauri team, widely used in production apps, active development, 1.4k+ GitHub stars |
| tray-icon | System tray icon and menu | Safe - Also maintained by Tauri team, companion to tao, actively maintained |
| rdev | Low-level keyboard hook (listen/grab/simulate) | Use with caution - Popular (~500 stars), but requires elevated permissions on some platforms. Last significant update was mid-2024 |
| active-win-pos-rs | Get active window info (process name, title) | Moderate risk - Smaller community (~100 stars), less frequent updates |
| proptest | Property-based testing | Safe - Industry standard for Rust PBT, 1.6k+ stars, actively maintained |
| thiserror | Error type derivation | Safe - De facto standard for error handling in Rust, maintained by dtolnay (highly trusted) |

## Reference Projects Analyzed

### Kanata (keyboard remapper, 3k+ stars)
- Uses **interception driver** on Windows (via `interception` crate) - kernel-level, very low latency
- Also supports **kanata-keyberon** for key processing
- Uses **winapi** / **windows-rs** for direct Windows API calls
- No system tray - runs as a background service

### Espanso (text expander, 10k+ stars)
- Uses **windows-rs** (Microsoft's official Rust bindings) for keyboard hooks on Windows
- Uses **wry** + **tao** for UI (same Tauri ecosystem)
- Custom implementation for window detection via Windows API
- Has its own **espanso-detect** and **espanso-inject** modules

## Recommended Safer Alternatives

| Current Library | Safer Alternative | Used By | Why It's Safer |
|-----------------|-------------------|---------|----------------|
| rdev | windows-rs (direct WH_KEYBOARD_LL hook) | Espanso | Microsoft-maintained, 5k+ stars, official bindings, production-proven |
| rdev | interception driver | Kanata | Kernel-level, lowest latency, but requires driver install |
| active-win-pos-rs | windows-rs (GetForegroundWindow + GetWindowThreadProcessId) | Espanso | Direct API calls, no middleman crate |
| tao + tray-icon | Keep as-is | Espanso (uses tao) | Already the safest choice for this purpose |
| proptest | Keep as-is | Many projects | Industry standard |
| thiserror | Keep as-is | Many projects | Industry standard |

## Recommendation

Replace `rdev` and `active-win-pos-rs` with direct **windows-rs** calls. This is what Espanso does, and it provides:

1. **Microsoft-maintained bindings** - Official support from Microsoft
2. **No dependency on smaller community crates** - Reduces supply chain risk
3. **Full control over Windows API calls** - Better debugging and customization
4. **Better long-term maintainability** - Won't be abandoned

The tradeoff is more boilerplate code for the Windows API calls, but it's safer and more reliable for a keyboard interception application.

## Final Library Selection

| Library | Purpose | Notes |
|---------|---------|-------|
| windows-rs | Keyboard hooks, key injection, window detection | Microsoft official, replaces rdev and active-win-pos-rs |
| tao | Event loop | Tauri ecosystem, production-proven |
| tray-icon | System tray | Tauri ecosystem, companion to tao |
| proptest | Property-based testing | Industry standard |
| thiserror | Error handling | Industry standard |

## Standard Library Dependencies

- `std::sync` (Arc, Mutex, AtomicBool) - Thread-safe state sharing
- `std::time` (Instant, Duration) - Timeout handling
- `std::collections` (HashMap) - Character mapping tables
