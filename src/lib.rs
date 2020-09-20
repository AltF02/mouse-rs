#![feature(in_band_lifetimes)]
//! ![Rust](https://github.com/DankDumpster/mouse-rs/workflows/Rust/badge.svg)
//! # Mouse-rs
//!
//! Mouse-rs is a rust library for controlling your mouse from a rust program, without having to go into your kernel yourself.
//!
//! This project was loosely based on the python [mouse](https://github.com/boppreh/mouse/) library.
//! Currently it only supports windows based machines but I plan on adding unix later.
//!
//! ## Installation
//! Add mouse-rs to your cargo.toml
//!
//! ```toml
//! [dependencies]
//! mouse-rs = "0.3"
//! ```
//!
//! ## Example
//! This is a simple example that moves your mouse to a position on screen and presses the left button.
//!
//! ```no_run
//! use mouse_rs::{types::keys::*, Mouse};
//!
//! fn move_and_press() {
//!     let mouse = Mouse::new();
//!     mouse.move_to(500, 500).expect("Unable to move mouse");
//!     mouse.press(RIGHT).expect("Unable to press button");
//!     mouse.release(RIGHT).expect("Unable to release button");
//! }
//! ```

mod mouse;
mod sys;
pub mod types;

pub use mouse::Mouse;
