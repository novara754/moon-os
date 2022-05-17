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
        assert_eq!(self.bytes_per_pixel, 4, "framebuffer bpp is not 24");

        let pixel_idx = y * self.width + x;
        let byte_idx = pixel_idx * self.bytes_per_pixel;
        self.data[byte_idx..][..self.bytes_per_pixel].copy_from_slice(&color.to_bgra());
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: ColorRGB) {
        for dy in y..(y + h) {
            for dx in x..(x + w) {
                self.put_pixel(dx, dy, color);
            }
        }
    }
}

// let framebuffer = stivale_struct.framebuffer().unwrap();
// println!("framebuffer_addr: {:X}", framebuffer.framebuffer_addr);
// println!("framebuffer_width: {}", framebuffer.framebuffer_width);
// println!("framebuffer_height: {}", framebuffer.framebuffer_height);
// println!("framebuffer_pitch: {}", framebuffer.framebuffer_pitch);
// println!("framebuffer_bpp: {}", framebuffer.framebuffer_bpp);
// println!("memory_model: {}", framebuffer.memory_model);

// let video_buffer = unsafe {
//     core::slice::from_raw_parts_mut(
//         framebuffer.framebuffer_addr as *mut u8,
//         framebuffer.framebuffer_pitch as usize * framebuffer.framebuffer_height as usize,
//     )
// };

// for y in 100..355 {
//     for x in 100..355 {
//         let i = y * framebuffer.framebuffer_width as usize + x;
//         video_buffer[i * 4 + 0] = (y - 100) as u8;
//         video_buffer[i * 4 + 1] = (x - 100) as u8;
//         video_buffer[i * 4 + 2] = 200;
//         video_buffer[i * 4 + 3] = 255;

//         // println!("{:X?}", &video_buffer[(i * 4)..(i * 4 + 4)])
//     }
// }
