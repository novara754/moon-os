#![no_std]
#![no_main]
#![feature(bench_black_box)]

use stivale_boot::v2::{
    StivaleFramebufferHeaderTag, StivaleHeader, StivaleStruct, StivaleTerminalHeaderTag,
};

// const STACK_SIZE: usize = 4096;
// static STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
extern "C" {
    static KERNEL_STACK_TOP: u8;
}

static TERMINAL_TAG: StivaleTerminalHeaderTag = StivaleTerminalHeaderTag::new();
static FRAMEBUFFER_TAG: StivaleFramebufferHeaderTag = StivaleFramebufferHeaderTag::new()
    .next((&TERMINAL_TAG as *const StivaleTerminalHeaderTag).cast());

#[link_section = ".stivale2hdr"]
#[no_mangle]
#[used]
static STIVALE_HDR: StivaleHeader = StivaleHeader::new()
    .flags((1 << 1) | (1 << 2) | (1 << 3) | (1 << 4))
    .stack(unsafe { &KERNEL_STACK_TOP as *const u8 })
    .tags((&FRAMEBUFFER_TAG as *const StivaleFramebufferHeaderTag).cast());

#[no_mangle]
pub extern "C" fn kernel_main(stivale_struct: &'static StivaleStruct) -> ! {
    // for &b in b"Hello, World!\n" {
    //     unsafe {
    //         let port = 0x3F8; // COM1
    //         core::arch::asm!("out dx, al", in("dx") port, in("al") b, options(nomem, nostack, preserves_flags));
    //     }
    // }

    if let Some(term_write) = stivale_struct.terminal().map(|t| t.term_write()) {
        term_write("Hello world from MoonOS, booted with Stivale2 & Limine!");
    }

    loop {
        unsafe { core::arch::asm!("hlt") }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe { core::arch::asm!("hlt") }
    }
}
