use std::time::Instant;

pub mod pixel;
pub mod sprite;
pub mod vector;

pub use pixel::*;
pub use sprite::*;
pub use vector::*;

mod font_data;
use font_data::FONT_DATA;

#[derive(PartialEq)]
pub enum RCode {
    Fail, Ok, NoFile
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Key {
    None,                                                                                      // 1
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,              // 26
    K0, K1, K2, K3, K4, K5, K6, K7, K8, K9,                                                    // 10
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,                                         // 12
    Up, Down, Left, Right,                                                                     // 4
    Comma, Period, Apostrophe, BackQuote, Semicolon,                                           // 5
    Space, Tab, Minus, Equal,                                                                  // 4
    LeftBracket, RightBracket, Slash, BackSlash,                                               // 4
    Alt, LeftAlt, RightAlt,                                                                    // 3
    Shift, LeftShift, RightShift,                                                              // 3
    Control, LeftControl, RightControl,                                                        // 3
    System, LeftSystem, RightSystem,                                                           // 3
    Menu, Insert, Delete, Home, End, PageUp, PageDown,                                         // 7
    Back, Escape, Return, Enter, Pause,                                                        // 5
    NumLock, CapsLock, ScrollLock,                                                             // 3
    NumPad0, NumPad1, NumPad2, NumPad3, NumPad4, NumPad5, NumPad6, NumPad7, NumPad8, NumPad9,  // 10
    NumPadMul, NumPadDiv, NumPadAdd, NumPadSub, NumPadDecimal, NumPadEnter,                    // 6
    Count = 109
}

const KEYMAP: [Key; minifb::Key::Count as usize] = [
    Key::K0, Key::K1, Key::K2, Key::K3, Key::K4, Key::K5, Key::K6, Key::K7, Key::K8, Key::K9,
    Key::A, Key::B, Key::C, Key::D, Key::E, Key::F, Key::G, Key::H, Key::I, Key::J, Key::K, Key::L,
    Key::M, Key::N, Key::O, Key::P, Key::Q, Key::R, Key::S, Key::T, Key::U, Key::V, Key::W, Key::X,
    Key::Y, Key::Z,
    Key::F1, Key::F2, Key::F3, Key::F4, Key::F5, Key::F6,
    Key::F7, Key::F8, Key::F9, Key::F10, Key::F11, Key::F12,
    Key::None, Key::None, Key::None, // F13, F14, F15
    Key::Down, Key::Left, Key::Right, Key::Up,
    Key::Apostrophe, Key::BackQuote, Key::BackSlash, Key::Comma,
    Key::Equal, Key::LeftBracket, Key::Minus, Key::Period,
    Key::RightBracket, Key::Semicolon, Key::Slash,
    Key::Back, Key::Delete, Key::End, Key::Return, Key::Escape,
    Key::Home, Key::Insert, Key::Menu,
    Key::PageDown, Key::PageUp, Key::Pause, Key::Space, Key::Tab,
    Key::NumLock, Key::CapsLock, Key::ScrollLock,
    Key::LeftShift, Key::RightShift, Key::LeftControl, Key::RightControl,
    Key::NumPad0, Key::NumPad1, Key::NumPad2, Key::NumPad3, Key::NumPad4,
    Key::NumPad5, Key::NumPad6, Key::NumPad7, Key::NumPad8, Key::NumPad9,
    Key::NumPadDecimal, Key::NumPadDiv, Key::NumPadMul, Key::NumPadSub, Key::NumPadAdd,
    Key::NumPadEnter, Key::LeftAlt, Key::RightAlt,
    Key::LeftSystem, Key::RightSystem,
    Key::None // Unknown
];

#[derive(Debug, Copy, Clone)]
pub struct HWButton {
    pub pressed: bool,
    pub released: bool,
    pub held: bool
}

impl Default for HWButton {
    fn default() -> Self { Self { pressed: false, released: false, held: false } }
}

pub trait PGEApplication {
    const APP_NAME: &'static str;
    fn on_user_create(&mut self, _pge: &mut PixelGameEngine) -> bool { true }
    fn on_user_update(&mut self, pge: &mut PixelGameEngine, elapsed_time: f32) -> bool;
    fn on_user_destroy(&mut self) -> bool { true }
}

#[derive(Debug)]
pub struct PixelGameEngine {
    screen_w: usize,
    screen_h: usize,
    pixel_w: usize,
    pixel_h: usize,

