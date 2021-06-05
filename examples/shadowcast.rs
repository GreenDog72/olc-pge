use olc_pge as olc;

struct Edge {
    sx: f32, sy: f32, // start coordinate
    ex: f32, ey: f32  // end coordinate
}

#[derive(Copy, Clone)]
struct Cell {
    edge_id: [usize; Direction::COUNT],
    edge_exist: [bool; Direction::COUNT],
    exist: bool
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            edge_id: [0, 0, 0, 0],
            edge_exist: [false, false, false, false],
            exist: false
        }
    }
}

enum Direction {
    North, South, East, West
}
use Direction::*;

impl Direction { const COUNT: usize = West as usize + 1; }

impl<T> std::ops::Index<Direction> for [T] {
    type Output = T; fn index(&self, index: Direction) -> &Self::Output { &self[index as usize] } }
impl<T> std::ops::IndexMut<Direction> for [T] {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output { &mut self[index as usize] } }

struct ShadowCasting2D {
    world: Vec<Cell>,
    world_width: usize,
    world_height: usize,
    light_cast: olc::SpriteRef,
    buff_light_ray: olc::SpriteRef,
    buff_light_tex: olc::SpriteRef,
    edges: Vec<Edge>,
    visibility_polygon_points: Vec<(f32, f32, f32)> // (angle, x, y)
}

