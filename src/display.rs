use std::{os::raw::c_ulong, ptr::null};
use x11::xlib::{self, Display};

type Screen = i32;

type Window = c_ulong;

pub fn get_display() -> *mut Display {
    let display_name = null();
    unsafe {
        let display_ptr = xlib::XOpenDisplay(display_name);

        display_ptr
    }
}

pub fn close_display(display: *mut Display) {
    unsafe {
        xlib::XCloseDisplay(display);
    }
}

pub fn get_screen(display: *mut Display) -> Screen {
    unsafe {
        let default_screen = xlib::XDefaultScreen(display);

        default_screen
    }
}

pub fn get_window(display: *mut Display) -> Window {
    unsafe {
        let window = xlib::XDefaultRootWindow(display);

        window
    }
}