    mouse_pos_x: i32,
    mouse_pos_y: i32,
    mouse_wheel_delta: i32,

    mouse_state: [HWButton; 5],

    keyboard_state: [HWButton; Key::Count as usize],

    active: bool,
    default_draw_target: SpriteRef,
    draw_target: SpriteRef,

    pixel_mode: PixelMode,
    blend_factor: f32,

    window: minifb::Window
}

impl PixelGameEngine {
    // Hardware Interfaces
    pub fn is_focused(&mut self) -> bool { self.window.is_active() }
    pub fn get_key(&self, k: Key) -> HWButton { self.keyboard_state[k as usize] }
    pub fn get_mouse(&self, button: usize) -> HWButton { self.mouse_state[button] }
    pub fn get_mouse_x(&self) -> i32 { self.mouse_pos_x }
    pub fn get_mouse_y(&self) -> i32 { self.mouse_pos_y }
    pub fn get_mouse_wheel(&self) -> i32 { self.mouse_wheel_delta }

    // Utility
    pub fn screen_width(&self) -> usize { self.screen_w }
    pub fn screen_height(&self) -> usize { self.screen_h }
    pub fn get_draw_target_width(&self) -> u32 { self.draw_target.borrow().width() }
    pub fn get_draw_target_height(&self) -> u32 { self.draw_target.borrow().height() }
    pub fn get_draw_target(&self) -> SpriteRef { self.draw_target.clone() }

    // Draw Routines
    pub fn set_draw_target(&mut self, target: Option<SpriteRef>) {
        match target {
            Some(target) => self.draw_target = target,
            None => self.draw_target = self.default_draw_target.clone()
        }
    }
    pub fn set_pixel_mode(&mut self, mode: PixelMode) { self.pixel_mode = mode; }
    pub fn get_pixel_mode(&self) -> PixelMode { self.pixel_mode }
    pub fn set_pixel_blend(&mut self, blend: f32) { self.blend_factor = blend; }
    pub fn set_sub_pixel_offset(&mut self, _ox: f32, _oy: f32) { /* unimplemented!() */ }

    pub fn draw(&mut self, x: i32, y: i32, p: Pixel) -> bool { self._draw(x, y, p) }
    pub fn draw_v(&mut self, pos: Vi2d, p: Pixel) -> bool { self._draw(pos.x, pos.y, p) }

    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, p: Pixel) { self._draw_line_pattern(x1, y1, x2, y2, p, 0xffffffff); }
    pub fn draw_line_pattern(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, p: Pixel, pattern: u32) { self._draw_line_pattern(x1, y1, x2, y2, p, pattern); }

    pub fn draw_line_v(&mut self, pos1: Vi2d, pos2: Vi2d, p: Pixel) { self._draw_line_pattern(pos1.x, pos1.y, pos2.x, pos2.y, p, 0xffffffff); }
    pub fn draw_line_pattern_v(&mut self, pos1: Vi2d, pos2: Vi2d, p: Pixel, pattern: u32) { self._draw_line_pattern(pos1.x, pos1.y, pos2.x, pos2.y, p, pattern); }

    pub fn draw_circle(&mut self, x: i32, y: i32, radius: i32, p: Pixel) { self._draw_circle_mask(x, y, radius, p, 0xff); }
    pub fn draw_circle_mask(&mut self, x: i32, y: i32, radius: i32, p: Pixel, mask: u8) { self._draw_circle_mask(x, y, radius, p, mask); }

