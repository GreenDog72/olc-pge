use std::rc::Rc;
use std::cell::RefCell;

use crate::pixel::Pixel;

use image::Pixel as i_just_need_the_trait;
use image::io::Reader as ImageReader;

#[allow(dead_code)]
#[derive(Debug)]
enum SpriteMode {
    Normal,
    Periodic
}

#[derive(Debug)]
pub struct Sprite {
    width: u32,
    height: u32,
    data: Vec<u32>,
    mode_sample: SpriteMode,
}

pub type SpriteRef = Rc<RefCell<Sprite>>;

impl Default for Sprite {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            data: vec![],
            mode_sample: SpriteMode::Normal,
        }
    }
}

impl Sprite {
    pub fn new(w: u32, h: u32) -> Self {
        Self {
            width: w,
            height: h,
            data: vec![0; (w * h) as usize],
            mode_sample: SpriteMode::Normal,
        }
    }

    pub fn load_file(file_name: &str) -> Self {
        //panic!("load_file not implemented");

        let img = ImageReader::open(file_name).unwrap().decode().unwrap().into_bgra8();

        let mut sprite = Sprite::new(img.width(), img.height());

        for (i, p) in img.pixels().enumerate() {
            let p = p.channels();
            sprite.data[i] = Pixel::rgba(p[2], p[1], p[0], p[3]).into();
        }

        sprite
    }

    pub fn into_ref(self) -> SpriteRef { Rc::new(RefCell::new(self)) }

    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }

    pub fn get_data(&self) -> &[u32] {
        &self.data
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> Pixel {
        match self.mode_sample {
            SpriteMode::Normal => {
                if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                    Pixel::from(self.data[y as usize * self.width as usize + x as usize])
                }
                else {
                    Pixel::default()
                }
            }
            SpriteMode::Periodic => {
                let index = (y % self.height as i32).abs() as usize * self.width as usize + (x % self.width as i32).abs() as usize;
                Pixel::from(self.data[index])
            }
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, p: Pixel) -> bool {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let p = p.into();
            self.data[(y as usize * self.width as usize) + x as usize] = p;
            return true
        }
        false
    }

    pub fn sample(_x: f32, _y: f32) -> Pixel {
        unimplemented!()
    }

    pub fn sample_bl(_u: f32, _v: f32) -> Pixel {
        unimplemented!()
    }
}
