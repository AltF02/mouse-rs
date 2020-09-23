use crate::types::keys::Keys;
use crate::types::Point;
use std::error::Error;

use libc;

use libc::{c_int, c_void, c_char};
use std::{ptr};

type XDO = *const c_void;
type WINDOW = c_int;

fn xdo_translate_key(key: &Keys) -> c_int {
    match key {
        Keys::LEFT => 1,
        Keys::MIDDLE => 2,
        Keys::RIGHT => 3,
        _ => panic!("Invalid key passed")
    }
}

pub struct Mouse {
    xdo: XDO,
    current_window: c_int
}

#[link(name = "xdo")]
extern "C" {
    fn xdo_new(display: *const c_char) -> XDO;

    fn xdo_move_mouse(xdo: XDO, x: c_int, y: c_int, screen: c_int) -> c_int;
    fn xdo_mouse_down(xdo: XDO, window: WINDOW, button: c_int);
    fn xdo_mouse_up(xdo: XDO, window: WINDOW, button: c_int);
}

impl Mouse {
    pub fn new() -> Self {
        Mouse {
            xdo: unsafe { xdo_new(ptr::null()) },
            current_window: 0
        }
    }

    pub fn move_to(&self, x: i32, y: i32) -> Result<(), Box<dyn Error>> {
        unsafe {
            xdo_move_mouse(self.xdo, x as c_int, y as c_int, 0);
        }
        Ok(())
    }

    pub fn press<'a>(&self, key: &'a Keys) -> Result<(), Box<dyn Error>> {
        unsafe {
            xdo_mouse_down(self.xdo, self.current_window, xdo_translate_key(button))
        }
        Ok(())
    }

    pub fn release<'a>(&self, key: &'a Keys) -> Result<(), Box<dyn Error>> {
        unsafe {
            xdo_mouse_up(self.xdo, self.current_window, xdo_translate_key(button))
        }
        Ok(())
    }

    pub fn get_position(&self) -> Result<Point, Box<dyn Error>> {

        unimplemented!()
    }

    pub fn wheel(&self, delta: i32) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
}