    pub fn draw_circle_v(&mut self, pos: Vi2d, radius: i32, p: Pixel) { self._draw_circle_mask(pos.x, pos.y, radius, p, 0xff); }
    pub fn draw_circle_mask_v(&mut self, pos: Vi2d, radius: i32, p: Pixel, mask: u8) { self._draw_circle_mask(pos.x, pos.y, radius, p, mask); }

    pub fn fill_circle(&mut self, x: i32, y: i32, radius: i32, p: Pixel) { self._fill_circle(x, y, radius, p); }
    pub fn fill_circle_v(&mut self, pos: Vi2d, radius: i32, p: Pixel) { self.fill_circle(pos.x, pos.y, radius, p); }

    pub fn draw_rect(&mut self, x: i32, y: i32, w: u32, h: u32, p: Pixel) { self._draw_rect(x, y, w as i32, h as i32, p); }
    pub fn draw_rect_v(&mut self, pos: Vi2d, size: Vi2d, p: Pixel) { self._draw_rect(pos.x, pos.y, size.x, size.y, p); }

    pub fn fill_rect(&mut self, x: i32, y: i32, w: u32, h: u32, p: Pixel) { self._fill_rect(x, y, w, h, p); }
    pub fn fill_rect_v(&mut self, pos: Vi2d, size: Vi2d, p: Pixel) { self.fill_rect(pos.x, pos.y, size.x as u32, size.y as u32, p); }