impl ShadowCasting2D {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            world: vec![Cell::default(); width * height],
            world_width: width,
            world_height: height,
            light_cast: olc::Sprite::load_file("graphics/light_cast.png").into_ref(),
            buff_light_ray: olc::Sprite::default().into_ref(),
            buff_light_tex: olc::Sprite::default().into_ref(),
            edges: vec![],
            visibility_polygon_points: vec![]
        }
    }

    fn convert_tile_map_to_poly_map(&mut self, sx: usize, sy: usize, w: usize, h: usize, block_width: usize, pitch: usize) {
        // clear poly map
        self.edges.clear();

        for x in 0..w {
            for y in 0..h {
                for j in 0..4 {
                    self.world[(y + sy) * pitch + (x + sx)].edge_exist[j] = false;
        }}}

        // iterate through region from top left to bottom right
        for x in 1 .. w - 1 {
            for y in 1 .. h - 1 {
                let i = (y + sy) * pitch + x + sx;
                
                // if this cell exists, check if it needs edges
                if self.world[i].exist {
                    let n = (y + sy - 1) * pitch + x + sx;
                    let s = (y + sy + 1) * pitch + x + sx;
                    let w = (y + sy) * pitch + x + sx - 1;
                    let e = (y + sy) * pitch + x + sx + 1;

                    // if this cell has no wetern neighbor, it needs a western edge
                    if !self.world[w].exist {
                        // it can either extend it from its northern neighbor if they have one, or start a new one
                        if self.world[n].edge_exist[West] {
                            // northern neighbor has western edge, so grow it down
                            self.edges[self.world[n].edge_id[West]].ey += block_width as f32;
                            self.world[i].edge_id[West] = self.world[n].edge_id[West];
                            self.world[i].edge_exist[West] = true;
                        }
                        else {
                            // northern neighbor does not have one, so create one
                            let sx = (sx + x) * block_width;
                            let sy = (sy + y) * block_width;

                            let edge = Edge {
                                sx: sx as f32,
                                sy: sy as f32,
                                ex: sx as f32,
                                ey: (sy + block_width) as f32
                            };

                            // add edge to pool
                            let edge_id = self.edges.len();
                            self.edges.push(edge);

                            // update tile info with edge info
                            self.world[i].edge_id[West] = edge_id;
                            self.world[i].edge_exist[West] = true;
                        }
                    }

                    // if this cell has no eastern neighbor, it needs a eastern edge
                    if !self.world[e].exist {
                        // it can either extend it from its northern neighbor if they have one, or start a new one
                        if self.world[n].edge_exist[East] {
                            // northern neighbor has western edge, so grow it down
                            self.edges[self.world[n].edge_id[East]].ey += block_width as f32;
                            self.world[i].edge_id[East] = self.world[n].edge_id[East];
                            self.world[i].edge_exist[East] = true;
                        }
                        else {
                            // northern neighbor does not have one, so create one
                            let sx = (sx + x + 1) * block_width;
                            let sy = (sy + y) * block_width;

                            let edge = Edge {
                                sx: sx as f32,
                                sy: sy as f32,
                                ex: sx as f32,
                                ey: (sy + block_width) as f32
                            };

                            // add edge to pool
                            let edge_id = self.edges.len();
                            self.edges.push(edge);

                            // update tile info with edge info
                            self.world[i].edge_id[East] = edge_id;
                            self.world[i].edge_exist[East] = true;
                        }
                    }

                    // if this cell has no northern neighbor, it needs a northern edge
                    if !self.world[n].exist {
                        // it can either extend it from its western neighbor if they have one, or start a new one
                        if self.world[w].edge_exist[North] {
                            // western neighbor has northern edge, so grow it east
                            self.edges[self.world[w].edge_id[North]].ex += block_width as f32;
                            self.world[i].edge_id[North] = self.world[w].edge_id[North];
                            self.world[i].edge_exist[North] = true;
                        }
                        else {
                            // western neighbor does not have one, so create one
                            let sx = (sx + x) * block_width;
                            let sy = (sy + y) * block_width;

                            let edge = Edge {
                                sx: sx as f32,
                                sy: sy as f32,
                                ex: (sx + block_width) as f32,
                                ey: sy as f32
                            };

                            // add edge to pool
                            let edge_id = self.edges.len();
                            self.edges.push(edge);

                            // update tile info with edge info
                            self.world[i].edge_id[North] = edge_id;
                            self.world[i].edge_exist[North] = true;
                        }
                    }

                    // if this cell has no southern neighbor, it needs a southern edge
                    if !self.world[s].exist {
                        // it can either extend it from its western neighbor if they have one, or start a new one
                        if self.world[w].edge_exist[South] {
                            // western neighbor has southern edge, so grow it east
                            self.edges[self.world[w].edge_id[South]].ex += block_width as f32;
                            self.world[i].edge_id[South] = self.world[w].edge_id[South];
                            self.world[i].edge_exist[South] = true;
                        }
                        else {
                            // western neighbor does not have one, so create one
                            let sx = (sx + x) * block_width;
                            let sy = (sy + y + 1) * block_width;

                            let edge = Edge {
                                sx: sx as f32,
                                sy: sy as f32,
                                ex: (sx + block_width) as f32,
                                ey: sy as f32
                            };

                            // add edge to pool
                            let edge_id = self.edges.len();
                            self.edges.push(edge);

                            // update tile info with edge info
                            self.world[i].edge_id[South] = edge_id;
                            self.world[i].edge_exist[South] = true;
                        }
                    }
                }
            }
        }
    }

    fn calculate_visibility_polygon(&mut self, ox: f32, oy: f32, radius: f32) {
        self.visibility_polygon_points.clear();

        // for each edge in the poly map
        for e in self.edges.iter() {
            // take the start point, then the end point
            // we could use a pool of non-duplicated points here, it would be more optimal
            for i in 0..2 {
                let (mut rdx, mut rdy) =
                    if i == 0 { (e.sx - ox, e.sy - oy) }
                    else { (e.ex - ox, e.ey - oy) };
                
                let base_ang = rdy.atan2(rdx);

                // for each point, cast 3 rays, 1 direct, 1 to either side
                for ang in [base_ang - 0.0001, base_ang, base_ang + 0.0001].iter() {
                    // create ray along angle for required distance
                    rdx = radius * ang.cos();
                    rdy = radius * ang.sin();

                    let mut min_t1 = std::f32::INFINITY;
                    let (mut min_px, mut min_py, mut min_ang) = (0.0, 0.0, 0.0);
                    let mut valid = false;

                    // check for ray intersection with all edges
                    for e in self.edges.iter() {
                        // create line segment vector
                        let (sdx, sdy) = (e.ex - e.sx, e.ey - e.sy);

                        if (sdx - rdx).abs() > 0.0 && (sdy - rdy).abs() > 0.0 {
                            // t2 is normalised distance from line segment start to line segment end of intersect point
                            let t2 = (rdx * (e.sy - oy) + (rdy * (ox - e.sx))) / (sdx * rdy - sdy * rdx);
                            // t1 is normalised distance from source along ray to ray length of intersect point
                            let t1 = (e.sx + sdx * t2 - ox) / rdx;

                            // if intersect point exists along ray, and along line segment, then intersect is valid
                            if t1 > 0.0 && t2 >= 0.0 && t2 <= 1.0 {
                                // check if this intersect point is closest to source
                                // if it is, then store this point and refect others
                                if t1 < min_t1 {
                                    min_t1 = t1;
                                    min_px = ox + rdx * t1;
                                    min_py = oy + rdy * t1;
                                    min_ang = (min_py - oy).atan2(min_px - ox);
                                    valid = true;
                                }
                            }
                        }
                    }

                    if valid { // add intersection point to visibility polygon perimeter
                        self.visibility_polygon_points.push((min_ang, min_px, min_py));
                    }
                }
            }
        }

        // sort perimeter points by angle from source
        // this will allow us to draw a triangle fan
        self.visibility_polygon_points.sort_by(|a, b| {
            if a.0 == b.0 { std::cmp::Ordering::Equal }
            else if a.0 < b.0 { std::cmp::Ordering::Less }
            else { std::cmp::Ordering::Greater }
        });
    }
}

impl olc::PGEApplication for ShadowCasting2D {
    const APP_NAME: &'static str = "ShadowCasting2D - Rust";

