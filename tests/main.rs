#[cfg(test)]
#[allow(unused_must_use)]
mod winmouse {
    use mouse_rs::winmouse::*;
    use mouse_rs::types::event_types::{RIGHT, MIDDLE};

    #[test]
    fn move_and_press() {
        WinMouse::move_to(500, 500).expect("Something went wrong");
        WinMouse::press(RIGHT).expect("Something went wrong");
        WinMouse::release(RIGHT).expect("Something went wrong");
        // println!("{:?}", WinMouse::get_position());
    }

    #[test]
    fn scroll_wheel() {
        WinMouse::wheel(Some(1));
    }

    #[test]
    fn press_button() {
        WinMouse::press(MIDDLE);
        WinMouse::release(MIDDLE);
    }

    #[test]
    fn print_position() {
        let pos = WinMouse::get_position().unwrap();
        println!("{:?}, {:?}", pos.x, pos.y)
    }
}