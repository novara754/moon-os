const PSF_VERSION1_MAGIC: u16 = 0x0436;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    InvalidMagic(u16),
    MissingHeader,
    MissingFontData,
}

#[derive(Debug)]
pub struct Font<'a> {
    pub num_glyphs: usize,
    _has_unicode_table: bool,
    _has_glyph_seqs: bool,
    pub glyph_width: usize,
    pub glyph_height: usize,
    font_data: &'a [u8],
}

#[derive(Debug)]
pub struct Glyph<'a>(pub &'a [u8]);

impl<'a> Font<'a> {
    pub fn try_from_slice(data: &'a [u8]) -> Result<Self, ParseError> {
        if data.len() < 4 {
            return Err(ParseError::MissingHeader);
        }

        let magic = ((data[1] as u16) << 8) | data[0] as u16;
        if magic != PSF_VERSION1_MAGIC {
            return Err(ParseError::InvalidMagic(magic));
        }

        let mode = data[2];
        let num_glyphs = if (mode & (1 << 0)) == 0 { 256 } else { 512 };
        let has_unicode_table = (mode & (1 << 1)) != 0;
        let has_char_seqs = (mode & (1 << 2)) != 0;
        let glyph_height = data[3] as usize;

        if data[4..].len() < (num_glyphs * glyph_height) {
            return Err(ParseError::MissingFontData);
        }

        Ok(Self {
            num_glyphs,
            _has_unicode_table: has_unicode_table,
            _has_glyph_seqs: has_char_seqs,
            glyph_width: 8,
            glyph_height,
            font_data: &data[4..],
        })
    }

    pub fn get_nth_glyph(&self, n: usize) -> Glyph<'a> {
        let len = self.glyph_height as usize;
        let idx = n * len;
        Glyph(&self.font_data[idx..][..len])
    }
}
