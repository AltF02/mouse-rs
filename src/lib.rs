//! Mouse-rs is a rust library for controlling your mouse around
//!
//! View the examples for more complicated examples
//!
//! # Installation
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! mouse-rs = "0.1.0"
//! ```
//!
//! # Usage
//!
//! Inside your project run:
//!
//! ```no_run
//! use mouse_rs::winmouse::*;
//!
//! fn main() {
//!     WinMouse::move_to(50, 50);
//!     WinMouse::press("right");
//!     WinMouse::release("right");
//! }
//! ```

pub mod winmouse;
mod utils;
pub mod types;
