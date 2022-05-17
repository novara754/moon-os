use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    static ref COM1: Mutex<Com1> = Mutex::new(Com1);
}

struct Com1;

impl core::fmt::Write for Com1 {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for &b in s.as_bytes() {
            unsafe {
                let port = 0x3F8; // COM1
                core::arch::asm!("out dx, al", in("dx") port, in("al") b, options(nomem, nostack, preserves_flags));
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ($crate::serial::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    COM1.lock().write_fmt(args).unwrap();
}
