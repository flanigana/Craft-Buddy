use std::{ thread::sleep, time::Duration, env };
use mki_fork::{ Keyboard, register_hotkey };
use clipboard::{ ClipboardContext, ClipboardProvider };

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    register_hotkey(&[Keyboard::LeftControl, Keyboard::Z], || {
        copy();
        print_clipboard();
    });

    loop {}
}

fn copy() {
    Keyboard::LeftControl.press();
    Keyboard::C.press();
    sleep(Duration::from_millis(10));
    Keyboard::C.release();
}

fn print_clipboard() {
    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
    println!("{:?}", clipboard.get_contents());
}
