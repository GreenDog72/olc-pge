use olc_pge as olc;

const STAR_COLOURS: [u32; 8] = [
    0xffffffff, 0xffffffd9, 0xffffffa3, 0xffc8c8ff,
    0xff9dcbff, 0xffff9f9f, 0xffff5e41, 0xff9d1928
];

pub struct Planet {
    distance: f64,
    diameter: f64,
    foliage: f64,
    minerals: f64,
    water: f64,
    gases: f64,
    temperature: f64,
    population: f64,
    ring: bool,
    moons: Vec<f64>,
}

impl Default for Planet {
    fn default() -> Self { Self {
        distance: 0.0,
        diameter: 0.0,
        foliage: 0.0,
        minerals: 0.0,
        water: 0.0,
        gases: 0.0,
        temperature: 0.0,
        population: 0.0,
        ring: false,
        moons: Vec::new(),
    }}
}

pub struct StarSystem {
    lehmer: u32,
    star_exists: bool,
    star_diameter: f64,
    star_colour: olc::Pixel,
    planets: Vec<Planet>,
}

impl StarSystem {
    pub fn new(x: u32, y: u32, generate_full_system: bool) -> Self {
        let mut this = Self {
            lehmer: (x & 0xffff) << 16 | (y & 0xffff),
            star_exists: false,
            star_diameter: 0.0,
            star_colour: olc::WHITE,
            planets: Vec::new(),
        };

        this.star_exists = this.rand_i32(0, 20) == 1;

        if this.star_exists {
            this.star_diameter = this.rand_f64(10.0, 40.0);
            this.star_colour = STAR_COLOURS[this.rand_i32(0, 8) as usize].into();

            if generate_full_system {
                let mut distance_from_star = this.rand_f64(60.0, 200.0);
                let planets = this.rand_i32(0, 10);
                for _ in 0..planets {
                    let mut p = Planet::default();
                    p.distance = distance_from_star;
                    distance_from_star += this.rand_f64(20.0, 200.0);
                    p.diameter = this.rand_f64(4.0, 20.0);
                    p.temperature = this.rand_f64(-200.0, 300.0);
                    p.foliage = this.rand_f64(0.0, 1.0);
                    p.minerals = this.rand_f64(0.0, 1.0);
                    p.gases = this.rand_f64(0.0, 1.0);
                    p.water = this.rand_f64(0.0, 1.0);
                    let sum = 1.0 / (p.foliage + p.minerals + p.gases + p.water);
                    p.foliage *= sum;
                    p.minerals *= sum;
                    p.gases *= sum;
                    p.water *= sum;

                    p.population = this.rand_f64(-5000000.0, 20000000.0).max(0.0);

                    p.ring = this.rand_i32(0, 10) == 1;

                    let moons = this.rand_i32(-5, 5).max(0);

                    for _ in 0..moons {
                        p.moons.push(this.rand_f64(1.0, 5.0));
                    }

                    this.planets.push(p);
                }
            }
        }

        this
    }

    fn lehmer32(&mut self) -> u32 {
        self.lehmer += 0xe120fc15;
        let tmp = self.lehmer as u64 * 0x4a39b70d;
        let m1: u32 = ((tmp >> 32) ^ tmp) as u32;
        let tmp = m1 as u64 * 0x12fad5c9;
        let m2 = (tmp >> 32) ^ tmp;
        m2 as u32
    }

    fn rand_i32(&mut self, min: i32, max: i32) -> i32 {
        (self.lehmer32() % (max - min) as u32) as i32 + min
    }

    fn rand_f64(&mut self, min: f64, max: f64) -> f64 {
        (self.lehmer32() as f64 / (0x7fffffff as f64)) * (max - min) + min
    }
}

pub struct Galaxy {
    galaxy_offset: olc::Vf2d,
    star_selected: bool,
    star_selected_pos: olc::Vi2d,
}

impl Default for Galaxy {
    fn default() -> Self { Self {
        galaxy_offset: (0.0, 0.0).into(),
        star_selected: false,
        star_selected_pos: (0, 0).into(),
    } }
}

impl olc::PGEApplication for Galaxy {
    const APP_NAME: &'static str = "olcGalaxy - Rust";

    fn on_user_update(&mut self, pge: &mut olc::PixelGameEngine, elapsed_time: f32) -> bool {
        if pge.get_key(olc::Key::W).held { self.galaxy_offset.y -= 50.0 * elapsed_time; }
        if pge.get_key(olc::Key::S).held { self.galaxy_offset.y += 50.0 * elapsed_time; }
        if pge.get_key(olc::Key::A).held { self.galaxy_offset.x -= 50.0 * elapsed_time; }
        if pge.get_key(olc::Key::D).held { self.galaxy_offset.x += 50.0 * elapsed_time; }

        pge.clear(olc::BLACK);

        let sectors_x = pge.screen_width() / 16;
        let sectors_y = pge.screen_height() / 16;

        let mouse = olc::Vi2d::new(pge.get_mouse_x() / 16, pge.get_mouse_y() / 16);
        let galaxy_mouse = mouse + olc::Vi2d::from(self.galaxy_offset);

        for x in 0..sectors_x as u32 {
            for y in 0..sectors_y as u32 {
                let pos = self.galaxy_offset + (x as f32, y as f32);
                let star = StarSystem::new(pos.x as u32, pos.y as u32, false);

                if star.star_exists {
                    pge.fill_circle(x as i32 * 16 + 8, y as i32 * 16 + 8,
                        star.star_diameter as i32 / 8, star.star_colour);

                    if mouse.x == x as i32 && mouse.y == y as i32 {
                        pge.draw_circle(x as i32 * 16 + 8, y as i32 * 16 + 8, 12, olc::YELLOW);
                    }
                }
            }
        }

        if pge.get_mouse(0).pressed {
            let star = StarSystem::new(galaxy_mouse.x as u32, galaxy_mouse.y as u32, false);

            if star.star_exists {
                self.star_selected = true;
                self.star_selected_pos = galaxy_mouse;
            }
            else {
                self.star_selected = false;
            }
        }

        if self.star_selected {
            let star = StarSystem::new(self.star_selected_pos.x as u32, self.star_selected_pos.y as u32, true);

            // Draw Window
            pge.fill_rect(8, 240, 496, 232, olc::DARK_BLUE);
            pge.draw_rect(8, 240, 496, 232, olc::WHITE);

            // Draw Star
            let mut body = olc::Vi2d::new(14, 356);
            body.x += (star.star_diameter * 1.375) as i32;
            pge.fill_circle_v(body, (star.star_diameter * 1.375) as i32, star.star_colour);
            body.x += (star.star_diameter * 1.375) as i32 + 8;

            // Draw Planets
            for planet in star.planets {
                if body.x + planet.diameter as i32 >= 496 { break; }

                body.x += planet.diameter as i32;
                pge.fill_circle_v(body, planet.diameter as i32, olc::RED);

                let mut moon_pos = body;
                moon_pos.y += planet.diameter as i32 + 10;

                for moon in planet.moons {
                    moon_pos.y += moon as i32;
                    pge.fill_circle_v(moon_pos, moon as i32, olc::GREY);
                    moon_pos.y += moon as i32 + 10;
                }

                body.x += planet.diameter as i32 + 8;
            }
        }

        true
    }
}

fn main() {
    olc::PixelGameEngine::construct(Galaxy::default(), 512, 480, 2, 2).start();
}