use crate::state;
use state::State;
use std::mem::MaybeUninit;
use std::os::raw::c_int;
use x11::{
    xlib::{
        self, Display, FocusChangeMask, FocusOut, GrabModeAsync, KeyPress, KeyPressMask,
        KeyRelease, KeyReleaseMask, MotionNotify, PointerMotionMask, Window, XEvent,
    },
    xtest,
};

pub fn grab_key(display: *mut Display, window: Window, keycode: i32) {
    unsafe {
        xlib::XGrabKey(display, keycode, 0, window, 1, GrabModeAsync, GrabModeAsync);
    }
}

pub fn ungrab_key(display: *mut Display, window: Window, keycode: i32) {
    unsafe {
        xlib::XUngrabKey(display, keycode, 0, window);
    }
}

pub fn grab_keyboard(display: *mut Display, window: Window) {
    unsafe {
        xlib::XGrabKeyboard(display, window, 1, GrabModeAsync, GrabModeAsync, 0);
    }
}

pub fn ungrab_keyboard(display: *mut Display) {
    unsafe {
        xlib::XUngrabKeyboard(display, 0);
    }
}

pub fn simulate_key(display: *mut Display, keycode: u32, pressed: bool) {
    unsafe {
        xtest::XTestFakeKeyEvent(display, keycode, pressed as i32, 0);
    }
}

pub fn handle_keyboard_events(display: *mut Display, root_window: Window, log_mouse: bool) {
    unsafe {
        let mut state = State::new(display, root_window);

        let mut focused_window = MaybeUninit::<Window>::uninit();
        let mut revert = MaybeUninit::<c_int>::uninit();
        let mut event = MaybeUninit::<XEvent>::uninit();
        let mask = KeyPressMask | KeyReleaseMask | PointerMotionMask | FocusChangeMask;

        xlib::XGetInputFocus(display, focused_window.as_mut_ptr(), revert.as_mut_ptr());
        xlib::XSelectInput(display, *focused_window.as_ptr(), mask);

        loop {
            xlib::XNextEvent(display, event.as_mut_ptr());

            let event = *event.as_ptr();
            let event_type = event.get_type();

            #[allow(non_upper_case_globals)]
            match event_type {
                FocusOut => {
                    xlib::XGetInputFocus(display, focused_window.as_mut_ptr(), revert.as_mut_ptr());
                    xlib::XSelectInput(display, *focused_window.as_ptr(), mask);
                }
                KeyPress => {
                    state.handle_key_press(event.key.keycode);
                }
                KeyRelease => {
                    state.handle_key_release(event.key.keycode);
                }
                MotionNotify if log_mouse => println!("mouse moved"),
                _ => (),
            }
        }
    }
}
