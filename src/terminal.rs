use lazy_static::lazy_static;
use spin::Mutex;
use stivale_boot::v2::StivaleTerminalTag;

use crate::STIVALE_STRUCT;

lazy_static! {
    static ref TERMINAL: Mutex<Terminal> =
        unsafe { Mutex::new(Terminal::new(STIVALE_STRUCT.unwrap().terminal().unwrap(),)) };
}

pub struct Terminal {
    terminal_tag: &'static StivaleTerminalTag,
}

impl Terminal {
    pub fn new(terminal_tag: &'static StivaleTerminalTag) -> Self {
        Self { terminal_tag }
    }
}

impl core::fmt::Write for Terminal {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let term_write_ptr = self.terminal_tag.term_write_addr as *const ();
        let term_write = unsafe {
            core::mem::transmute::<*const (), extern "C" fn(*const i8, u64)>(term_write_ptr)
        };
        term_write(s.as_ptr() as *const i8, s.len() as u64);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::terminal::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    TERMINAL.lock().write_fmt(args).unwrap();
}
