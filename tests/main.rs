#[cfg(test)]
mod mouse {
    use mouse_rs::{types::keys::Keys, Mouse};

    #[test]
    fn move_and_press() {
        let mouse = Mouse::new();
        mouse.move_to(500, 500).expect("Unable to move mouse");
        mouse.press(&Keys::RIGHT).expect("Unable to press button");
        mouse.release(&Keys::RIGHT).expect("Unable to release button");
        mouse.click(&Keys::WHEEL).expect("Unable to click button");
    }

    #[test]
    fn scroll_wheel() {
        let mouse = Mouse::new();
        mouse.wheel(1).expect("Unable to scroll mouse");
    }

    #[test]
    fn press_button() {
        let mouse = Mouse::new();
        mouse.press(&Keys::MIDDLE).expect("Unable to press button");
        mouse.release(&Keys::MIDDLE).expect("Unable to release button");
    }

    // #[test]
    // fn print_post() {
    //     let mouse = Mouse::new();
    //     let pos = mouse.get_position().unwrap();
    //     println!("{:?}, {:?}", pos.y, pos.x);
    // }
}
