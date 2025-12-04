//! Platform-specific implementations
//!
//! This module contains platform-specific keyboard interceptor implementations.
//! - `windows.rs` - Windows implementation using windows-rs (primary target)
//! - `linux.rs` - Linux implementation using rdev (development/testing)

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;