    pub fn draw_triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, p: Pixel) { self._draw_triangle(x1, y1, x2, y2, x3, y3, p); }
    pub fn draw_triangle_v(&mut self, pos1: Vi2d, pos2: Vi2d, pos3: Vi2d, p: Pixel) { self._draw_triangle(pos1.x, pos1.y, pos2.x, pos2.y, pos3.x, pos3.y, p); }

    pub fn fill_triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, p: Pixel) { self._fill_triangle(x1, y1, x2, y2, x3, y3, p); }
    pub fn fill_triangle_v(&mut self, pos1: Vi2d, pos2: Vi2d, pos3: Vi2d, p: Pixel) { self._fill_triangle(pos1.x, pos1.y, pos2.x, pos2.y, pos3.x, pos3.y, p); }

    pub fn draw_sprite(&mut self, x: i32, y: i32, sprite: SpriteRef) { self._draw_sprite(x, y, sprite, 1); }
    pub fn draw_sprite_scaled(&mut self, x: i32, y: i32, sprite: SpriteRef, scale: u32) { self._draw_sprite(x, y, sprite, scale); }

    pub fn draw_sprite_v(&mut self, pos: Vi2d, sprite: SpriteRef) { self.draw_sprite_scaled(pos.x, pos.y, sprite, 1); }
    pub fn draw_sprite_scaled_v(&mut self, pos: Vi2d, sprite: SpriteRef, scale: u32) { self.draw_sprite_scaled(pos.x, pos.y, sprite, scale); }

    pub fn draw_partial_sprite(&mut self, x: i32, y: i32, sprite: SpriteRef, ox: i32, oy: i32, w: u32, h: u32) { self._draw_partial_sprite(x, y, sprite, ox, oy, w, h, 1); }
    pub fn draw_partial_sprite_scaled(&mut self, x: i32, y: i32, sprite: SpriteRef, ox: i32, oy: i32, w: u32, h: u32, scale: u32) { self._draw_partial_sprite(x, y, sprite, ox, oy, w, h, scale); }

    pub fn draw_partial_sprite_v(&mut self, pos: Vi2d, sprite: SpriteRef, source_pos: Vi2d, size: Vi2d) { self._draw_partial_sprite(pos.x, pos.y, sprite, source_pos.x, source_pos.y, size.x as u32, size.y as u32, 1); }
    pub fn draw_partial_sprite_scaled_v(&mut self, pos: Vi2d, sprite: SpriteRef, source_pos: Vi2d, size: Vi2d, scale: u32) { self._draw_partial_sprite(pos.x, pos.y, sprite, source_pos.x, source_pos.y, size.x as u32, size.y as u32, scale); }

    pub fn draw_string(&mut self, x: i32, y: i32, text: &String, col: Pixel) { self._draw_string_scaled(x, y, text, col, 1); }
    pub fn draw_string_scaled(&mut self, x: i32, y: i32, text: &String, col: Pixel, scale: u32) { self._draw_string_scaled(x, y, text, col, scale); }

    pub fn draw_string_v(&mut self, pos: Vi2d, text: &String, col: Pixel) { self._draw_string_scaled(pos.x, pos.y, text, col, 1); }
    pub fn draw_string_scaled_v(&mut self, pos: Vi2d, text: &String, col: Pixel, scale: u32) { self._draw_string_scaled(pos.x, pos.y, text, col, scale); }

    pub fn clear(&mut self, p: Pixel) { self._clear(p); }
    pub fn set_screen_size(&mut self, _w: usize, _h: usize) { /* unimplemented!() */ }

    // implementations

    #[inline]
    fn _draw(&mut self, x: i32, y: i32, p: Pixel) -> bool {
        let mut draw_target = self.draw_target.borrow_mut();
        match self.pixel_mode {
            PixelMode::Normal => draw_target.set_pixel(x, y, p),
            PixelMode::Mask => if p.a == 255 { draw_target.set_pixel(x, y, p) } else { false }
            PixelMode::Alpha => {
                let d = draw_target.get_pixel(x, y);
                let a = (p.a as f32 / 255.0) * self.blend_factor;
                let c = 1.0 - a;
                let r = a * p.r as f32 + c * d.r as f32;
                let g = a * p.g as f32 + c * d.g as f32;
                let b = a * p.b as f32 + c * d.b as f32;
                draw_target.set_pixel(x, y, Pixel::rgb(r as u8, g as u8, b as u8))
            }
            PixelMode::Custom => {
                //unimplemented!()
                draw_target.set_pixel(x, y, p)
            }
        }
    }

    #[inline]
    fn _draw_line_pattern(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, p: Pixel, pattern: u32) {
        let dx = x2 - x1;
        let dy = y2 - y1;

        let mut pattern = pattern;

        let mut rol = || -> bool {
            pattern = (pattern << 1) | (pattern >> 31);
            pattern & 1 != 0
        };

        let dx1 = dx.abs();
        let dy1 = dy.abs();

        let mut px = 2 * dy1 - dx1;
        let mut py = 2 * dx1 - dy1;

        if dy1 <= dx1 {
            let (mut x, mut y, xe) =
                if dx >= 0 { (x1, y1, x2) }
                else { (x2, y2, x1) };

            if rol() { self.draw(x, y, p); }

            while x < xe {
                x += 1;
                if px < 0 {
                    px += 2 * dy1;
                }
                else {
                    if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
                        y += 1;
                    }
                    else {
                        y -= 1;
                    }
                    px += 2 * (dy1 - dx1);
                }
                if rol() { self.draw(x, y, p); }
            }
        }
        else {
            let (mut x, mut y, ye) =
                if dy >= 0 { (x1, y1, y2) }
                else { (x2, y2, y1) };

            if rol() { self.draw(x, y, p); }

            while y < ye {
                y += 1;
                if py <= 0 {
                    py += 2 * dx1;
                }
                else {
                    if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
                        x += 1;
                    }
                    else {
                        x -= 1;
                    }
                    py += 2 * (dx1 - dy1);
                }
                if rol() { self.draw(x, y, p); }
            }
        }
    }

    #[inline]
    fn _draw_circle_mask(&mut self, x: i32, y: i32, radius: i32, p: Pixel, mask: u8) {
        if radius == 0 { return; }

        let mut x0 = 0;
        let mut y0 = radius;
        let mut d = 3 - (2 * radius);

        while y0 >= x0 {
            if mask & 0x01 != 0 { self._draw(x + x0, y - y0, p); }
            if mask & 0x02 != 0 { self._draw(x + y0, y - x0, p); }
            if mask & 0x04 != 0 { self._draw(x + y0, y + x0, p); }
            if mask & 0x08 != 0 { self._draw(x + x0, y + y0, p); }
            if mask & 0x10 != 0 { self._draw(x - x0, y + y0, p); }
            if mask & 0x20 != 0 { self._draw(x - y0, y + x0, p); }
            if mask & 0x40 != 0 { self._draw(x - y0, y - x0, p); }
            if mask & 0x80 != 0 { self._draw(x - x0, y - y0, p); }

            if d < 0 { d += 4 * x0 + 6; x0 += 1; }
            else { d += 4 * (x0 - y0) + 10; x0 += 1; y0 -= 1; }
        }
    }

    #[inline]
    fn _fill_circle(&mut self, x: i32, y: i32, r: i32, p: Pixel) {
        if r == 0 { return }
        
        let mut x0 = 0;
        let mut y0 = r;
        let mut d = 3 - 2 * r;

        let mut draw_line = |sx: i32, ex: i32, ny: i32| {
            for i in sx..ex {
                self.draw(i, ny, p);
            }
        };

        while y0 >= x0 {
            draw_line(x - x0, x + x0, y - y0);
            draw_line(x - y0, x + y0, y - x0);
            draw_line(x - x0, x + x0, y + y0);
            draw_line(x - y0, x + y0, y + x0);
            if d < 0 {
                d += 4 * x0 + 6;
                x0 += 1;
            }
            else {
                d += 4 * (x0 - y0) + 10;
                x0 += 1;
                y0 -= 1;
            }
        }
    }

    #[inline]
    fn _draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32, p: Pixel) {
        self._draw_line_pattern(x, y, x + w, y, p, 0xffffffff);
        self._draw_line_pattern(x + w, y, x + w, y + h, p, 0xffffffff);
        self._draw_line_pattern(x + w, y + h, x, y + h, p, 0xffffffff);
        self._draw_line_pattern(x, y + h, x, y, p, 0xffffffff);
    }

    #[inline]
    fn _fill_rect(&mut self, x: i32, y: i32, w: u32, h: u32, p: Pixel) {
        for x in x..x + w as i32 {
            for y in y..y + h as i32 {
                self.draw(x, y, p);
        }}
    }

    #[inline]
    fn _draw_triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, p: Pixel) {
        self._draw_line_pattern(x1, y1, x2, y2, p, 0xffffffff);
        self._draw_line_pattern(x2, y2, x3, y3, p, 0xffffffff);
        self._draw_line_pattern(x3, y3, x1, y1, p, 0xffffffff);
    }

    #[inline]
    // http://www.sunshine2k.de/coding/java/TriangleRasterization/TriangleRasterization.html
    fn _fill_triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, p: Pixel) {
        let (v1, mut v2, v3) =
            if y1 < y2 {
                if y1 < y3 {
                    if y2 < y3 { (Vi2d::new(x1, y1), Vi2d::new(x2, y2), Vi2d::new(x3, y3)) }
                    else { (Vi2d::new(x1, y1), Vi2d::new(x3, y3), Vi2d::new(x2, y2)) }}
                else { (Vi2d::new(x3, y3), Vi2d::new(x1, y1), Vi2d::new(x2, y2)) }
            }
            else {
                if y2 < y3 {
                    if y1 < y3 { (Vi2d::new(x2, y2), Vi2d::new(x1, y1), Vi2d::new(x3, y3)) }
                    else { (Vi2d::new(x2, y2), Vi2d::new(x3, y3), Vi2d::new(x1, y1)) }}
                else { (Vi2d::new(x3, y3), Vi2d::new(x2, y2), Vi2d::new(x1, y1)) }
            };

        let mut v4 = Vi2d::new(
            v1.x + ((v2.y - v1.y) as f32 / (v3.y - v1.y) as f32 * (v3.x - v1.x) as f32) as i32,
            v2.y
        );

        if v4.x < v2.x {
            std::mem::swap(&mut v2, &mut v4);
        }
        
        // top, bottom-flat triangle
        {
            let (v1, v2, v3) = (v1, v2, v4);

            let invslope1 = (v2.x - v1.x) as f32 / (v2.y - v1.y) as f32;
            let invslope2 = (v3.x - v1.x) as f32 / (v3.y - v1.y) as f32;

            let mut curx1 = v1.x as f32;
            let mut curx2 = v1.x as f32;

            for scanline_y in v1.y ..= v2.y {
                for x in curx1 as i32 ..= curx2 as i32 {
                    self.draw(x, scanline_y, p);
                }
                curx1 += invslope1;
                curx2 += invslope2;
            }
        }

        // bottom, top-flat triangle
        {
            let (v1, v2, v3) = (v2, v4, v3);

            let invslope1 = (v3.x - v1.x) as f32 / (v3.y - v1.y) as f32;
            let invslope2 = (v3.x - v2.x) as f32 / (v3.y - v2.y) as f32;

            let mut curx1 = v1.x as f32;
            let mut curx2 = v2.x as f32;

            for y in v1.y ..= v3.y {
                for x in curx1 as i32 ..= curx2 as i32 {
                    self.draw(x, y, p);
                }
                curx1 += invslope1;
                curx2 += invslope2;
            }
        }
    }

    #[inline]
    fn _draw_sprite(&mut self, x: i32, y: i32, sprite: SpriteRef, scale: u32) {
        let sprite = sprite.borrow();
        match scale {
            0 => (),
            1 => for i in 0..sprite.width() as i32 {
                for j in 0..sprite.height() as i32 {
                    self.draw(x + i, y + j, sprite.get_pixel(i, j));
            }}
            _ => for i in 0..sprite.width() as i32 {
                for j in 0..sprite.height() as i32 {
                    for is in 0..scale as i32 {
                        for js in 0..scale as i32 {
                            let scale = scale as i32;
                            self.draw(x + (i * scale) + is, y + (j * scale) + js, sprite.get_pixel(i, j));
            }}}}
        }
    }

    #[inline]
    fn _draw_partial_sprite(&mut self, x: i32, y: i32, sprite: SpriteRef, ox: i32, oy: i32, w: u32, h: u32, scale: u32) {
        let sprite = sprite.borrow();
        match scale {
            0 => (),
            1 => for i in 0..w as i32 {
                for j in 0..h as i32 {
                    self.draw(x + i, y + j, sprite.get_pixel(i + ox, j + oy));
            }}
            // I didn't test this code, but it probably works.
            _ => for i in 0..sprite.width() as i32 {
                for j in 0..sprite.height() as i32 {
                    let scale = scale as i32;
                    for is in 0..scale {
                        for js in 0..scale {
                            self.draw(x + (i * scale) + is, y + (j * scale) + js, sprite.get_pixel(i + ox, j + oy));
            }}}}
        }
    }

    #[inline]
    fn _draw_string_scaled(&mut self, x: i32, y: i32, text: &String, p: Pixel, scale: u32) {
        let mut sx = 0;
        let mut sy = 0;
        let m = self.get_pixel_mode();

        if p.a != 255 { self.set_pixel_mode(PixelMode::Alpha) }
        else { self.set_pixel_mode(PixelMode::Mask) }

        for c in text.bytes() {
            if c == b'\n' {
                sx = 0; sy += 8 * scale;
            }
            else {
                if scale > 1 {
                    for i in 0..8 {
                        for j in 0..8 {
                            let index = (c as usize * 64) + (j * 8) + i;
                            if FONT_DATA[index] == 1 {
                                for is in 0..scale {
                                    for js in 0..scale {
                                        let px = x + (sx + (i as u32 * scale) + is) as i32;
                                        let py = y + (sy + (j as u32 * scale) + js) as i32;
                                        self.draw(px, py, p);
                }}}}}}
                else {
                    for i in 0..8 {
                        for j in 0..8 {
                            let index = (c as usize * 64) + (j * 8) + i;
                            if FONT_DATA[index] == 1 {
                                self.draw(x + sx as i32 + i as i32, y + sy as i32 + j as i32, p);
                }}}}
                sx += 8 * scale;
            }
        }

        self.set_pixel_mode(m);
    }

    #[inline]
    fn _clear(&mut self, p: Pixel) {
        let mut draw_target = self.draw_target.borrow_mut();
        
        for y in 0..draw_target.height() {
            for x in 0..draw_target.width() {
                draw_target.set_pixel(x as i32, y as i32, p);
            }
        }
    }

    fn _update_keys(&mut self) {
        self.keyboard_state = [ HWButton::default(); Key::Count as usize];

        for key in self.window.get_keys().unwrap() {
            self.keyboard_state[KEYMAP[key as usize] as usize].held = true;
        }

        for key in self.window.get_keys_pressed(minifb::KeyRepeat::No).unwrap() {
            self.keyboard_state[KEYMAP[key as usize] as usize].pressed = true;
        }

        for key in self.window.get_keys_released().unwrap() {
            self.keyboard_state[KEYMAP[key as usize] as usize].released = true;
        }

        self.keyboard_state[Key::Alt as usize] = HWButton {
            pressed: self.keyboard_state[Key::LeftAlt as usize].pressed | self.keyboard_state[Key::RightAlt as usize].pressed,
            released: self.keyboard_state[Key::LeftAlt as usize].released | self.keyboard_state[Key::RightAlt as usize].released,
            held: self.keyboard_state[Key::LeftAlt as usize].held | self.keyboard_state[Key::RightAlt as usize].held
        };

        self.keyboard_state[Key::Shift as usize] = HWButton {
            pressed: self.keyboard_state[Key::LeftShift as usize].pressed | self.keyboard_state[Key::RightShift as usize].pressed,
            released: self.keyboard_state[Key::LeftShift as usize].released | self.keyboard_state[Key::RightShift as usize].released,
            held: self.keyboard_state[Key::LeftShift as usize].held | self.keyboard_state[Key::RightShift as usize].held
        };

        self.keyboard_state[Key::Control as usize] = HWButton {
            pressed: self.keyboard_state[Key::LeftControl as usize].pressed | self.keyboard_state[Key::RightControl as usize].pressed,
            released: self.keyboard_state[Key::LeftControl as usize].released | self.keyboard_state[Key::RightControl as usize].released,
            held: self.keyboard_state[Key::LeftControl as usize].held | self.keyboard_state[Key::RightControl as usize].held
        };

        self.keyboard_state[Key::System as usize] = HWButton {
            pressed: self.keyboard_state[Key::LeftSystem as usize].pressed | self.keyboard_state[Key::RightSystem as usize].pressed,
            released: self.keyboard_state[Key::LeftSystem as usize].released | self.keyboard_state[Key::RightSystem as usize].released,
            held: self.keyboard_state[Key::LeftSystem as usize].held | self.keyboard_state[Key::RightSystem as usize].held
        };

        self.keyboard_state[Key::Enter as usize] = HWButton {
            pressed: self.keyboard_state[Key::Return as usize].pressed | self.keyboard_state[Key::NumPadEnter as usize].pressed,
            released: self.keyboard_state[Key::Return as usize].released | self.keyboard_state[Key::NumPadEnter as usize].released,
            held: self.keyboard_state[Key::Return as usize].held | self.keyboard_state[Key::NumPadEnter as usize].held
        };
    }

    fn _update_mouse(&mut self) {
        use minifb::{MouseButton, MouseMode};

        if let Some((x, y)) = self.window.get_mouse_pos(MouseMode::Discard) {
            self.mouse_pos_x = x as i32 / self.pixel_w as i32;
            self.mouse_pos_y = y as i32 / self.pixel_h as i32;
        }

        for (i, button) in [MouseButton::Left, MouseButton::Right, MouseButton::Middle].iter().enumerate() {
            let old = self.mouse_state[i];
            let current = self.window.get_mouse_down(*button);
            self.mouse_state[i] = HWButton {
                held: current & (old.pressed | old.held),
                pressed: current & !(old.pressed | old.held),
                released: !current & (old.pressed | old.held)
            }
        }

        if let Some((_, y)) = self.window.get_scroll_wheel() {
            self.mouse_wheel_delta = y as i32;
        }
        else {
            self.mouse_wheel_delta = 0;
        }
    }

    fn _update_window(&mut self) -> minifb::Result<()> {
        let frame_buffer = self.default_draw_target.borrow_mut();
        self.window.update_with_buffer(frame_buffer.get_data(), self.screen_w, self.screen_h)
    }
}

