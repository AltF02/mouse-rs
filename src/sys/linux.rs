use crate::types::keys::Keys;
use crate::types::Point;
use std::error::Error;

use libc;

use libc::{c_int, c_void, c_char};
use std::{ptr};

type XDO = *const c_void;
type WINDOW = c_int;
type INTPTR = *mut c_int;

fn xdo_translate_key(key: &Keys) -> c_int {
    match key {
        Keys::LEFT => 1,
        Keys::WHEEL | Keys::MIDDLE => 2,
        Keys::RIGHT => 3,
        _ => panic!("Invalid key passed: {:?}", key)
    }
}

impl From<(c_int, c_int)> for Point {
    fn from(other: (c_int, c_int)) -> Point {
        Point {
            x: other.0 as _,
            y: other.1 as _,
        }
    }
}

pub struct Mouse {
    xdo: XDO,
    current_window: c_int
}

#[link(name = "xdo")]
extern "C" {
    fn xdo_new(display: *const c_char) -> XDO;
    fn xdo_free(xdo: XDO);

    fn xdo_move_mouse(xdo: XDO, x: c_int, y: c_int, screen: c_int) -> c_int;
    fn xdo_mouse_down(xdo: XDO, window: WINDOW, button: c_int);
    fn xdo_mouse_up(xdo: XDO, window: WINDOW, button: c_int);
    fn xdo_click_window(xdo: XDO, window: WINDOW, button: c_int);
    fn xdo_get_mouse_location(xdo: XDO, x: INTPTR, y: INTPTR, screen_num: INTPTR);
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
            xdo_mouse_down(self.xdo, self.current_window, xdo_translate_key(key))
        }
        Ok(())
    }

    pub fn release<'a>(&self, key: &'a Keys) -> Result<(), Box<dyn Error>> {
        unsafe {
            xdo_mouse_up(self.xdo, self.current_window, xdo_translate_key(key))
        }
        Ok(())
    }

    pub fn get_position(&self) -> Result<Point, Box<dyn Error>> {
        let pos: Point;
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            let mut _screen_num: c_int = 0;
            xdo_get_mouse_location(self.xdo, &mut x as INTPTR, &mut y as INTPTR, &mut _screen_num as INTPTR);
            pos = (x, y).into();
        }

        Ok(pos)
    }

    pub fn wheel(&self, mut delta: i32) -> Result<(), Box<dyn std::error::Error>> {
        let key = if delta < 0 { 4 } else { 5 };

        if delta < 0 {
            delta = -delta;
        }

        for _ in 0..delta {
            unsafe {
                xdo_click_window(self.xdo, self.current_window, key);
            }
        }
        Ok(())
    }
}

impl Drop for Mouse {
    fn drop(&mut self) {
        unsafe { 
            xdo_free(self.xdo); 
        }
    }
}
