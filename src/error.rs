//! Error types for GhostKeys

use thiserror::Error;

/// Main error type for GhostKeys operations
#[derive(Debug, Error)]
pub enum GhostKeysError {
    /// Failed to install the keyboard hook
    #[error("Failed to install keyboard hook: {0}")]
    HookInstallError(String),

    /// Failed to release the keyboard hook
    #[error("Failed to release keyboard hook: {0}")]
    HookReleaseError(String),

    /// System tray icon error
    #[error("Tray icon error: {0}")]
    TrayError(String),

    /// Shared state lock was poisoned
    #[error("State lock poisoned")]
    StateLockPoisoned,

    /// Key injection failed
    #[error("Failed to inject key: {0}")]
    KeyInjectionError(String),
}

/// Result type alias for GhostKeys operations
pub type Result<T> = std::result::Result<T, GhostKeysError>;