pub struct PixelGameEngineContext<App: PGEApplication> {
    app: App,
    engine: PixelGameEngine,

    frame_instant: Instant,
    frame_timer: f32,
    frame_count: u32,
}

impl<App: PGEApplication> PixelGameEngineContext<App> {
    fn core_update(&mut self) {
        // handle when the window closes
        if !self.engine.window.is_open() {
            self.engine.active = false;
            return
        }

        // handle timing
        let elapsed_time = self.frame_instant.elapsed().as_micros() as f32 / 1_000_000.0;
        self.frame_instant = Instant::now();

        // handle hardware
        self.engine._update_mouse();
        self.engine._update_keys();

        if !self.app.on_user_update(&mut self.engine, elapsed_time) {
            self.engine.active = false;
        }

        if let Err(_) = self.engine._update_window() {
            self.engine.active = false;
        }

        // update title bar with fps
        self.frame_timer += elapsed_time;
        self.frame_count += 1;

        if self.frame_timer >= 1.0 {
            self.frame_timer -= 1.0;
            self.engine.window.set_title(
                format!("OneLoneCoder.com - Pixel Game Engine - {} - FPS: {}",
                App::APP_NAME,
                self.frame_count).as_str()
            );
            self.frame_count = 0;
        }
    }

    pub fn start(&mut self) -> RCode {
        if !self.app.on_user_create(&mut self.engine) {
            return RCode::Fail
        }

        while self.engine.active {
            while self.engine.active {
                self.core_update()
            }

            if !self.app.on_user_destroy() && self.engine.window.is_open() {
                self.engine.active = true
            }
        }

        RCode::Ok
    }
}

