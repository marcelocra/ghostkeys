# Approved Libraries for GhostKeys

This steering file defines the approved libraries for the project based on security research.

## Core Dependencies

| Library | Purpose | Justification |
|---------|---------|---------------|
| windows-rs | Keyboard hooks, key injection (Windows) | Microsoft official, replaces rdev |
| tao | Event loop | Tauri ecosystem, production-proven |
| tray-icon | System tray | Tauri ecosystem, companion to tao |
| proptest | Property-based testing | Industry standard |
| thiserror | Error handling | Industry standard, dtolnay maintained |

## Platform-Specific

| Library | Platform | Purpose |
|---------|----------|---------|
| windows-rs | Windows | WH_KEYBOARD_LL hook, SendInput |
| rdev | Linux | Development/testing only (not production) |

## Explicitly NOT Approved

| Library | Reason |
|---------|--------|
| active-win-pos-rs | Small community, use windows-rs directly |
| rdev (for Windows) | Use windows-rs for production Windows builds |

## Standard Library Usage

- `std::sync` (Arc, Mutex, AtomicBool) - Thread-safe state
- `std::time` (Instant, Duration) - Timeout handling
- `std::collections` (HashMap) - Character mapping tables

## Security Notes

- All keyboard interception must use official/trusted libraries
- No network dependencies allowed
- Minimal permission scope required
