pub mod cursor;
pub mod display;
pub mod keyboard;
pub mod state;

pub fn run() {
    let display = display::get_display();
    let window = display::get_window(display);

    keyboard::handle_keyboard_events(display, window, false);

    display::close_display(display);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_runs() {
        run();
    }
}
