//! GhostKeys - ABNT2 keyboard layout emulation on US keyboards
//!
//! This library provides the core functionality for intercepting keyboard input
//! and translating US key positions to ABNT2 characters.

pub mod error;
pub mod interceptor;
pub mod mapper;
pub mod platform;
pub mod state;

// Re-export commonly used types
pub use error::{GhostKeysError, Result};
pub use interceptor::{KeyAction, KeyboardInterceptor};
pub use mapper::{AccentType, Mapper, MapperState, VirtualKey};
pub use state::{OperationMode, SharedState};