    fn on_user_create(&mut self, pge: &mut olc::PixelGameEngine) -> bool {
        // add a boundary to the world
        for x in 1..(self.world_width - 1) {
            self.world[self.world_width + x].exist = true;
            self.world[(self.world_height - 2) * self.world_width + x].exist = true;
        }

        for y in 1..(self.world_height - 1) {
            self.world[y * self.world_width + 1].exist = true;
            self.world[y * self.world_width + (self.world_width - 2)].exist = true;
        }

        self.buff_light_ray = olc::Sprite::new(pge.screen_width() as u32, pge.screen_height() as u32).into_ref();
        self.buff_light_tex = olc::Sprite::new(pge.screen_width() as u32, pge.screen_height() as u32).into_ref();

        true
    }

    fn on_user_update(&mut self, pge: &mut olc::PixelGameEngine, _elapsed_time: f32) -> bool {
        let block_width = 16;
        let source_x = pge.get_mouse_x();
        let source_y = pge.get_mouse_y();

        // set tile map blocks to on or off
        if pge.get_mouse(0).released {
            // i = y * width + x
            let i = (source_y as usize / block_width) * self.world_width + (source_x as usize / block_width);
            self.world[i].exist = !self.world[i].exist;
        }

        // take a region of tile map and convert it to poly map
        // this is done every frame here, but could be a pre-processing stage
        // depending on how your final application interacts with tilemaps
        self.convert_tile_map_to_poly_map(0, 0, self.world_width, self.world_height, block_width, self.world_width);

        if pge.get_mouse(1).held {
            self.calculate_visibility_polygon(source_x as f32, source_y as f32, 1000.0);
        }

        // drawing
        pge.clear(olc::BLACK);

        let rays_cast = self.visibility_polygon_points.len();

        // remove duplicates (or similar) points from polygon
        let mut new_points: Vec<(f32, f32, f32)> = vec![];
        for t1 in self.visibility_polygon_points.iter() {
            let mut unique = true;
            for t2 in new_points.iter() {
                if (t1.1 - t2.1).abs() < 0.1 && (t1.2 - t2.2).abs() < 0.1 {
                    unique = false;
                }
            }
            if unique {
                new_points.push(t1.clone());
            }
        }
        self.visibility_polygon_points = new_points;

        let rays_drawn = self.visibility_polygon_points.len();

        pge.draw_string(4, 4, &format!("Rays Cast: {} Rays Drawn: {}", rays_cast, rays_drawn), olc::WHITE);

        // if drawing rays, use an offscreen sprite as out render target
        if pge.get_mouse(1).held && self.visibility_polygon_points.len() > 1 {
            pge.set_draw_target(Some(self.buff_light_tex.clone()));

            // clear offscreen buffer for sprite
            pge.clear(olc::BLACK);

            // draw radial light sprite to buffer
            // centered around source location (mouse coordinates)
            pge.draw_sprite(source_x - 255, source_y - 255, self.light_cast.clone());

            pge.set_draw_target(Some(self.buff_light_ray.clone()));

            // clear offscreen buffer for rays
            pge.clear(olc::BLANK);

            // draw each triangle in fan
            for i in 0 .. self.visibility_polygon_points.len() - 1 {
                pge.fill_triangle(
                    source_x, source_y,
                    self.visibility_polygon_points[i].1 as i32, self.visibility_polygon_points[i].2 as i32,
                    self.visibility_polygon_points[i + 1].1 as i32, self.visibility_polygon_points[i + 1].2 as i32,
                    olc::WHITE
                );
            }

            // Fan will have one open edge, so draw last point of fan to first
            pge.fill_triangle(
                source_x, source_y,

                self.visibility_polygon_points[self.visibility_polygon_points.len() - 1].1 as i32,
                self.visibility_polygon_points[self.visibility_polygon_points.len() - 1].2 as i32,

                self.visibility_polygon_points[0].1 as i32,
                self.visibility_polygon_points[0].2 as i32,

                olc::WHITE
            );

            pge.set_draw_target(None);

            // Wherever rays exist in ray sprite, copy over radial light sprite pixels
            let buff_light_tex = self.buff_light_tex.borrow();
            let buff_light_ray = self.buff_light_ray.borrow();

            for x in 0..pge.screen_width() as i32 {
                for y in 0..pge.screen_height() as i32 {
                    if buff_light_ray.get_pixel(x, y).r > 0 {
                        pge.draw(x, y, buff_light_tex.get_pixel(x, y));
                    }
                }
            }
        }

        let block_width = block_width as i32;

        // draw blocks from tile map
        for x in 0..self.world_width as i32 {
            for y in 0..self.world_height as i32 {
                if self.world[y as usize * self.world_width + x as usize].exist {
                    pge.fill_rect(x * block_width, y * block_width, block_width as u32, block_width as u32, olc::BLUE);
                }
            }
        }

        // draw edges from poly map
        for e in self.edges.iter() {
            pge.draw_line(e.sx as i32, e.sy as i32, e.ex as i32, e.ey as i32, olc::WHITE);
            pge.fill_circle(e.sx as i32, e.sy as i32, 3, olc::RED);
            pge.fill_circle(e.ex as i32, e.ey as i32, 3, olc::RED);
        }

        true
    }
}

fn main() {
    let app = ShadowCasting2D::new(40, 30);
    olc::PixelGameEngine::construct(app, 640, 480, 2, 2).start();
}