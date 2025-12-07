//! Windows keyboard interceptor implementation
//!
//! Uses windows-rs (Microsoft official bindings) for keyboard hooks.
//! This is the primary production implementation for Windows 11.

#![cfg(target_os = "windows")]

use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Mutex;

use windows::Win32::Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT,
    KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, VK_LSHIFT, VK_RSHIFT, VK_SHIFT,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, SetWindowsHookExW, UnhookWindowsHookEx, HHOOK, KBDLLHOOKSTRUCT,
    WH_KEYBOARD_LL, WM_KEYDOWN, WM_SYSKEYDOWN,
};

use crate::error::{GhostKeysError, Result};
use crate::interceptor::{KeyAction, KeyboardInterceptor};
use crate::mapper::{Mapper, VirtualKey};
use crate::state::SharedState;

// Thread-local storage for the mapper and hook handle
thread_local! {
    static MAPPER: RefCell<Option<Mapper>> = RefCell::new(None);
    static HOOK_HANDLE: RefCell<Option<HHOOK>> = RefCell::new(None);
}

// Global pause state
// This allows the hook to immediately return if the app is paused,
// minimizing overhead and preventing any interference.
pub static GLOBAL_PAUSED: AtomicBool = AtomicBool::new(false);

/// Sets the global pause state
pub fn set_paused(paused: bool) {
    GLOBAL_PAUSED.store(paused, Ordering::SeqCst);
}

// Global hook handle for panic handler access (separate from thread-local)
// We use isize to store the handle as HHOOK is not Send/Sync
static GLOBAL_HOOK_HANDLE: Mutex<Option<isize>> = Mutex::new(None);

/// Release the keyboard hook from the panic handler
/// This is called from the global panic hook to ensure the keyboard is freed
pub fn release_hook_on_panic() {
    if let Ok(mut handle) = GLOBAL_HOOK_HANDLE.lock() {
        if let Some(raw_handle) = handle.take() {
            unsafe {
                let hhook = HHOOK(raw_handle as *mut std::ffi::c_void);
                // We ignore the result here as we are panicking anyway
                let _ = UnhookWindowsHookEx(hhook);
            }
        }
    }
}

/// Convert Windows virtual key code to our VirtualKey enum
fn vk_to_virtual_key(vk: u32) -> VirtualKey {
    match vk {
        0xBA => VirtualKey::Semicolon,    // VK_OEM_1 (;:)
        0xDE => VirtualKey::Apostrophe,   // VK_OEM_7 ('")
        0xDB => VirtualKey::LeftBracket,  // VK_OEM_4 ([{)
        0xDD => VirtualKey::RightBracket, // VK_OEM_6 (]})
        0xDC => VirtualKey::Backslash,    // VK_OEM_5 (\|)
        0xBF => VirtualKey::Slash,        // VK_OEM_2 (/?)
        0x20 => VirtualKey::Space,        // VK_SPACE
        0x41..=0x5A => VirtualKey::Char((vk as u8) as char), // A-Z
        _ => VirtualKey::Other,
    }
}

/// Check if shift is currently pressed
fn is_shift_pressed() -> bool {
    unsafe {
        GetAsyncKeyState(VK_SHIFT.0 as i32) < 0
            || GetAsyncKeyState(VK_LSHIFT.0 as i32) < 0
            || GetAsyncKeyState(VK_RSHIFT.0 as i32) < 0
    }
}

/// Inject a Unicode character using SendInput
fn inject_char(c: char) {
    let mut inputs: Vec<INPUT> = Vec::new();

    // Key down
    inputs.push(INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY(0),
                wScan: c as u16,
                dwFlags: KEYEVENTF_UNICODE,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    });

    // Key up
    inputs.push(INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY(0),
                wScan: c as u16,
                dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    });

    unsafe {
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }
}

/// Inject multiple Unicode characters
fn inject_chars(chars: &[char]) {
    for &c in chars {
        inject_char(c);
    }
}

/// Low-level keyboard procedure callback
unsafe extern "system" fn low_level_keyboard_proc(
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // If code < 0, pass to next hook
    if code < 0 {
        return CallNextHookEx(None, code, wparam, lparam);
    }

    // Check if paused - do this early to minimize overhead
    if GLOBAL_PAUSED.load(Ordering::SeqCst) {
        return CallNextHookEx(None, code, wparam, lparam);
    }

    // Get key info from lparam
    let kb_struct = &*(lparam.0 as *const KBDLLHOOKSTRUCT);

    // Check if the event was injected (LLKHF_INJECTED is bit 4)
    // This prevents infinite recursion when we inject keys
    if (kb_struct.flags.0 & 0x10) != 0 {
         return CallNextHookEx(None, code, wparam, lparam);
    }

    // Only process key down events
    let msg = wparam.0 as u32;
    if msg != WM_KEYDOWN && msg != WM_SYSKEYDOWN {
        return CallNextHookEx(None, code, wparam, lparam);
    }

    let vk_code = kb_struct.vkCode;

    // Convert to our VirtualKey
    let virtual_key = vk_to_virtual_key(vk_code);

    // Skip keys we don't handle
    if matches!(virtual_key, VirtualKey::Other) {
        return CallNextHookEx(None, code, wparam, lparam);
    }

    // Check shift state
    let shift = is_shift_pressed();

    // Process through mapper
    let action = MAPPER.with(|mapper| {
        if let Some(ref mut m) = *mapper.borrow_mut() {
            m.process_key(virtual_key, shift)
        } else {
            KeyAction::Pass
        }
    });

    // Handle the action
    match action {
        KeyAction::Pass => CallNextHookEx(None, code, wparam, lparam),
        KeyAction::Suppress => LRESULT(1), // Block the key
        KeyAction::Replace(c) => {
            inject_char(c);
            LRESULT(1) // Block original key
        }
        KeyAction::ReplaceMultiple(chars) => {
            inject_chars(&chars);
            LRESULT(1) // Block original key
        }
    }
}

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

    /// Install the low-level keyboard hook
    fn install_hook(&self) -> Result<HHOOK> {
        unsafe {
            let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(low_level_keyboard_proc), HINSTANCE::default(), 0)
                .map_err(|e| GhostKeysError::HookInstallError(format!("SetWindowsHookExW failed: {}", e)))?;
            Ok(hook)
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

        // Initialize the mapper
        MAPPER.with(|mapper| {
            *mapper.borrow_mut() = Some(Mapper::new());
        });

        // Install the hook
        let hook = self.install_hook()?;
        
        // Store in thread-local
        HOOK_HANDLE.with(|h| {
            *h.borrow_mut() = Some(hook);
        });
        
        // Store raw handle in global for panic handler
        if let Ok(mut global) = GLOBAL_HOOK_HANDLE.lock() {
            *global = Some(hook.0 as isize);
        }

        self.running.store(true, Ordering::SeqCst);
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        if !self.running.load(Ordering::SeqCst) {
            return Ok(());
        }

        // Unhook
        HOOK_HANDLE.with(|h| {
            if let Some(hook) = h.borrow_mut().take() {
                unsafe {
                    let _ = UnhookWindowsHookEx(hook);
                }
            }
        });
        
        // Clear global handle
        if let Ok(mut global) = GLOBAL_HOOK_HANDLE.lock() {
            *global = None;
        }

        // Clear mapper
        MAPPER.with(|mapper| {
            *mapper.borrow_mut() = None;
        });

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
