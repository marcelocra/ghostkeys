//! Linux keyboard interceptor implementation
//!
//! Uses rdev for keyboard hooks on X11/Wayland.
//! This implementation is for development and testing only, NOT for production.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::error::{GhostKeysError, Result};
use crate::interceptor::KeyboardInterceptor;
use crate::state::SharedState;

/// Linux keyboard interceptor using rdev
///
/// NOTE: This is for development/testing only. Production builds target Windows.
pub struct LinuxInterceptor {
    running: Arc<AtomicBool>,
}

impl LinuxInterceptor {
    /// Create a new Linux interceptor
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Default for LinuxInterceptor {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyboardInterceptor for LinuxInterceptor {
    fn start(&mut self, _state: SharedState) -> Result<()> {
        if self.running.load(Ordering::SeqCst) {
            return Err(GhostKeysError::HookInstallError(
                "Interceptor already running".to_string(),
            ));
        }

        // TODO: Implement Linux keyboard hook using rdev
        // - rdev::listen for key events
        // - rdev::simulate for key injection

        self.running.store(true, Ordering::SeqCst);
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        if !self.running.load(Ordering::SeqCst) {
            return Ok(());
        }

        // TODO: Implement hook release

        self.running.store(false, Ordering::SeqCst);
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

impl Drop for LinuxInterceptor {
    fn drop(&mut self) {
        // Ensure hook is released on drop
        let _ = self.stop();
    }
}
