use std::{
    fmt::Debug,
    os::raw::{c_int, c_uint, c_ulong},
};
use x11::{
    xlib::{self, Display},
    xtest,
};

pub struct XQueryPointerResponse {
    pub root_return: c_ulong,
    pub child_return: c_ulong,
    pub root_x_return: c_int,
    pub root_y_return: c_int,
    pub win_x_return: c_int,
    pub win_y_return: c_int,
    pub mask_return: c_uint,
}

impl Default for XQueryPointerResponse {
    fn default() -> Self {
        Self {
            root_return: 0,
            child_return: 0,
            root_x_return: 0,
            root_y_return: 0,
            win_x_return: 0,
            win_y_return: 0,
            mask_return: 0,
        }
    }
}

#[derive(Debug)]
pub struct CursorCoordinates(u32, u32);

pub fn get_cursor_position(display: *mut Display, window: c_ulong) -> CursorCoordinates {
    let mut response = XQueryPointerResponse::default();
    unsafe {
        xlib::XQueryPointer(
            display,
            window,
            &mut response.root_return,
            &mut response.child_return,
            &mut response.root_x_return,
            &mut response.root_y_return,
            &mut response.win_x_return,
            &mut response.win_y_return,
            &mut response.mask_return,
        );
    }

    CursorCoordinates(response.root_x_return as u32, response.root_y_return as u32)
}

pub fn move_pointer(display: *mut Display, x: c_int, y: c_int) {
    unsafe {
        // xlib::XWarpPointer(display, 0, 0, 0, 0, 0, 0, x, y);
        x11::xtest::XTestFakeRelativeMotionEvent(display, x, y, 0);
    }
}

pub fn left_click(display: *mut Display, pressed: bool) {
    let pressed = pressed as i32;
    unsafe {
        xtest::XTestFakeButtonEvent(display, 1, pressed, 0);
    }
}

pub fn middle_click(display: *mut Display, pressed: bool) {
    let pressed = pressed as i32;
    unsafe {
        xtest::XTestFakeButtonEvent(display, 2, pressed, 0);
    }
}

pub fn right_click(display: *mut Display, pressed: bool) {
    let pressed = pressed as i32;
    unsafe {
        xtest::XTestFakeButtonEvent(display, 3, pressed, 0);
    }
}