impl PixelGameEngine {
    pub fn construct<App: PGEApplication>(app: App, width: usize, height: usize, pixel_width: usize, pixel_height: usize) -> PixelGameEngineContext<App> {
        let frame_buffer = Sprite::new(width as u32, height as u32).into_ref();
        
        let mut context = PixelGameEngineContext {
            engine: PixelGameEngine {
                screen_w: width,
                screen_h: height,
                pixel_w: pixel_width,
                pixel_h: pixel_height,
    
                mouse_pos_x: 300,
                mouse_pos_y: 200,
                mouse_wheel_delta: 0,
    
                mouse_state: [ HWButton::default(); 5],
                keyboard_state: [ HWButton::default(); Key::Count as usize ],
                
                active: true,
                default_draw_target: frame_buffer.clone(),
                draw_target: frame_buffer,

                pixel_mode: PixelMode::Normal,
                blend_factor: 1.0,
    
                window: minifb::Window::new(
                    App::APP_NAME,
                    width * pixel_width, height * pixel_height,
                    minifb::WindowOptions {
                        borderless: false,
                        title: true,
                        resize: false,
                        scale: minifb::Scale::X1,
                        scale_mode: minifb::ScaleMode::Stretch,
                        topmost: false,
                        transparency: false,
                        none: false
                    }
                ).unwrap_or_else(|e| {
                    panic!("{}", e)
                }),
            },
            app,

            frame_instant: Instant::now(),
            frame_timer: 0.0,
            frame_count: 0,
        };

        context.engine.window.limit_update_rate(None);

        context
    }
}