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

use interceptor::create_interceptor;
use state::SharedState;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use tao::event::{Event, StartCause};
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    TrayIconBuilder,
};

/// Creates a simple 32x32 colored icon as RGBA bytes
fn create_icon_rgba(active: bool) -> Vec<u8> {
    let mut rgba = Vec::with_capacity(32 * 32 * 4);
    for y in 0..32 {
        for x in 0..32 {
            let is_border = x < 2 || x >= 30 || y < 2 || y >= 30;
            if is_border {
                // Border color
                if active {
                    rgba.extend_from_slice(&[0, 100, 0, 255]); // Dark green
                } else {
                    rgba.extend_from_slice(&[100, 100, 0, 255]); // Dark yellow
                }
            } else {
                // Center color
                if active {
                    rgba.extend_from_slice(&[50, 205, 50, 255]); // Bright green
                } else {
                    rgba.extend_from_slice(&[255, 200, 0, 255]); // Yellow (paused)
                }
            }
        }
    }
    rgba
}

fn main() {
    println!("GhostKeys - ABNT2 keyboard layout emulation");
    println!("Platform: {}", std::env::consts::OS);

    // Initialize shared state
    let state = SharedState::new();
    let state_for_hook = state.clone();
    let is_active = Arc::new(AtomicBool::new(true));

    // Spawn keyboard interceptor thread
    let _hook_thread = thread::spawn(move || {
        let mut interceptor = create_interceptor();

        if let Err(e) = interceptor.start(state_for_hook) {
            eprintln!("Failed to start keyboard interceptor: {:?}", e);
            return;
        }

        println!("Keyboard interceptor started successfully!");

        // Keep thread alive - on Windows the hook needs a message loop
        // The main thread's event loop handles this, but we park here
        // to keep the interceptor alive
        #[cfg(target_os = "windows")]
        {
            use windows::Win32::UI::WindowsAndMessaging::{GetMessageW, MSG};
            unsafe {
                let mut msg = MSG::default();
                while GetMessageW(&mut msg, None, 0, 0).as_bool() {
                    // Process messages to keep hook alive
                }
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            // On Linux, just park the thread
            loop {
                thread::park();
            }
        }
    });

    // Build event loop
    let event_loop = EventLoopBuilder::new().build();

    // Create tray menu
    let menu = Menu::new();
    let status_item = MenuItem::new("GhostKeys: Active", false, None);
    let pause_item = MenuItem::new("Pause", true, None);
    let exit_item = MenuItem::new("Exit", true, None);

    let _ = menu.append(&status_item);
    let _ = menu.append(&pause_item);
    let _ = menu.append(&exit_item);

    // Create icon from RGBA data
    let icon_rgba = create_icon_rgba(true);
    let icon = tray_icon::Icon::from_rgba(icon_rgba, 32, 32).expect("Failed to create icon");

    // Build tray icon
    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("GhostKeys - ABNT2 Emulation (Active)")
        .with_icon(icon)
        .build()
        .expect("Failed to create tray icon");

    println!("System tray initialized. Right-click the tray icon for options.");

    // Store menu item IDs for event handling
    let pause_id = pause_item.id().clone();
    let exit_id = exit_item.id().clone();

    // Run event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                println!("GhostKeys is running...");
            }
            _ => {}
        }

        // Handle menu events
        if let Ok(menu_event) = MenuEvent::receiver().try_recv() {
            if menu_event.id == pause_id {
                let currently_active = is_active.load(Ordering::SeqCst);
                is_active.store(!currently_active, Ordering::SeqCst);

                if currently_active {
                    println!("GhostKeys paused");
                    status_item.set_text("GhostKeys: Paused");
                    pause_item.set_text("Resume");
                    let _ = state.set_mode(state::OperationMode::Passthrough);

                    // Update icon to yellow (paused)
                    let paused_icon = create_icon_rgba(false);
                    if let Ok(icon) = tray_icon::Icon::from_rgba(paused_icon, 32, 32) {
                        let _ = tray_icon.set_icon(Some(icon));
                        let _ = tray_icon.set_tooltip(Some("GhostKeys - ABNT2 Emulation (Paused)"));
                    }
                } else {
                    println!("GhostKeys resumed");
                    status_item.set_text("GhostKeys: Active");
                    pause_item.set_text("Pause");
                    let _ = state.set_mode(state::OperationMode::Active);

                    // Update icon to green (active)
                    let active_icon = create_icon_rgba(true);
                    if let Ok(icon) = tray_icon::Icon::from_rgba(active_icon, 32, 32) {
                        let _ = tray_icon.set_icon(Some(icon));
                        let _ = tray_icon.set_tooltip(Some("GhostKeys - ABNT2 Emulation (Active)"));
                    }
                }
            } else if menu_event.id == exit_id {
                println!("Exiting GhostKeys...");
                *control_flow = ControlFlow::Exit;
            }
        }
    });
}
