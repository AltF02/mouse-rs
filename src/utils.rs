use crate::types::keys::*;
use crate::types::win_types::*;

pub(crate) fn win_translate_key(key: (&str, &str)) -> i32 {
    match key {
        (WHEEL, HORIZONTAL) => MOUSEEVENTF_HWHEEL,
        (WHEEL, VERTICAL) => MOUSEEVENTF_WHEEL,

        (DOWN, LEFT) => MOUSEEVENTF_LEFTDOWN,
        (UP, LEFT) => MOUSEEVENTF_LEFTUP,

        (DOWN, RIGHT) => MOUSEEVENTF_RIGHTDOWN,
        (UP, RIGHT) => MOUSEEVENTF_RIGHTUP,

        (DOWN, MIDDLE) => MOUSEEVENTF_MIDDLEDOWN,
        (UP, MIDDLE) => MOUSEEVENTF_MIDDLEUP,

        (DOWN, X) => MOUSEEVENTF_XDOWN,
        (UP, X) => MOUSEEVENTF_XUP,
        _ => panic!("Invalid parameter passed, please use constants from types::keys"),
    }
}
