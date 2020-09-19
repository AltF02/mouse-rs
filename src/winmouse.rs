// #[derive(Copy, Clone)]
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

    pub fn move_to(x: i32, y: i32) -> Result<(), Box<dyn std::error::Error>> {
        let user32 = libloading::Library::new("user32").unwrap();
        unsafe {
            let set_cursor_pos: libloading::Symbol<unsafe extern fn(x: i32, y: i32)> = user32.get(b"SetCursorPos")?;
            Ok(set_cursor_pos(x, y))
        }
    }

    pub fn press(button: &str) -> Result<(), Box<dyn std::error::Error>> {
        let user32 = libloading::Library::new("user32").unwrap();
        let (button, data) = WinMouse::translate_button(button);
        let code = win_translate_key((DOWN, button));
        WinMouse::mouse_event(code, 0, 0, data, 0)
    }

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

    pub fn wheel(delta: Option<i32>) -> Result<(), Box<dyn std::error::Error>> {
        let code = win_translate_key((WHEEL, VERTICAL));
        let actual_delta = delta.unwrap_or(1);
        WinMouse::mouse_event(code, 0, 0, actual_delta * 120, 0)
    }
}