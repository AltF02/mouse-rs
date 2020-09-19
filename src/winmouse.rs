


use winapi::shared::windef::POINT;
use crate::types::keys::*;
use crate::utils::win_translate_key;
use libloading::Library;

/// Struct for the mouse in windows
///
/// This struct represents a windows mouse and doesn't hold any values
pub struct WinMouse {
    user32: Library
}

#[allow(unreachable_code, unused_variables)]
impl WinMouse {
    fn translate_button(button: &str) -> (&str, i32) {
        return match button {
            X => (X, 0x10000),
            X2 => (X2, 0x20000),
            _ => (button, 0)
        }
    }

    fn mouse_event(&self, dw_flags: i32, dx: i32, dy: i32, dw_data: i32, dw_extra_info: i32) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let mouse_event: libloading::Symbol<unsafe extern fn(dw_flags: i32, dx: i32, dy: i32, dw_data: i32, dw_extra_info: i32)> = self.user32.get(b"mouse_event")?;
            Ok(mouse_event(dw_flags, dx, dy, dw_data, dw_extra_info))
        }
    }

    /// This method creates a new mouse instance, must always be run before anything else
    pub fn new() -> WinMouse {
        WinMouse {
            user32: libloading::Library::new("user32").unwrap()
        }
    }

    /// This method moves the windows mouse around
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mouse_rs::{
    ///     winmouse::WinMouse,
    ///     types::keys::*
    /// };
    ///
    /// fn move_mouse() {
    ///     let mouse = WinMouse::new();
    ///     mouse.move_to(500, 500).expect("Unable to move mouse");
    /// }
    ///
    /// ```
    pub fn move_to(&self, x: i32, y: i32) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let set_cursor_pos: libloading::Symbol<unsafe extern fn(x: i32, y: i32)> = self.user32.get(b"SetCursorPos")?;
            Ok(set_cursor_pos(x, y))
        }
    }


    /// This method presses the given button in
    ///
    /// *NOTE: This doesn't release the button so it will keep pressing*
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mouse_rs::{
    ///     winmouse::WinMouse,
    ///     types::keys::*
    /// };
    ///
    /// fn press_button() {
    ///     let mouse = WinMouse::new();
    ///     mouse.press(LEFT).expect("Unable to press button"); // This will keep pressing
    /// }
    ///
    /// fn press_and_release_button() {
    ///     let mouse = WinMouse::new();
    ///     mouse.press(RIGHT).expect("Unable to press button");
    ///     mouse.release(RIGHT).expect("Unable to release button"); // This will press the right mouse quickly
    /// }
    /// ```
    pub fn press(&self, button: &str) -> Result<(), Box<dyn std::error::Error>> {
        let (button, data) = WinMouse::translate_button(button);
        let code = win_translate_key((DOWN, button));
        self.mouse_event(code, 0, 0, data, 0)
    }

    /// This will release the button as noted above
    pub fn release(&self, button: &str) -> Result<(), Box<dyn std::error::Error>> {
        let (button, data) = WinMouse::translate_button(button);
        let code = win_translate_key((UP, button));
        self.mouse_event(code, 0, 0, data, 0)
    }

    fn get_position(&self) -> Result<POINT, Box<dyn std::error::Error>> {
        // TODO Make this work
        unsafe {
            let get_cursor_pos: libloading::Symbol<unsafe extern fn() -> POINT> = self.user32.get(b"GetCursorPos")?;
            Ok(get_cursor_pos())
        }
    }

    /// This will scroll the mouse,
    ///
    /// For scrolling down use negative values, for scrolling up use positive values
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mouse_rs::{
    ///     winmouse::WinMouse,
    ///     types::keys::*
    /// };
    ///
    /// fn scroll_up() {
    ///     let mouse = WinMouse::new();
    ///     mouse.wheel(1);
    /// }
    ///
    /// fn scroll_down() {
    ///     let mouse = WinMouse::new();
    ///     mouse.wheel(-1);
    /// }
    /// ```
    pub fn wheel(&self, delta: i32) -> Result<(), Box<dyn std::error::Error>> {
        let code = win_translate_key((WHEEL, VERTICAL));
        self.mouse_event(code, 0, 0, delta * 120, 0)
    }

    /// This is the exact same as wheel
    pub fn scroll(&self, delta: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.wheel(delta)
    }
}