use crate::{
    psf::Font,
    video::{ColorRGB, FRAMEBUFFER},
};

pub struct Terminal {
    font: Font<'static>,
    col: usize,
    row: usize,
    height: usize,
    width: usize,
}

impl Terminal {
    pub fn new(font: Font<'static>) -> Self {
        let framebuffer = FRAMEBUFFER.lock();
        let height = framebuffer.height / font.glyph_height;
        let width = framebuffer.width / font.glyph_width;

        Self {
            font,
            col: 0,
            row: 0,
            height,
            width,
        }
    }

    pub fn write(&mut self, s: &str) {
        for b in s.bytes() {
            match b {
                b'\n' => self.next_line(),
                b if !b.is_ascii_control() => self.write_byte(b),
                _ => self.write_byte(0),
            }
        }
    }

    fn write_byte(&mut self, b: u8) {
        let mut framebuffer = FRAMEBUFFER.lock();
        let glyph = self.font.get_nth_glyph(b as usize);
        for (dy, row) in glyph.0.iter().enumerate() {
            for dx in 0..8 {
                let x = self.col * self.font.glyph_width + dx;
                let y = self.row * self.font.glyph_height + dy;

                let bit_idx = 7 - dx;
                let bit = (row >> bit_idx) & 1;
                if bit == 1 {
                    framebuffer.put_pixel(x, y, ColorRGB(255, 255, 255));
                }
            }
        }

        self.col += 1;
        if self.col >= self.width {
            self.next_line();
        }
    }

    fn next_line(&mut self) {
        self.col = 0;
        self.row += 1;

        if self.row >= self.height {
            self.scroll()
        }
    }

    fn scroll(&mut self) {
        unimplemented!()
    }
}

impl core::fmt::Write for Terminal {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s);
        Ok(())
    }
}
