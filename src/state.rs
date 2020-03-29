use crate::cursor;
use x11::xlib::{Display, Window, XKeyEvent};

#[derive(Debug)]
pub struct State {
    display: *mut Display,
    window: Window,
    log_mouse: bool,
    activate_mapping: u32,
    activate2_mapping: u32,
    left_mapping: u32,
    down_mapping: u32,
    up_mapping: u32,
    right_mapping: u32,
    act_pressed: bool,
    act2_pressed: bool,
    active: bool,
    left_pressed: bool,
    down_pressed: bool,
    up_pressed: bool,
    right_pressed: bool,
}

impl State {
    pub fn new(display: *mut Display, window: Window) -> Self {
        Self {
            display,
            window,
            log_mouse: false,
            activate_mapping: 65,  // space
            activate2_mapping: 40, // d
            left_mapping: 43,      // h
            down_mapping: 44,      // j
            up_mapping: 45,        // k
            right_mapping: 46,     // l
            act_pressed: false,
            act2_pressed: false,
            active: false,
            left_pressed: false,
            down_pressed: false,
            up_pressed: false,
            right_pressed: false,
        }
    }

    pub fn check_active(&mut self) {
        if self.act_pressed && self.act2_pressed {
            self.active = true;
        } else {
            self.active = false;
        }
    }

    pub fn handle_act_changed(&mut self, pressed: bool) {
        self.act_pressed = pressed;

        self.check_active();
    }

    pub fn handle_act2_changed(&mut self, pressed: bool) {
        self.act2_pressed = pressed;

        self.check_active();
    }

    pub fn handle_left_changed(&mut self, pressed: bool) {
        if self.active {
            self.left_pressed = pressed;
            cursor::move_pointer(self.display, -10, 0);
        }
    }

    pub fn handle_down_changed(&mut self, pressed: bool) {
        if self.active {
            self.down_pressed = pressed;
            cursor::move_pointer(self.display, 0, 10);
        }
    }

    pub fn handle_up_changed(&mut self, pressed: bool) {
        if self.active {
            self.up_pressed = pressed;
            cursor::move_pointer(self.display, 0, -10);
        }
    }

    pub fn handle_right_changed(&mut self, pressed: bool) {
        if self.active {
            self.right_pressed = pressed;
            cursor::move_pointer(self.display, 10, 0);
        }
    }

    pub fn handle_key_press(&mut self, event: XKeyEvent) {
        let keycode = event.keycode;

        if self.log_mouse {
            let location = cursor::get_cursor_position(self.display, self.window);
            println!("pre cursor location: {:#?}", location);
        }

        match keycode {
            x if x == self.activate_mapping => self.handle_act_changed(true),
            x if x == self.activate2_mapping => self.handle_act2_changed(true),
            x if x == self.left_mapping => self.handle_left_changed(true),
            x if x == self.down_mapping => self.handle_down_changed(true),
            x if x == self.up_mapping => self.handle_up_changed(true),
            x if x == self.right_mapping => self.handle_right_changed(true),
            _ => println!("unmapped key pressed"),
        }

        println!("{:#?}", self);

        if self.log_mouse {
            let location = cursor::get_cursor_position(self.display, self.window);
            println!("pre cursor location: {:#?}", location);
        }
    }

    pub fn handle_key_release(&mut self, event: XKeyEvent) {
        let keycode = event.keycode;
        println!("key {}", keycode);

        match keycode {
            x if x == self.activate_mapping => self.handle_act_changed(false),
            x if x == self.activate2_mapping => self.handle_act2_changed(false),
            x if x == self.left_mapping => self.handle_left_changed(false),
            x if x == self.down_mapping => self.handle_down_changed(false),
            x if x == self.up_mapping => self.handle_up_changed(false),
            x if x == self.right_mapping => self.handle_right_changed(false),
            _ => println!("unmapped key released"),
        }

        println!("{:#?}", self);
    }
}
