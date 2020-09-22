use crate::types::keys::Keys;
use crate::types::Point;
use std::error::Error;

use x11::xlib::{XWarpPointer, Display, AspectRatio, XDefaultRootWindow, XOpenDisplay};
use x11::xtest::XTestFakeButtonEvent;

pub struct Mouse;

impl Mouse {
    pub fn new() -> Mouse {
        Mouse
    }

    pub fn move_to(&self, x: i32, y: i32) -> Result<(), Box<dyn Error>> {
        unsafe {
            let dpy = XOpenDisplay(&mut 1);
            println!("{:?}", time::Instant::now());
            // XTestFakeButtonEvent(dpy, 1, 1, );
            println!("{:?}", dpy);
        }
        Ok(())
        // unimplemented!()
    }

    pub fn press<'a>(&self, button: &'a Keys) -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }

    pub fn release<'a>(&self, button: &'a Keys) -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }

    pub fn get_position(&self) -> Result<Point, Box<dyn Error>> {
        unimplemented!()
    }

    pub fn wheel(&self, delta: i32) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
}
