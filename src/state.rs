use crate::cursor;
use x11::xlib::{Display, Window, XKeyEvent};

#[derive(Debug)]
pub struct State {
    display: *mut Display,
    window: Window,
    log: bool,
    log_mouse: bool,

    activate_mapping: u32,
    activate2_mapping: u32,
    left_mapping: u32,
    down_mapping: u32,
    up_mapping: u32,
    right_mapping: u32,
    left_click_mapping: u32,
    middle_click_mapping: u32,
    right_click_mapping: u32,

    active: bool,
    act_pressed: bool,
    act2_pressed: bool,
    left_pressed: bool,
    down_pressed: bool,
    up_pressed: bool,
    right_pressed: bool,
    left_click_pressed: bool,
    middle_click_pressed: bool,
    right_click_pressed: bool,
}

impl State {
    pub fn new(display: *mut Display, window: Window) -> Self {
        Self {
            display,
            window,
            log: false,
            log_mouse: false,
            activate_mapping: 65,     // space
            activate2_mapping: 40,    // d
            left_mapping: 43,         // h
            down_mapping: 44,         // j
            up_mapping: 45,           // k
            right_mapping: 46,        // l
            left_click_mapping: 55,   // v
            middle_click_mapping: 56, // b
            right_click_mapping: 57,  // n
            act_pressed: false,
            act2_pressed: false,
            active: false,
            left_pressed: false,
            down_pressed: false,
            up_pressed: false,
            right_pressed: false,
            left_click_pressed: false,
            middle_click_pressed: false,
            right_click_pressed: false,
        }
    }

    fn check_active(&mut self) {
        if self.act_pressed && self.act2_pressed {
            self.active = true;
        } else {
            self.active = false;
        }
    }

    fn handle_act(&mut self, pressed: bool) {
        self.act_pressed = pressed;

        self.check_active();
    }

    fn handle_act2(&mut self, pressed: bool) {
        self.act2_pressed = pressed;

        self.check_active();
    }

    fn handle_left(&mut self, pressed: bool) {
        if self.active {
            self.left_pressed = pressed;
            cursor::move_pointer(self.display, -10, 0);
        }
    }

    fn handle_down(&mut self, pressed: bool) {
        if self.active {
            self.down_pressed = pressed;
            cursor::move_pointer(self.display, 0, 10);
        }
    }

    fn handle_up(&mut self, pressed: bool) {
        if self.active {
            self.up_pressed = pressed;
            cursor::move_pointer(self.display, 0, -10);
        }
    }

    fn handle_right(&mut self, pressed: bool) {
        if self.active {
            self.right_pressed = pressed;
            cursor::move_pointer(self.display, 10, 0);
        }
    }

    fn handle_left_click(&mut self, pressed: bool) {
        if self.active {
            self.left_click_pressed = pressed;
            cursor::left_click(self.display, pressed);
        }
    }

    fn handle_middle_click(&mut self, pressed: bool) {
        if self.active {
            self.middle_click_pressed = pressed;
            cursor::middle_click(self.display, pressed);
        }
    }

    fn handle_right_click(&mut self, pressed: bool) {
        if self.active {
            self.right_click_pressed = pressed;
            cursor::right_click(self.display, pressed);
        }
    }

    pub fn handle_key_press(&mut self, event: XKeyEvent) {
        let keycode = event.keycode;

        if self.log && self.log_mouse {
            let location = cursor::get_cursor_position(self.display, self.window);
            println!("pre cursor location: {:#?}", location);
        }

        match keycode {
            x if x == self.activate_mapping => self.handle_act(true),
            x if x == self.activate2_mapping => self.handle_act2(true),
            x if x == self.left_mapping => self.handle_left(true),
            x if x == self.down_mapping => self.handle_down(true),
            x if x == self.up_mapping => self.handle_up(true),
            x if x == self.right_mapping => self.handle_right(true),
            x if x == self.left_click_mapping => self.handle_left_click(true),
            x if x == self.middle_click_mapping => self.handle_middle_click(true),
            x if x == self.right_click_mapping => self.handle_right_click(true),
            x => println!("unmapped key pressed: {}", x),
        }

        if self.log {
            println!("{:#?}", self);
        }

        if self.log && self.log_mouse {
            let location = cursor::get_cursor_position(self.display, self.window);
            println!("pre cursor location: {:#?}", location);
        }
    }

    pub fn handle_key_release(&mut self, event: XKeyEvent) {
        let keycode = event.keycode;

        match keycode {
            x if x == self.activate_mapping => self.handle_act(false),
            x if x == self.activate2_mapping => self.handle_act2(false),
            x if x == self.left_mapping => self.handle_left(false),
            x if x == self.down_mapping => self.handle_down(false),
            x if x == self.up_mapping => self.handle_up(false),
            x if x == self.right_mapping => self.handle_right(false),
            x if x == self.left_click_mapping => self.handle_left_click(false),
            x if x == self.middle_click_mapping => self.handle_middle_click(false),
            x if x == self.right_click_mapping => self.handle_right_click(false),
            x => println!("unmapped key released: {}", x),
        }

        if self.log {
            println!("{:#?}", self);
        }
    }
}
