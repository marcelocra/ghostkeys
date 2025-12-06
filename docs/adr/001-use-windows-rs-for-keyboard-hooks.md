# ADR 001: Use windows-rs for Keyboard Hooks

## Status

Accepted

## Context

GhostKeys needs to intercept keyboard input at a low level on Windows to remap keys from US layout to ABNT2 layout. We evaluated several options:

1. **rdev** - Cross-platform keyboard library
2. **windows-rs** - Official Microsoft Rust bindings
3. **winapi** - Legacy Windows bindings (unmaintained)

## Decision

We will use **windows-rs** for Windows keyboard interception.

## Rationale

- **Official Microsoft Support**: windows-rs is the official Rust bindings maintained by Microsoft
- **Type Safety**: Provides safe Rust abstractions over Win32 APIs
- **Active Maintenance**: Regular updates and security patches
- **Production Ready**: Used by major projects (Tauri, etc.)
- **Low-Level Access**: Direct access to `SetWindowsHookExW` and `SendInput` APIs
- **Security**: Better than rdev for production Windows builds (per security research)

## Consequences

### Positive

- Safe, idiomatic Rust code
- Direct control over keyboard hooks
- Minimal dependencies
- Strong type safety prevents common Win32 errors

### Negative

- Windows-only (but that's our target platform)
- Requires understanding Win32 concepts
- Larger binary size than pure C bindings

## Alternatives Considered

- **rdev**: Cross-platform but less control, security concerns for production
- **winapi**: Unmaintained, unsafe APIs

## References

- [windows-rs GitHub](https://github.com/microsoft/windows-rs)
- [Win32 Keyboard Hooks Documentation](https://learn.microsoft.com/en-us/windows/win32/winmsg/about-hooks)
