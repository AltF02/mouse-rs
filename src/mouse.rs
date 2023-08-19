use crate::sys;
use crate::types::keys::Keys;
use crate::types::Point;

/// Represents the user's mouse pointer.
///
/// This is an abstraction of the internal [`sys::Mouse`] struct.
pub struct Mouse(sys::Mouse);

impl Mouse {
    #[must_use]
    /// Create a new [`Mouse`].
    pub const fn new() -> Self {
        Self(sys::Mouse::new())
    }

    /// Move the mouse to the position `(x, y)`, where the origin `(0, 0)` is the top-left of the screen.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// use mouse_rs::{types::keys::*, Mouse};
    ///
    /// fn move_mouse() {
    ///     let mouse = Mouse::new();
    ///     mouse.move_to(500, 500).expect("Unable to move mouse");
    /// }
    ///
    /// ```
    ///
    /// # Errors
    ///
    /// Fails if the mouse cannot move to the position.
    pub fn move_to(&self, x: i32, y: i32) -> Result<(), Box<dyn std::error::Error>> {
        sys::Mouse::move_to(x, y)
    }

    /// Press a button on the mouse and hold it.
    ///
    /// _NOTE: This doesn't release the button so it will be kept held until [`Self::release`] is called._
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// use mouse_rs::{types::keys::Keys, Mouse};
    ///
    /// fn press_button() {
    ///     let mouse = Mouse::new();
    ///     mouse.press(&Keys::LEFT).expect("Unable to press button"); // This will keep pressing
    /// }
    ///
    /// fn press_and_release_button() {
    ///     let mouse = Mouse::new();
    ///     mouse.press(&Keys::RIGHT).expect("Unable to press button");
    ///     mouse.release(&Keys::RIGHT).expect("Unable to release button"); // This will press the right mouse quickly
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Fails if the button cannot be pressed.
    pub fn press<'a>(&self, button: &'a Keys) -> Result<(), Box<dyn std::error::Error + 'a>> {
        sys::Mouse::press(button)
    }

    /// Release a button. Normally used after [`Self::press`].
    ///
    /// # Errors
    ///
    /// Fails if the button cannot be released.
    pub fn release<'a>(&self, button: &'a Keys) -> Result<(), Box<dyn std::error::Error + 'a>> {
        sys::Mouse::release(button)
    }

    /// Return the current mouse position.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use mouse_rs::Mouse;
    ///
    /// fn get_post() {
    ///     let mouse = Mouse::new();
    ///     let pos = mouse.get_position().unwrap();
    ///     println!("X = {}, Y = {}", pos.x, pos.y)
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Fails if the mouse position cannot be retrieved.
    pub fn get_position(&self) -> Result<Point, Box<dyn std::error::Error>> {
        sys::Mouse::get_position()
    }

    /// Scroll the mouse.
    ///
    /// For scrolling down use negative values, for scrolling up use positive values.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mouse_rs::{types::keys::*, Mouse};
    ///
    /// fn scroll_up() {
    ///     let mouse = Mouse::new();
    ///     mouse.wheel(1);
    /// }
    ///
    /// fn scroll_down() {
    ///     let mouse = Mouse::new();
    ///     mouse.wheel(-1);
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Fails if the mouse cannot be scrolled.
    pub fn wheel(&self, delta: i32) -> Result<(), Box<dyn std::error::Error>> {
        sys::Mouse::wheel(delta)
    }

    /// Alias for [`Self::wheel`].
    ///
    /// # Errors
    ///
    /// See [`Self::wheel`].
    pub fn scroll(&self, delta: i32) -> Result<(), Box<dyn std::error::Error>> {
        sys::Mouse::wheel(delta)
    }

    /// Click a mouse button by pressing then releasing.
    ///
    /// # Errors
    ///
    /// Fails if [`Self::press`] or [`Self::release`] fails.
    pub fn click<'a>(&self, button: &'a Keys) -> Result<(), Box<dyn std::error::Error + 'a>> {
        sys::Mouse::press(button)?;
        sys::Mouse::release(button)?;
        Ok(())
    }
}

impl Default for Mouse {
    fn default() -> Self {
        Self::new()
    }
}
