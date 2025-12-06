# 0001 - Use windows-rs for Keyboard Hooks

**Status:** Accepted

**Date:** 2025-12-05

**Deciders:** Marcelo Almeida (repository owner)

## Context

GhostKeys needs to intercept keyboard input at a low level on Windows to remap keys from US layout to ABNT2 layout. This requires:

- Installing a global keyboard hook (`WH_KEYBOARD_LL`)
- Processing key events before they reach applications
- Injecting replacement characters via `SendInput`
- Handling the Windows message loop

Several Rust libraries exist for keyboard interaction, each with different trade-offs for security, maintenance, and control.

## Decision

Use **windows-rs** (Microsoft's official Rust bindings) for all Windows keyboard interception.

```rust
use windows::Win32::UI::WindowsAndMessaging::{
    SetWindowsHookExW, CallNextHookEx, UnhookWindowsHookEx,
    WH_KEYBOARD_LL, KBDLLHOOKSTRUCT,
};
use windows::Win32::UI::Input::KeyboardAndMouse::{SendInput, INPUT};
```

## Alternatives Considered

### Option 1: rdev (cross-platform library)

**Rejected** because:
- Less control over low-level Windows APIs
- Security concerns for production Windows builds (per library research)
- Abstractions hide important details for keyboard hooks
- Acceptable for Linux development, not Windows production

### Option 2: winapi (legacy bindings)

**Rejected** because:
- Unmaintained (last significant update years ago)
- All APIs are unsafe with no type safety
- No official Microsoft support
- Being replaced by windows-rs in the ecosystem

### Option 3: Raw FFI bindings

**Rejected** because:
- Maximum maintenance burden
- No type safety
- Reinventing what windows-rs already provides
- Higher risk of memory safety issues

## Consequences

### Positive

- **Official support** - Microsoft maintains windows-rs with regular updates
- **Type safety** - Rust types prevent common Win32 errors (wrong handle types, etc.)
- **Production ready** - Used by major projects (Tauri, etc.)
- **Direct API access** - Full control over `SetWindowsHookExW` and `SendInput`
- **Security updates** - Microsoft patches security issues promptly

### Negative

- **Windows-only** - Cannot use for Linux implementation (but that's expected)
- **Learning curve** - Requires understanding Win32 concepts
- **Larger binary** - More code than raw FFI (acceptable trade-off)

### Neutral

- Requires separate Linux implementation using rdev
- Win32 documentation applies directly to windows-rs usage

## Notes

- Library research documented in `docs/library-research.md`
- rdev is still used for Linux development/testing (see `src/platform/linux.rs`)
- This decision aligns with "security first" principle in AGENTS.md

**References:**
- [windows-rs GitHub](https://github.com/microsoft/windows-rs)
- [Win32 Keyboard Hooks](https://learn.microsoft.com/en-us/windows/win32/winmsg/about-hooks)
- [SendInput Documentation](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendinput)
