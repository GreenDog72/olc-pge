use olc_pge as olc;

struct IsometricDemo {
    world_size: olc::Vi2d,
    tile_size: olc::Vi2d,
    origin: olc::Vi2d,
    spr_isom: olc::SpriteRef,
    world: Vec<u32>,
}

impl Default for IsometricDemo {
    fn default() -> Self {
        IsometricDemo { 
            world_size: olc::Vi2d::new(14, 10),
            tile_size: olc::Vi2d::new(40, 20),
            origin: olc::Vi2d::new(5, 1),
            spr_isom: olc::Sprite::default().into_ref(),
            world: Vec::new(),
        }
    }
}

impl olc::PGEApplication for IsometricDemo {
    const APP_NAME: &'static str = "Coding Quickie: Isometric Tiles - Rust";

    fn on_user_create(&mut self, _pge: &mut olc::PixelGameEngine) -> bool {
        self.spr_isom = olc::Sprite::load_file("graphics/isometric_demo.png").into_ref();
        self.world = vec![0; self.world_size.x as usize * self.world_size.y as usize];

        true
    }

    fn on_user_update(&mut self, pge: &mut olc::PixelGameEngine, _elapsed_time: f32) -> bool {
        pge.clear(olc::WHITE);

        let mouse = olc::Vi2d::new(pge.get_mouse_x(), pge.get_mouse_y());

        let cell = olc::Vi2d::new(mouse.x / self.tile_size.x, mouse.y / self.tile_size.y);

        let offset = olc::Vi2d::new(mouse.x % self.tile_size.x, mouse.y % self.tile_size.y);

        let col = self.spr_isom.borrow().get_pixel(3 * self.tile_size.x + offset.x, offset.y);

        let mut selected = olc::Vi2d::new(
            (cell.y - self.origin.y) + (cell.x - self.origin.x),
            (cell.y - self.origin.y) - (cell.x - self.origin.x)
        );

        if col == olc::RED { selected += olc::Vi2d::new(-1, 0); }
        if col == olc::BLUE { selected += olc::Vi2d::new(0, -1); }
        if col == olc::GREEN { selected += olc::Vi2d::new(0, 1); }
        if col == olc::YELLOW { selected += olc::Vi2d::new(1, 0); }

        if pge.get_mouse(0).pressed {
            if selected.x >= 0 && selected.x < self.world_size.x && selected.y >= 0 && selected.y < self.world_size.y {
                let index = selected.y * self.world_size.x + selected.x;
                self.world[index as usize] = (self.world[index as usize] + 1) % 6;
            }
        }

        let to_screen = |x: i32, y: i32| -> olc::Vi2d {
            olc::Vi2d::new(
                (self.origin.x * self.tile_size.x) + (x - y) * (self.tile_size.x / 2),
                (self.origin.y * self.tile_size.y) + (x + y) * (self.tile_size.y / 2)
            )
        };

        pge.set_pixel_mode(olc::PixelMode::Mask);

        for y in 0..self.world_size.y {
            for x in 0..self.world_size.x {
                let world = to_screen(x, y);
                let tile_id = self.world[(y * self.world_size.x + x) as usize];

                let tall_tile_pos = world - olc::Vi2d { x: 0, y: self.tile_size.y };
                let tall_tile_size = self.tile_size + olc::Vi2d { x: 0, y: self.tile_size.y };

                match tile_id {
                    0 => pge.draw_partial_sprite_v(world, self.spr_isom.clone(), olc::Vi2d::new(1 * self.tile_size.x, 0), self.tile_size),
                    1 => pge.draw_partial_sprite_v(world, self.spr_isom.clone(), olc::Vi2d::new(2 * self.tile_size.x, 0), self.tile_size),
                    2 => pge.draw_partial_sprite_v(tall_tile_pos, self.spr_isom.clone(), olc::Vi2d::new(0 * self.tile_size.x, self.tile_size.y), tall_tile_size),
                    3 => pge.draw_partial_sprite_v(tall_tile_pos, self.spr_isom.clone(), olc::Vi2d::new(1 * self.tile_size.x, self.tile_size.y), tall_tile_size),
                    4 => pge.draw_partial_sprite_v(tall_tile_pos, self.spr_isom.clone(), olc::Vi2d::new(2 * self.tile_size.x, self.tile_size.y), tall_tile_size),
                    5 => pge.draw_partial_sprite_v(tall_tile_pos, self.spr_isom.clone(), olc::Vi2d::new(3 * self.tile_size.x, self.tile_size.y), tall_tile_size),
                    _ => println!("Unknown tile id: {}", tile_id)
                }
            }
        }

        pge.set_pixel_mode(olc::PixelMode::Alpha);

        let selected_world = to_screen(selected.x, selected.y);

        pge.draw_partial_sprite_v(selected_world, self.spr_isom.clone(), olc::Vi2d::new(0, 0), self.tile_size);

        pge.set_pixel_mode(olc::PixelMode::Normal);

        //pge.draw_rect(cell.x * self.tile_size.x, cell.y * self.tile_size.y, self.tile_size.x as u32, self.tile_size.y as u32, olc::RED);

        pge.draw_string(4,  4, &format!("Mouse   : {}, {}", mouse.x, mouse.y), olc::BLACK);
        pge.draw_string(4, 14, &format!("Cell    : {}, {}", cell.x, cell.y), olc::BLACK);
        pge.draw_string(4, 24, &format!("Selected: {}, {}", selected.x, selected.y), olc::BLACK);

        true
    }
}

fn main() {
    let app = IsometricDemo::default();
    olc::PixelGameEngine::construct(app, 512, 480, 2, 2).start();
}