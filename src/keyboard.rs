use crate::state;
use std::mem::MaybeUninit;
use std::os::raw::c_int;
use x11::xlib::{
    self, Display, FocusChangeMask, FocusOut, KeyPress, KeyPressMask, KeyRelease, KeyReleaseMask,
    MotionNotify, PointerMotionMask, Window, XEvent,
};

pub fn handle_keyboard_events(display: *mut Display, root_window: Window, log_mouse: bool) {
    unsafe {
        let mut focused_window = MaybeUninit::<Window>::uninit();
        let mut revert = MaybeUninit::<c_int>::uninit();
        let mut event = MaybeUninit::<XEvent>::uninit();
        let mask = KeyPressMask | KeyReleaseMask | PointerMotionMask | FocusChangeMask;

        xlib::XGetInputFocus(display, focused_window.as_mut_ptr(), revert.as_mut_ptr());
        xlib::XSelectInput(display, *focused_window.as_ptr(), mask);

        let mut state = state::State::new(display, root_window);

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
                    state.handle_key_press(event.key);
                }
                KeyRelease => {
                    state.handle_key_release(event.key);
                }
                MotionNotify if log_mouse => println!("mouse moved"),
                _ => (),
            }
        }
    }
}
