#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod gdt;
mod interrupts;
mod module;
mod psf;
mod serial;
mod terminal;
mod video;

use stivale_boot::v2::{StivaleFramebufferHeaderTag, StivaleHeader, StivaleStruct};

use crate::terminal::Terminal;

extern "C" {
    static KERNEL_STACK_TOP: u8;
}

static FRAMEBUFFER_TAG: StivaleFramebufferHeaderTag =
    StivaleFramebufferHeaderTag::new().framebuffer_bpp(24);

#[link_section = ".stivale2hdr"]
#[no_mangle]
#[used]
static STIVALE_HDR: StivaleHeader = StivaleHeader::new()
    .flags((1 << 1) | (1 << 2) | (1 << 3) | (1 << 4))
    .stack(unsafe { &KERNEL_STACK_TOP as *const u8 })
    .tags((&FRAMEBUFFER_TAG as *const StivaleFramebufferHeaderTag).cast());

pub static mut STIVALE_STRUCT: Option<&'static StivaleStruct> = None;

#[no_mangle]
pub extern "C" fn kernel_main(stivale_struct: &'static StivaleStruct) -> ! {
    unsafe {
        STIVALE_STRUCT = Some(stivale_struct);
    }

    kprint!("[KERNEL] Initializing GDT...");
    gdt::init();
    kprintln!("OK");

    kprint!("[KERNEL] Initializing IDT...");
    interrupts::init();
    kprintln!("OK");

    let modules = stivale_struct.modules().unwrap();
    kprintln!("[KERNEL] Modules loaded:");
    for (i, module) in modules.iter().enumerate() {
        kprintln!("  {}. {}", i + 1, module.as_str());
    }

    let mut terminal = {
        use module::KernelModule;
        let font_module = &modules.as_slice()[0];
        assert_eq!(
            font_module.as_str(),
            "TERMINAL_FONT",
            "expected terminal font to be first kernel module"
        );
        let data = font_module.data();
        let font = psf::Font::try_from_slice(data).expect("invalid psf data");
        Terminal::new(font)
    };

    for i in 0..100 {
        use core::fmt::Write;
        write!(terminal, "{i} ").unwrap();
    }

    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    kprintln!("{}", info);

    loop {
        x86_64::instructions::hlt();
    }
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {{
        $crate::serial::_print(format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! kprintln {
    () => ($crate::kprint!("\n"));
    ($($arg:tt)*) => ($crate::kprint!("{}\n", format_args!($($arg)*)));
}
