use crate::{sys, types::Point};

/// Struct for the mouse
///
/// This struct represents a mouse and doesn't hold any values
pub struct Mouse(sys::Mouse);

#[allow(unreachable_code, unused_variables)]
impl Mouse {
    /// This method creates a new mouse instance, must always be run before anything else
    pub fn new() -> Mouse {
        Mouse(sys::Mouse::new())
    }

    /// This method moves the mouse around
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mouse_rs::{types::keys::*, Mouse};
    ///
    /// fn move_mouse() {
    ///     let mouse = Mouse::new();
    ///     mouse.move_to(500, 500).expect("Unable to move mouse");
    /// }
    ///
    /// ```
    pub fn move_to(&self, x: i32, y: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.0.move_to(x, y)
    }

    /// This method presses the given button in
    ///
    /// *NOTE: This doesn't release the button so it will keep pressing*
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mouse_rs::{types::keys::*, Mouse};
    ///
    /// fn press_button() {
    ///     let mouse = Mouse::new();
    ///     mouse.press(LEFT).expect("Unable to press button"); // This will keep pressing
    /// }
    ///
    /// fn press_and_release_button() {
    ///     let mouse = Mouse::new();
    ///     mouse.press(RIGHT).expect("Unable to press button");
    ///     mouse.release(RIGHT).expect("Unable to release button"); // This will press the right mouse quickly
    /// }
    /// ```
    pub fn press<'a>(&self, button: &'a str) -> Result<(), Box<dyn std::error::Error + 'a>> {
        self.0.press(button)
    }

    /// This will release the button as noted above
    pub fn release<'a>(&self, button: &'a str) -> Result<(), Box<dyn std::error::Error + 'a>> {
        self.0.release(button)
    }

    /// This gets the current mouse position
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
    pub fn get_position(&self) -> Result<Point, Box<dyn std::error::Error>> {
        self.0.get_position()
    }

    /// This will scroll the mouse,
    ///
    /// For scrolling down use negative values, for scrolling up use positive values
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
    pub fn wheel(&self, delta: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.0.wheel(delta)
    }

    /// This is the exact same as wheel
    pub fn scroll(&self, delta: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.0.scroll(delta)
    }

    // Does the exact same thing as press and release combined, but into one function
    pub fn click(&self, button: &'a str) -> Result<(), Box<dyn std::error::Error + 'a>> {
        self.0.press(button).unwrap_or(());
        self.0.release(button)
    }
}
