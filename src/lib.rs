pub mod cursor;
pub mod display;
pub mod keyboard;

pub fn run() {
    let display = display::get_display();
    println!("display: {:?}", display);

    let window = display::get_window(display);
    println!("window: {:?}", window);

    keyboard::handle_keyboard_events(display, window);

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
