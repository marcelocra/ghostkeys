//! Shared state types for GhostKeys

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::error::{GhostKeysError, Result};

/// Operation mode for GhostKeys
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OperationMode {
    /// Active mode: intercept and remap keyboard input
    #[default]
    Active,
    /// Passthrough mode: allow all keystrokes through unmodified
    Passthrough,
}

/// Application state shared between threads
#[derive(Debug)]
pub struct AppState {
    /// Current operation mode
    pub mode: OperationMode,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            mode: OperationMode::Active,
        }
    }
}

/// Thread-safe wrapper for shared application state
#[derive(Debug, Clone)]
pub struct SharedState {
    inner: Arc<Mutex<AppState>>,
    exit_flag: Arc<AtomicBool>,
}

impl SharedState {
    /// Create a new shared state with default values
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(AppState::default())),
            exit_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Get the current operation mode
    pub fn get_mode(&self) -> Result<OperationMode> {
        self.inner
            .lock()
            .map(|state| state.mode)
            .map_err(|_| GhostKeysError::StateLockPoisoned)
    }

    /// Set the operation mode
    pub fn set_mode(&self, mode: OperationMode) -> Result<()> {
        self.inner
            .lock()
            .map(|mut state| state.mode = mode)
            .map_err(|_| GhostKeysError::StateLockPoisoned)
    }

    /// Toggle between Active and Passthrough modes
    pub fn toggle_mode(&self) -> Result<OperationMode> {
        let mut state = self
            .inner
            .lock()
            .map_err(|_| GhostKeysError::StateLockPoisoned)?;

        state.mode = match state.mode {
            OperationMode::Active => OperationMode::Passthrough,
            OperationMode::Passthrough => OperationMode::Active,
        };

        Ok(state.mode)
    }

    /// Signal that the application should exit
    pub fn signal_exit(&self) {
        self.exit_flag.store(true, Ordering::SeqCst);
    }

    /// Check if the application should exit
    pub fn should_exit(&self) -> bool {
        self.exit_flag.load(Ordering::SeqCst)
    }
}

impl Default for SharedState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_mode_is_active() {
        let state = SharedState::new();
        assert_eq!(state.get_mode().unwrap(), OperationMode::Active);
    }

    #[test]
    fn test_toggle_mode() {
        let state = SharedState::new();

        // Active -> Passthrough
        let mode = state.toggle_mode().unwrap();
        assert_eq!(mode, OperationMode::Passthrough);

        // Passthrough -> Active
        let mode = state.toggle_mode().unwrap();
        assert_eq!(mode, OperationMode::Active);
    }

    #[test]
    fn test_exit_flag() {
        let state = SharedState::new();
        assert!(!state.should_exit());

        state.signal_exit();
        assert!(state.should_exit());
    }

    #[test]
    fn test_shared_state_is_clone() {
        let state1 = SharedState::new();
        let state2 = state1.clone();

        state1.set_mode(OperationMode::Passthrough).unwrap();
        assert_eq!(state2.get_mode().unwrap(), OperationMode::Passthrough);
    }
}
