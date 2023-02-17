use ps2_mouse::{Mouse, MouseState};
use spin::Mutex;

use crate::kprintln;

pub static MOUSE: Mutex<Mouse> = Mutex::new(Mouse::new());

pub fn init() {
    let mut mouse = MOUSE.lock();
    mouse.init().unwrap();
    mouse.set_on_complete(on_complete);
}

fn on_complete(mouse_state: MouseState) {
    kprintln!("{:?}", mouse_state);
}
