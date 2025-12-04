//! GhostKeys - ABNT2 keyboard layout emulation on US keyboards
//!
//! This application intercepts keyboard input and translates US key positions
//! to ABNT2 characters, allowing users with ABNT2 muscle memory to type
//! Portuguese naturally on US hardware.

mod error;
mod interceptor;
mod mapper;
mod platform;
mod state;

use state::SharedState;

fn main() {
    println!("GhostKeys - ABNT2 keyboard layout emulation");
    println!("Platform: {}", std::env::consts::OS);

    // Initialize shared state
    let _state = SharedState::new();

    // TODO: Initialize system tray
    // TODO: Start keyboard interceptor
    // TODO: Run event loop

    println!("GhostKeys initialized. Press Ctrl+C to exit.");
}
