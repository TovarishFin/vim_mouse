use std::os::raw::{c_int, c_ulong};
use std::{convert::TryInto, mem::MaybeUninit};
use x11::xlib::{
    self, Display, FocusChangeMask, FocusOut, KeyPress, KeyPressMask, KeyRelease, KeyReleaseMask,
    MotionNotify, PointerMotionMask, PointerRoot, XEvent,
};

use crate::cursor;

type Window = c_ulong;

pub fn handle_keyboard_events(display: *mut Display, root_window: Window) {
    unsafe {
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
                    println!("focus out");
                    let mut fw = *focused_window.as_mut_ptr();
                    if fw != root_window {
                        xlib::XSelectInput(display, fw, mask);
                    }

                    xlib::XGetInputFocus(display, focused_window.as_mut_ptr(), revert.as_mut_ptr());

                    if fw == PointerRoot.try_into().unwrap() {
                        fw = root_window;
                    }

                    xlib::XSelectInput(display, fw, mask);
                }
                KeyPress => {
                    let keycode = event.key.keycode;
                    // h = 43
                    // j = 44
                    // k = 45
                    // l = 46
                    // alt = 64
                    // d = 40

                    let location = cursor::get_cursor_position(display, root_window);
                    println!("pre cursor location: {:#?}", location);

                    match keycode {
                        43 => cursor::move_pointer(display, -10, 0),
                        44 => cursor::move_pointer(display, 0, 10),
                        45 => cursor::move_pointer(display, 0, -10),
                        46 => cursor::move_pointer(display, 10, 0),
                        _ => (),
                    }

                    let location = cursor::get_cursor_position(display, root_window);
                    println!("post cursor location: {:#?}", location);
                }
                KeyRelease => {
                    let keycode = event.key.keycode;
                    println!("keycode {} released", keycode);
                }
                MotionNotify => println!("mouse moved"),
                _ => println!("other event"),
            }
        }
    }
}
