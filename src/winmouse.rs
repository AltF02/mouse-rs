
/// Struct for the mouse in windows
///
/// This struct represents a windows mouse and doesn't hold any values
pub struct WinMouse;

use winapi::shared::windef::POINT;
use crate::types::keys::*;
use crate::utils::win_translate_key;

#[allow(unreachable_code, unused_variables)]
impl WinMouse {
    fn translate_button(button: &str) -> (&str, i32) {
        return match button {
            X => (X, 0x10000),
            X2 => (X2, 0x20000),
            _ => (button, 0)
        }
    }

    fn mouse_event(dw_flags: i32, dx: i32, dy: i32, dw_data: i32, dw_extra_info: i32) -> Result<(), Box<dyn std::error::Error>> {
        let user32 = libloading::Library::new("user32").unwrap();
        unsafe {
            let mouse_event: libloading::Symbol<unsafe extern fn(dw_flags: i32, dx: i32, dy: i32, dw_data: i32, dw_extra_info: i32)> = user32.get(b"mouse_event")?;
            Ok(mouse_event(dw_flags, dx, dy, dw_data, dw_extra_info))
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
    ///     WinMouse::move_to(500, 500).expect("Unable to move mouse")
    /// }
    ///
    /// ```
    pub fn move_to(x: i32, y: i32) -> Result<(), Box<dyn std::error::Error>> {
        let user32 = libloading::Library::new("user32").unwrap();
        unsafe {
            let set_cursor_pos: libloading::Symbol<unsafe extern fn(x: i32, y: i32)> = user32.get(b"SetCursorPos")?;
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
    ///     WinMouse::press(LEFT).expect("Unable to press button"); // This will keep pressing
    /// }
    ///
    /// fn press_and_release_button() {
    ///     WinMouse::press(RIGHT).expect("Unable to press button");
    ///     WinMouse::release(RIGHT).expect("Unable to release button"); // This will press the right mouse quickly
    /// }
    /// ```
    pub fn press(button: &str) -> Result<(), Box<dyn std::error::Error>> {
        let user32 = libloading::Library::new("user32").unwrap();
        let (button, data) = WinMouse::translate_button(button);
        let code = win_translate_key((DOWN, button));
        WinMouse::mouse_event(code, 0, 0, data, 0)
    }

    /// This will release the button as noted above
    pub fn release(button: &str) -> Result<(), Box<dyn std::error::Error>> {
        let user32 = libloading::Library::new("user32").unwrap();
        let (button, data) = WinMouse::translate_button(button);
        let code = win_translate_key((UP, button));
        let user32 = libloading::Library::new("user32").unwrap();
        WinMouse::mouse_event(code, 0, 0, data, 0)
    }

    fn get_position() -> Result<POINT, Box<dyn std::error::Error>> {
        // TODO Make this work
        let user32 = libloading::Library::new("user32").unwrap();
        unsafe {
            let get_cursor_pos: libloading::Symbol<unsafe extern fn() -> POINT> = user32.get(b"GetCursorPos")?;
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
    ///     WinMouse::wheel(2);
    /// }
    ///
    /// fn scroll_down() {
    ///     WinMouse::wheel(-2);
    /// }
    /// ```
    pub fn wheel(delta: i32) -> Result<(), Box<dyn std::error::Error>> {
        let code = win_translate_key((WHEEL, VERTICAL));
        WinMouse::mouse_event(code, 0, 0, delta * 120, 0)
    }

    /// This is the exact same as wheel
    pub fn scroll(delta: i32) -> Result<(), Box<dyn std::error::Error>> {
        WinMouse::wheel(delta)
    }
}