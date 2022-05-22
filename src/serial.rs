use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

pub const COM1_PORT: u16 = 0x3F8;

lazy_static! {
    static ref COM1: Mutex<SerialPort> = Mutex::new(unsafe { SerialPort::new(COM1_PORT) });
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
