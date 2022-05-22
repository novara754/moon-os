#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod gdt;
mod interrupts;
mod serial;
mod video;

use stivale_boot::v2::{StivaleFramebufferHeaderTag, StivaleHeader, StivaleStruct};

use crate::video::{ColorRGB, FRAMEBUFFER};

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

    /* Placeholder code just to see if we get here. */
    {
        let mut framebuffer = FRAMEBUFFER.lock();
        for y in 0..framebuffer.height {
            for x in 0..framebuffer.width {
                let r = x * 255 / framebuffer.width;
                let g = y * 255 / framebuffer.height;
                framebuffer.put_pixel(x, y, ColorRGB(r as u8, g as u8, 200));
            }
        }
    }
    /* --- */

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
