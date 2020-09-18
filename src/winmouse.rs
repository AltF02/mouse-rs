use libloading::Library;

// #[derive(Copy, Clone)]
pub struct WinMouse;

#[allow(unreachable_code)]
impl WinMouse {
    pub fn move_to(x: i32, y: i32) -> Result<(), Box<dyn std::error::Error>> {
        let user32 = libloading::Library::new("user32").unwrap();
        unsafe {
            let set_cursor_pos: libloading::Symbol<unsafe extern fn(x: i32, y: i32)> = user32.get(b"SetCursorPos")?;
            Ok(set_cursor_pos(x, y))
        }
    }

    pub fn press(button: &str) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Translate parameter
        let user32 = libloading::Library::new("user32").unwrap();
        unsafe {
            let mouse_event: libloading::Symbol<unsafe extern fn(dw_flags: i32, dx: i32, dy: i32, dw_data: i32, dw_extra_info: i32)> = user32.get(b"mouse_event")?;
            Ok(mouse_event(0x8, 0, 0, 0, 0))
        }
    }

    pub fn release(button: &str) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Translate parameter
        let user32 = libloading::Library::new("user32").unwrap();
        unsafe {
            let mouse_event: libloading::Symbol<unsafe extern fn(dw_flags: i32, dx: i32, dy: i32, dw_data: i32, dw_extra_info: i32)> = user32.get(b"mouse_event")?;
            Ok(mouse_event(0x10, 0, 0, 0, 0))
        }
    }

    pub fn get_position() -> Result<(i32, i32), Box<dyn std::error::Error>> {
        unimplemented!();
        // TODO Make this work
        let user32 = libloading::Library::new("user32").unwrap();
        unsafe {
            let get_cursor_pos: libloading::Symbol<unsafe extern fn() -> (i32, i32)> = user32.get(b"GetCursorPos")?;
            println!("{:?}", get_cursor_pos());
            Ok(get_cursor_pos())
        }
    }

    pub fn wheel(delta: Option<i32>) {
        unimplemented!();
        delta.unwrap_or(1);
    }
}