#![allow(dead_code)]
use crate::types::{keys::*, Point};
use libloading::Library;
use winapi::shared::windef::POINT;

const MOUSEEVENTF_ABSOLUTE: i32 = 0x8000;
const MOUSEEVENTF_MOVE: i32 = 0x1;
const MOUSEEVENTF_WHEEL: i32 = 0x800;
const MOUSEEVENTF_HWHEEL: i32 = 0x1000;
const MOUSEEVENTF_LEFTDOWN: i32 = 0x2;
const MOUSEEVENTF_LEFTUP: i32 = 0x4;
const MOUSEEVENTF_RIGHTDOWN: i32 = 0x8;
const MOUSEEVENTF_RIGHTUP: i32 = 0x10;
const MOUSEEVENTF_MIDDLEDOWN: i32 = 0x20;
const MOUSEEVENTF_MIDDLEUP: i32 = 0x40;
const MOUSEEVENTF_XDOWN: i32 = 0x0080;
const MOUSEEVENTF_XUP: i32 = 0x0100;

fn win_translate_key(key: (&str, &str)) -> i32 {
    match key {
        (WHEEL, HORIZONTAL) => MOUSEEVENTF_HWHEEL,
        (WHEEL, VERTICAL) => MOUSEEVENTF_WHEEL,

        (DOWN, LEFT) => MOUSEEVENTF_LEFTDOWN,
        (UP, LEFT) => MOUSEEVENTF_LEFTUP,

        (DOWN, RIGHT) => MOUSEEVENTF_RIGHTDOWN,
        (UP, RIGHT) => MOUSEEVENTF_RIGHTUP,

        (DOWN, MIDDLE) => MOUSEEVENTF_MIDDLEDOWN,
        (UP, MIDDLE) => MOUSEEVENTF_MIDDLEUP,

        (DOWN, X) => MOUSEEVENTF_XDOWN,
        (UP, X) => MOUSEEVENTF_XUP,
        _ => panic!("Invalid parameter passed, please use constants from types::keys"),
    }
}

impl From<POINT> for Point {
    fn from(other: POINT) -> Point {
        Point {
            x: other.x as _,
            y: other.y as _,
        }
    }
}

pub struct Mouse {
    user32: Library,
}

#[allow(unreachable_code, unused_variables)]
impl Mouse {
    fn translate_button(button: &str) -> (&str, i32) {
        return match button {
            X => (X, 0x10000),
            X2 => (X2, 0x20000),
            _ => (button, 0),
        };
    }

    fn mouse_event(
        &self,
        dw_flags: i32,
        dx: i32,
        dy: i32,
        dw_data: i32,
        dw_extra_info: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let mouse_event: libloading::Symbol<
                unsafe extern "C" fn(
                    dw_flags: i32,
                    dx: i32,
                    dy: i32,
                    dw_data: i32,
                    dw_extra_info: i32,
                ),
            > = self.user32.get(b"mouse_event")?;
            Ok(mouse_event(dw_flags, dx, dy, dw_data, dw_extra_info))
        }
    }

    pub fn new() -> Mouse {
        Mouse {
            user32: libloading::Library::new("user32").unwrap(),
        }
    }

    pub fn move_to(&self, x: i32, y: i32) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let set_cursor_pos: libloading::Symbol<unsafe extern "C" fn(x: i32, y: i32)> =
                self.user32.get(b"SetCursorPos")?;
            Ok(set_cursor_pos(x, y))
        }
    }

    pub fn press(&self, button: &str) -> Result<(), Box<dyn std::error::Error>> {
        let (button, data) = Mouse::translate_button(button);
        let code = win_translate_key((DOWN, button));
        self.mouse_event(code, 0, 0, data, 0)
    }

    pub fn release(&self, button: &str) -> Result<(), Box<dyn std::error::Error>> {
        let (button, data) = Mouse::translate_button(button);
        let code = win_translate_key((UP, button));
        self.mouse_event(code, 0, 0, data, 0)
    }

    pub fn get_position(&self) -> Result<Point, Box<dyn std::error::Error>> {
        let mut pos: POINT = POINT { x: 0, y: 0 };
        unsafe {
            let get_cursor_pos: libloading::Symbol<unsafe extern "C" fn(lp_point: &POINT) -> bool> =
                self.user32.get(b"GetCursorPos")?;
            get_cursor_pos(&mut pos);
            Ok(pos.into())
        }
    }

    pub fn wheel(&self, delta: i32) -> Result<(), Box<dyn std::error::Error>> {
        let code = win_translate_key((WHEEL, VERTICAL));
        self.mouse_event(code, 0, 0, delta * 120, 0)
    }

    pub fn scroll(&self, delta: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.wheel(delta)
    }
}
