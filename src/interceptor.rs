//! Keyboard interceptor trait and types
//!
//! This module defines the platform-agnostic interface for keyboard interception.
//! Platform-specific implementations are in the `platform` module.

use crate::error::Result;
use crate::state::SharedState;

/// Action to take after processing a keystroke
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyAction {
    /// Allow the keystroke through unmodified
    Pass,
    /// Block the keystroke (e.g., dead key pending)
    Suppress,
    /// Suppress original and inject a replacement character
    Replace(char),
    /// Suppress original and inject multiple characters
    ReplaceMultiple(Vec<char>),
}

/// Platform-agnostic keyboard interceptor trait
///
/// Implementations of this trait handle platform-specific keyboard hook
/// installation, event processing, and key injection.
pub trait KeyboardInterceptor: Send {
    /// Start intercepting keyboard events
    ///
    /// This method should install the keyboard hook and begin processing events.
    /// It typically runs in a separate thread.
    fn start(&mut self, state: SharedState) -> Result<()>;

    /// Stop intercepting keyboard events
    ///
    /// This method should release the keyboard hook and clean up resources.
    fn stop(&mut self) -> Result<()>;

    /// Check if the interceptor is currently running
    fn is_running(&self) -> bool;
}

/// Create a platform-specific keyboard interceptor
///
/// Returns the appropriate interceptor implementation for the current platform.
#[cfg(target_os = "windows")]
pub fn create_interceptor() -> Box<dyn KeyboardInterceptor> {
    Box::new(crate::platform::windows::WindowsInterceptor::new())
}

#[cfg(target_os = "linux")]
pub fn create_interceptor() -> Box<dyn KeyboardInterceptor> {
    Box::new(crate::platform::linux::LinuxInterceptor::new())
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
pub fn create_interceptor() -> Box<dyn KeyboardInterceptor> {
    compile_error!("Unsupported platform. GhostKeys supports Windows and Linux only.")
}
