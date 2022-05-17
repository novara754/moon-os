#![no_std]
#![no_main]
#![feature(bench_black_box)]

mod serial;
mod terminal;

use stivale_boot::v2::{
    StivaleFramebufferHeaderTag, StivaleHeader, StivaleStruct, StivaleTerminalHeaderTag,
};

extern "C" {
    static KERNEL_STACK_TOP: u8;
}

static TERMINAL_TAG: StivaleTerminalHeaderTag = StivaleTerminalHeaderTag::new();
static FRAMEBUFFER_TAG: StivaleFramebufferHeaderTag = StivaleFramebufferHeaderTag::new()
    .framebuffer_bpp(24)
    .next((&TERMINAL_TAG as *const StivaleTerminalHeaderTag).cast());

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

    assert!(false, "BAADD!");

    loop {
        unsafe { core::arch::asm!("hlt") }
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    serial_println!("{}", info);

    loop {
        unsafe { core::arch::asm!("hlt") }
    }
}
