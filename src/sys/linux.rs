use std::error::Error;
use crate::types::keys::Keys;
use crate::types::Point;

pub struct Mouse;

impl Mouse {
    pub fn new() -> Mouse {
        Mouse
    }

    pub fn move_to(&self, x: i32, y: i32) -> Result<(), Box<dyn Error>> {
        unimplemented!()
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