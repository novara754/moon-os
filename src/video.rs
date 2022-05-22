use lazy_static::lazy_static;
use spin::Mutex;
use stivale_boot::v2::StivaleFramebufferTag;

use crate::STIVALE_STRUCT;

lazy_static! {
    pub static ref FRAMEBUFFER: Mutex<Framebuffer> = unsafe {
        Mutex::new(Framebuffer::new(
            STIVALE_STRUCT.unwrap().framebuffer().unwrap(),
        ))
    };
}

#[derive(Debug, Clone, Copy)]
pub struct ColorRGB(pub u8, pub u8, pub u8);

impl ColorRGB {
    fn to_bgra(self) -> [u8; 4] {
        [self.2, self.1, self.0, 255]
    }
}

pub struct Framebuffer {
    data: &'static mut [u8],
    pub width: usize,
    pub height: usize,
    bytes_per_pixel: usize,
}

impl Framebuffer {
    pub fn new(framebuffer_tag: &'static StivaleFramebufferTag) -> Self {
        let data = unsafe {
            core::slice::from_raw_parts_mut(
                framebuffer_tag.framebuffer_addr as *mut u8,
                framebuffer_tag.framebuffer_pitch as usize
                    * framebuffer_tag.framebuffer_height as usize,
            )
        };
        Self {
            data,
            width: framebuffer_tag.framebuffer_width as usize,
            height: framebuffer_tag.framebuffer_height as usize,
            bytes_per_pixel: framebuffer_tag.framebuffer_bpp as usize / 8,
        }
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, color: ColorRGB) {
        assert_eq!(self.bytes_per_pixel, 4, "framebuffer bpp is not 32");

        let pixel_idx = y * self.width + x;
        let byte_idx = pixel_idx * self.bytes_per_pixel;
        self.data[byte_idx..][..self.bytes_per_pixel].copy_from_slice(&color.to_bgra());
    }

    // TODO: Remove this maybe?
    pub fn _draw_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: ColorRGB) {
        for dy in y..(y + h) {
            for dx in x..(x + w) {
                self.put_pixel(dx, dy, color);
            }
        }
    }
}
