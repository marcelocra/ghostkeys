//! Windows keyboard interceptor implementation
//!
//! Uses windows-rs (Microsoft official bindings) for keyboard hooks.
//! This is the primary production implementation.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::error::{GhostKeysError, Result};
use crate::interceptor::KeyboardInterceptor;
use crate::state::SharedState;

/// Windows keyboard interceptor using low-level keyboard hooks
pub struct WindowsInterceptor {
    running: Arc<AtomicBool>,
}

impl WindowsInterceptor {
    /// Create a new Windows interceptor
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Default for WindowsInterceptor {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyboardInterceptor for WindowsInterceptor {
    fn start(&mut self, _state: SharedState) -> Result<()> {
        if self.running.load(Ordering::SeqCst) {
            return Err(GhostKeysError::HookInstallError(
                "Interceptor already running".to_string(),
            ));
        }

        // TODO: Implement Windows keyboard hook using windows-rs
        // - SetWindowsHookEx with WH_KEYBOARD_LL
        // - Process key events in callback
        // - Use SendInput for key injection

        self.running.store(true, Ordering::SeqCst);
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        if !self.running.load(Ordering::SeqCst) {
            return Ok(());
        }

        // TODO: Implement hook release
        // - UnhookWindowsHookEx

        self.running.store(false, Ordering::SeqCst);
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

impl Drop for WindowsInterceptor {
    fn drop(&mut self) {
        // Ensure hook is released on drop
        let _ = self.stop();
    }
}
