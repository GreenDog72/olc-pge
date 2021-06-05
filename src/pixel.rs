#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Pixel {
    pub a: u8, pub r: u8, pub g: u8, pub b: u8
}

impl Pixel {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Pixel { a: 255, r: r, g: g, b: b }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Pixel { a: a, r: r, g: g, b: b }
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Pixel::rgb(0, 0, 0)
    }
}

impl Into<u32> for Pixel {
    fn into(self) -> u32 {
        (self.a as u32) << 24 | (self.r as u32) << 16 | (self.g as u32) << 8 | (self.b as u32)
    }
}

impl From<u32> for Pixel {
    fn from(p: u32) -> Self {
        Pixel { a: (p >> 24) as u8, r: (p >> 16) as u8, g: (p >> 8) as u8, b: p as u8 }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum PixelMode {
    Normal, Mask, Alpha, Custom
}

pub const GREY:              Pixel = Pixel { a: 255, r: 192, g: 192, b: 192 };
pub const DARK_GREY:         Pixel = Pixel { a: 255, r: 128, g: 128, b: 128 };
pub const VERY_DARK_GREY:    Pixel = Pixel { a: 255, r:  64, g:  64, b:  64 };
pub const RED:               Pixel = Pixel { a: 255, r: 255, g:   0, b:   0 };
pub const DARK_RED:          Pixel = Pixel { a: 255, r: 128, g:   0, b:   0 };
pub const VERY_DARK_RED:     Pixel = Pixel { a: 255, r:  64, g:   0, b:   0 };
pub const YELLOW:            Pixel = Pixel { a: 255, r: 255, g: 255, b:   0 };
pub const DARK_YELLOW:       Pixel = Pixel { a: 255, r: 128, g: 128, b:   0 };
pub const VERY_DARK_YELLOW:  Pixel = Pixel { a: 255, r:  64, g:  64, b:   0 };
pub const GREEN:             Pixel = Pixel { a: 255, r:   0, g: 255, b:   0 };
pub const DARK_GREEN:        Pixel = Pixel { a: 255, r:   0, g: 128, b:   0 };
pub const VERY_DARK_GREEN:   Pixel = Pixel { a: 255, r:   0, g:  64, b:   0 };
pub const CYAN:              Pixel = Pixel { a: 255, r:   0, g: 255, b: 255 };
pub const DARK_CYAN:         Pixel = Pixel { a: 255, r:   0, g: 128, b: 128 };
pub const VERY_DARK_CYAN:    Pixel = Pixel { a: 255, r:   0, g:  64, b:  64 };
pub const BLUE:              Pixel = Pixel { a: 255, r:   0, g:   0, b: 255 };
pub const DARK_BLUE:         Pixel = Pixel { a: 255, r:   0, g:   0, b: 128 };
pub const VERY_DARK_BLUE:    Pixel = Pixel { a: 255, r:   0, g:   0, b:  64 };
pub const MAGENTA:           Pixel = Pixel { a: 255, r: 255, g:   0, b: 255 };
pub const DARK_MAGENTA:      Pixel = Pixel { a: 255, r: 128, g:   0, b: 128 };
pub const VERY_DARK_MAGENTA: Pixel = Pixel { a: 255, r:  64, g:   0, b:  64 };
pub const WHITE:             Pixel = Pixel { a: 255, r: 255, g: 255, b: 255 };
pub const BLACK:             Pixel = Pixel { a: 255, r:   0, g:   0, b:   0 };
pub const BLANK:             Pixel = Pixel { a:   0, r:   0, g:   0, b:   0 };