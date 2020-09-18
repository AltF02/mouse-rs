pub mod winmouse;

#[cfg(test)]
mod tests {
    use crate::winmouse::*;

    #[test]
    fn move_to() {
        WinMouse::move_to(500, 500).expect("Something went wrong");
        WinMouse::press("right").expect("Something went wrong");
        WinMouse::release("right").expect("Something went wrong");
        // println!("{:?}", WinMouse::get_position());
    }
}
