use olc_pge as olc;

use rand::Rng;

pub struct Example;

impl olc::PGEApplication for Example {
    const APP_NAME: &'static str = "Example - Rust";

    fn on_user_update(&mut self, pge: &mut olc::PixelGameEngine, _: f32) -> bool {
        let mut rng = rand::thread_rng();

        for x in 0..pge.screen_width() as i32 {
            for y in 0..pge.screen_height() as i32 {
                pge.draw(x, y, olc::Pixel::rgb(rng.gen(), rng.gen(), rng.gen()));
            }
        }

        true
    }
}

fn main() {
    olc::PixelGameEngine::construct(Example, 256, 240, 4, 4).start();
}