use crate::cursor;
use crate::keyboard;
use std::time::{Duration, Instant};
use x11::xlib::{Display, Window};

enum Direction {
    Left,
    Down,
    Up,
    Right,
}

enum MouseButton {
    Left,
    Middle,
    Right,
}

enum SpeedMod {
    Slow,
    Fast,
}

#[derive(Debug)]
pub struct State {
    display: *mut Display,
    window: Window,

    rate: i32,
    activate_mapping: u32,
    fast_mapping: u32,
    slow_mapping: u32,
    scroll_mapping: u32,
    left_mapping: u32,
    down_mapping: u32,
    up_mapping: u32,
    right_mapping: u32,
    left_click_mapping: u32,
    left_click_toggle_mapping: u32,
    middle_click_mapping: u32,
    right_click_mapping: u32,
    cached_keys: [Option<u32>; 2],

    sleep_activate_start: Instant,
    sleep_cache_start: Instant,
    sleep_duration: Duration,
    active_key_threshold: Duration,
    active_last_pressed: Instant,
    left_click_toggle: bool,
    active: bool,
    fast_pressed: bool,
    slow_pressed: bool,
    scroll_pressed: bool,
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
        let new_self = Self {
            display,
            window,
            rate: 5,
            activate_mapping: 40,          // d
            fast_mapping: 41,              // f
            slow_mapping: 42,              // g
            scroll_mapping: 39,            // s
            left_mapping: 43,              // h
            down_mapping: 44,              // j
            up_mapping: 45,                // k
            right_mapping: 46,             // l
            left_click_mapping: 55,        // v
            left_click_toggle_mapping: 65, // space
            middle_click_mapping: 56,      // b
            right_click_mapping: 57,       // n
            cached_keys: [None, None],
            sleep_activate_start: Instant::now(),
            sleep_cache_start: Instant::now(),
            sleep_duration: Duration::from_millis(5),
            active_key_threshold: Duration::from_millis(500),
            active_last_pressed: Instant::now(),
            left_click_toggle: false,
            active: false,
            fast_pressed: false,
            slow_pressed: false,
            scroll_pressed: false,
            left_pressed: false,
            down_pressed: false,
            up_pressed: false,
            right_pressed: false,
            left_click_pressed: false,
            middle_click_pressed: false,
            right_click_pressed: false,
        };

        keyboard::grab_key(
            new_self.display,
            new_self.window,
            new_self.activate_mapping as i32,
        );

