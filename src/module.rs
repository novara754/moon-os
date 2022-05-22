use stivale_boot::v2::StivaleModule;

pub trait KernelModule {
    fn data(&'static self) -> &'static [u8];
}

impl KernelModule for StivaleModule {
    fn data(&'static self) -> &'static [u8] {
        let start_addr = self.start as *const u8;
        let len = (self.end - self.start) as usize;
        unsafe { core::slice::from_raw_parts(start_addr, len) }
    }
}