        new_self
    }

    pub fn activate_mapping(&self) -> u32 {
        self.activate_mapping
    }

    fn handle_speed(&mut self, speed: SpeedMod, pressed: bool) {
        if !self.active {
            return;
        }

        match speed {
            SpeedMod::Fast => {
                self.fast_pressed = pressed;
            }
            SpeedMod::Slow => {
                self.slow_pressed = pressed;
            }
        }
    }

    fn handle_scroll(&mut self, pressed: bool) {
        if !self.active {
            return;
        }

        self.scroll_pressed = pressed;
    }

    fn get_rate(&self) -> i32 {
        match self {
            Self {
                fast_pressed: true, ..
            } => self.rate * 2,
            Self {
                slow_pressed: true, ..
            } => self.rate / 2,
            _ => self.rate,
        }
    }

    fn handle_direction(&mut self, direction: Direction, pressed: bool) {
        if !self.active {
            return;
        }

        match direction {
            Direction::Left => {
                self.left_pressed = pressed;
            }
            Direction::Down => {
                self.down_pressed = pressed;
            }
            Direction::Up => {
                self.up_pressed = pressed;
            }
            Direction::Right => {
                self.right_pressed = pressed;
            }
        }

        match self {
            /*
             * diagonal directions
             */
            Self {
                left_pressed: true,
                down_pressed: true,
                ..
            } => {
                cursor::move_pointer(self.display, -self.get_rate(), self.get_rate());
            }
            Self {
                left_pressed: true,
                up_pressed: true,
                ..
            } => {
                cursor::move_pointer(self.display, -self.get_rate(), -self.get_rate());
            }
            Self {
                right_pressed: true,
                down_pressed: true,
                ..
            } => {
                cursor::move_pointer(self.display, self.get_rate(), self.get_rate());
            }
            Self {
                right_pressed: true,
                up_pressed: true,
                ..
            } => {
                cursor::move_pointer(self.display, self.get_rate(), -self.get_rate());
            }
            /*
             * scroll directions
             */
            Self {
                scroll_pressed: true,
                left_pressed: true,
                ..
            } => {
                cursor::scroll_left(self.display, pressed);
            }
            Self {
                scroll_pressed: true,
                down_pressed: true,
                ..
            } => {
                cursor::scroll_down(self.display, pressed);
            }
            Self {
                scroll_pressed: true,
                up_pressed: true,
                ..
            } => {
                cursor::scroll_up(self.display, pressed);
            }
            Self {
                scroll_pressed: true,
                right_pressed: true,
                ..
            } => {
                cursor::scroll_right(self.display, pressed);
            }
            /*
             * pointer directions
             */
            Self {
                left_pressed: true, ..
            } => {
                cursor::move_pointer(self.display, -self.get_rate(), 0);
            }
            Self {
                down_pressed: true, ..
            } => {
                cursor::move_pointer(self.display, 0, self.get_rate());
            }
            Self {
                up_pressed: true, ..
            } => {
                cursor::move_pointer(self.display, 0, -self.get_rate());
            }
            Self {
                right_pressed: true,
                ..
            } => {
                cursor::move_pointer(self.display, self.get_rate(), 0);
            }
            Self { .. } => (),
        }
    }

    fn handle_click(&mut self, button: MouseButton, pressed: bool) {
        if !self.active {
            return;
        }

        match button {
            MouseButton::Left => {
                self.left_click_pressed = pressed;
                cursor::left_click(self.display, pressed);
            }
            MouseButton::Middle => {
                self.middle_click_pressed = pressed;
                cursor::middle_click(self.display, pressed);
            }
            MouseButton::Right => {
                self.right_click_pressed = pressed;
                cursor::right_click(self.display, pressed);
            }
        }
    }

    fn handle_left_click_toggle(&mut self) {
        self.left_click_toggle = !self.left_click_toggle;

        cursor::left_click(self.display, self.left_click_toggle);
    }

    fn handle_other(&mut self, keycode: u32, pressed: bool) {
        if !self.active {
            return;
        }

        keyboard::simulate_key(self.display, keycode, pressed);
    }

    fn sleep_activate(&mut self) {
        self.sleep_activate_start = Instant::now();
    }

    fn check_activate_sleep(&self) -> bool {
        self.sleep_activate_start.elapsed() < self.sleep_duration
    }

    fn start_activate_threshold(&mut self) {
        self.active_last_pressed = Instant::now();
    }

    fn check_activate_threshold(&self) -> bool {
        self.active_last_pressed.elapsed() > self.active_key_threshold
    }

    fn sleep_cache(&mut self) {
        self.sleep_cache_start = Instant::now();
    }

    fn check_cache_sleep(&self) -> bool {
        self.sleep_cache_start.elapsed() < self.sleep_duration
    }

    fn handle_active(&mut self, pressed: bool) {
        if pressed {
            keyboard::grab_keyboard(self.display, self.window);

            self.start_activate_threshold();
        } else {
            keyboard::ungrab_keyboard(self.display);
            let activate_was_held = self.check_activate_threshold();
            if !activate_was_held {
                keyboard::ungrab_key(self.display, self.window, self.activate_mapping as i32);

                self.sleep_activate();
                self.sleep_cache();
                if let Some(keycode) = self.cached_keys[0].take() {
                    keyboard::simulate_key(self.display, self.activate_mapping, true);
                    keyboard::simulate_key(self.display, self.activate_mapping, false);

                    keyboard::simulate_key(self.display, keycode, false);
                    keyboard::simulate_key(self.display, keycode, true);
                    keyboard::simulate_key(self.display, keycode, false);

                    if let Some(keycode) = self.cached_keys[1].take() {
                        keyboard::simulate_key(self.display, keycode, false);
                        keyboard::simulate_key(self.display, keycode, true);
                        keyboard::simulate_key(self.display, keycode, false);
                    }
                } else {
                    keyboard::simulate_key(self.display, self.activate_mapping, true);
                    keyboard::simulate_key(self.display, self.activate_mapping, false);
                }

                keyboard::grab_key(self.display, self.window, self.activate_mapping as i32);
            }
        }

        self.active = pressed;
    }

    fn cache_key(&mut self, keycode: u32) {
        if let None = self.cached_keys[0] {
            self.cached_keys[0] = Some(keycode);
        } else {
            self.cached_keys[1] = Some(keycode);
        }
    }

    pub fn handle_key_press(&mut self, keycode: u32) {
        if keycode != self.activate_mapping || self.check_cache_sleep() {
            self.cache_key(keycode);
        }

        if self.check_activate_sleep() {
            return;
        }

        match keycode {
            x if x == self.activate_mapping => self.handle_active(true),
            x if x == self.scroll_mapping => self.handle_scroll(true),
            x if x == self.fast_mapping => self.handle_speed(SpeedMod::Fast, true),
            x if x == self.slow_mapping => self.handle_speed(SpeedMod::Slow, true),
            x if x == self.left_mapping => self.handle_direction(Direction::Left, true),
            x if x == self.down_mapping => self.handle_direction(Direction::Down, true),
            x if x == self.up_mapping => self.handle_direction(Direction::Up, true),
            x if x == self.right_mapping => self.handle_direction(Direction::Right, true),
            x if x == self.left_click_mapping => self.handle_click(MouseButton::Left, true),
            x if x == self.middle_click_mapping => self.handle_click(MouseButton::Middle, true),
            x if x == self.right_click_mapping => self.handle_click(MouseButton::Right, true),
            x if x == self.left_click_toggle_mapping => self.handle_left_click_toggle(),
            x => self.handle_other(x, true),
        }
    }

    pub fn handle_key_release(&mut self, keycode: u32) {
        if self.check_activate_sleep() {
            return;
        }

        match keycode {
            x if x == self.activate_mapping => self.handle_active(false),
            x if x == self.scroll_mapping => self.handle_scroll(false),
            x if x == self.fast_mapping => self.handle_speed(SpeedMod::Fast, false),
            x if x == self.slow_mapping => self.handle_speed(SpeedMod::Slow, false),
            x if x == self.left_mapping => self.handle_direction(Direction::Left, false),
            x if x == self.down_mapping => self.handle_direction(Direction::Down, false),
            x if x == self.up_mapping => self.handle_direction(Direction::Up, false),
            x if x == self.right_mapping => self.handle_direction(Direction::Right, false),
            x if x == self.left_click_mapping => self.handle_click(MouseButton::Left, false),
            x if x == self.middle_click_mapping => self.handle_click(MouseButton::Middle, false),
            x if x == self.right_click_mapping => self.handle_click(MouseButton::Right, false),
            x => self.handle_other(x, false),
        }
    }
}

impl Drop for State {
    fn drop(&mut self) {
        keyboard::ungrab_key(self.display, self.window, self.activate_mapping as i32);
    }
}
