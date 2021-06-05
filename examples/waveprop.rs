use olc_pge as olc;

use std::collections::HashSet;

struct PathFindingFlowFields {
    map_width: i32,
    map_height: i32,
    cell_size: i32,
    border_width: i32,
    obstacle_map: Vec<bool>,
    flow_field_z: Vec<i32>,
    flow_field_y: Vec<f32>,
    flow_field_x: Vec<f32>,
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
    wave: i32,
}

impl Default for PathFindingFlowFields {
    fn default() -> Self {
        PathFindingFlowFields {
            map_width: 0,
            map_height: 0,
            cell_size: 32,
            border_width: 4,
            obstacle_map: Vec::new(),
            flow_field_z: Vec::new(),
            flow_field_y: Vec::new(),
            flow_field_x: Vec::new(),
            start_x: 3,
            start_y: 7,
            end_x: 12,
            end_y: 7,
            wave: 1,
        }
    }
}

impl olc::PGEApplication for PathFindingFlowFields {
    const APP_NAME: &'static str = "PathFinding - Flow Fields - Rust";

    fn on_user_create(&mut self, pge: &mut olc::PixelGameEngine) -> bool {
        self.map_width = pge.screen_width() as i32 / self.cell_size;
        self.map_height = pge.screen_height() as i32 / self.cell_size;

        let size = (self.map_width * self.map_height) as usize;

        self.obstacle_map = vec![false; size];
        self.flow_field_z = vec![0; size];
        self.flow_field_y = vec![0.0; size];
        self.flow_field_x = vec![0.0; size];

        true
    }

    fn on_user_update(&mut self, pge: &mut olc::PixelGameEngine, _elapsed_time: f32) -> bool {
        let map_width = self.map_width;
        let p = |x: i32, y: i32| -> usize { (y * map_width + x) as usize };

        let selected_cell_x = pge.get_mouse_x() / self.cell_size;
        let selected_cell_y = pge.get_mouse_y() / self.cell_size;

        if pge.get_mouse(0).released {
            let index = p(selected_cell_x, selected_cell_y);
            self.obstacle_map[index] = !self.obstacle_map[index];
        }

        if pge.get_mouse(1).released {
            self.start_x = selected_cell_x;
            self.start_y = selected_cell_y;
        }

        if pge.get_key(olc::Key::Q).released {
            self.wave += 1;
        }

        if pge.get_key(olc::Key::A).released {
            self.wave -= 1;
            if self.wave == 0 {
                self.wave = 1;
            }
        }

        for x in 0..self.map_width {
            for y in 0..self.map_height {
                if x == 0 || y == 0 || x == self.map_width - 1 || y == self.map_height - 1 || self.obstacle_map[p(x, y)] {
                    self.flow_field_z[p(x, y)] = -1;
                }
                else {
                    self.flow_field_z[p(x, y)] = 0;
                }
            }
        }

        let mut nodes = HashSet::new();

        nodes.insert((self.end_x, self.end_y, 1));

        let mut is_empty = nodes.is_empty();

        while !is_empty {
            let mut new_nodes = HashSet::new();

            for n in nodes.iter() {
                let x = n.0;
                let y = n.1;
                let d = n.2;

                self.flow_field_z[p(x, y)] = d;

                if (x + 1) < self.map_width && self.flow_field_z[p(x + 1, y)] == 0 {
                    new_nodes.insert((x + 1, y, d + 1));
                }

                if (x - 1) >= 0 && self.flow_field_z[p(x - 1, y)] == 0 {
                    new_nodes.insert((x - 1, y, d + 1));
                }

                if (y + 1) < self.map_height && self.flow_field_z[p(x, y + 1)] == 0 {
                    new_nodes.insert((x, y + 1, d + 1));
                }

                if (y - 1) >= 0 && self.flow_field_z[p(x, y - 1)] == 0 {
                    new_nodes.insert((x, y - 1, d + 1));
                }
            }

            nodes = new_nodes;

            is_empty = nodes.is_empty();
        }

        let mut path = Vec::new();

        path.push((self.start_x, self.start_y));

        let mut loc_x = self.start_x;
        let mut loc_y = self.start_y;

        let mut no_path = false;

        while !(loc_x == self.end_x && loc_y == self.end_y) && !no_path {
            let mut neighbors = Vec::new();

            // 4 way connectivity
            if (loc_y - 1) >= 0 && self.flow_field_z[p(loc_x, loc_y - 1)] > 0 {
                neighbors.push((loc_x, loc_y - 1, self.flow_field_z[p(loc_x, loc_y - 1)])); }

            if (loc_x + 1) < self.map_width && self.flow_field_z[p(loc_x + 1, loc_y)] > 0 {
                neighbors.push((loc_x + 1, loc_y, self.flow_field_z[p(loc_x + 1, loc_y)])); }

            if (loc_y + 1) < self.map_height && self.flow_field_z[p(loc_x, loc_y + 1)] > 0 {
                neighbors.push((loc_x, loc_y + 1, self.flow_field_z[p(loc_x, loc_y + 1)])); }

            if (loc_x - 1) >= 0 && self.flow_field_z[p(loc_x - 1, loc_y)] > 0 {
                neighbors.push((loc_x - 1, loc_y, self.flow_field_z[p(loc_x - 1, loc_y)])); }

            // 8 way connectivity
            if (loc_y - 1) >= 0 && (loc_x - 1) >= 0 && self.flow_field_z[p(loc_x - 1, loc_y - 1)] > 0 {
                neighbors.push((loc_x - 1, loc_y - 1, self.flow_field_z[p(loc_x - 1, loc_y - 1)])); }

            if (loc_y - 1) >= 0 && (loc_x + 1) < self.map_width && self.flow_field_z[p(loc_x + 1, loc_y - 1)] > 0 {
                neighbors.push((loc_x + 1, loc_y - 1, self.flow_field_z[p(loc_x + 1, loc_y - 1)])); }

            if (loc_y + 1) < self.map_height && (loc_x - 1) >= 0 && self.flow_field_z[p(loc_x - 1, loc_y + 1)] > 0 {
                neighbors.push((loc_x - 1, loc_y + 1, self.flow_field_z[p(loc_x - 1, loc_y + 1)])); }

            if (loc_y + 1) < self.map_height && (loc_x + 1) < self.map_width && self.flow_field_z[p(loc_x + 1, loc_y + 1)] > 0 {
                neighbors.push((loc_x + 1, loc_y + 1, self.flow_field_z[p(loc_x + 1, loc_y + 1)])); }
            
            // a slight change in logic here
            if neighbors.is_empty() {
                no_path = true;
            }
            else {
                let mut lowest = neighbors[0];
                for n in neighbors.iter().skip(1) {
                    if n.2 < lowest.2 { lowest = *n; } }
                loc_x = lowest.0;
                loc_y = lowest.1;
                path.push((loc_x, loc_y));
            }
        }

        for x in 1..self.map_width - 1 {
            for y in 1..self.map_height - 1 {
                let mut vx = 0.0;
                let mut vy = 0.0;

                vy -= if self.flow_field_z[p(x, y + 1)] <= 0 { self.flow_field_z[p(x, y)] } else { self.flow_field_z[p(x, y + 1)] - self.flow_field_z[p(x, y)] } as f32;
                vx -= if self.flow_field_z[p(x + 1, y)] <= 0 { self.flow_field_z[p(x, y)] } else { self.flow_field_z[p(x + 1, y)] - self.flow_field_z[p(x, y)] } as f32;
                vy += if self.flow_field_z[p(x, y - 1)] <= 0 { self.flow_field_z[p(x, y)] } else { self.flow_field_z[p(x, y - 1)] - self.flow_field_z[p(x, y)] } as f32;
                vx += if self.flow_field_z[p(x - 1, y)] <= 0 { self.flow_field_z[p(x, y)] } else { self.flow_field_z[p(x - 1, y)] - self.flow_field_z[p(x, y)] } as f32;

                let r = 1.0 / (vx * vx + vy * vy).sqrt();

                self.flow_field_x[p(x, y)] = vx * r;
                self.flow_field_y[p(x, y)] = vy * r;
            }
        }

        // Draw map
        pge.clear(olc::BLACK);

        for x in 0..self.map_width {
            for y in 0..self.map_height {
                let mut colour = olc::BLUE;

                if self.obstacle_map[p(x, y)] { colour = olc::GREY; }
                if self.wave == self.flow_field_z[p(x, y)] { colour = olc::DARK_CYAN; }
                if x == self.start_x && y == self.start_y { colour = olc::GREEN; }
                if x == self.end_x && y == self.end_y { colour = olc::RED; }

                let size = (self.cell_size - self.border_width) as u32;

                pge.fill_rect(x * self.cell_size, y * self.cell_size, size, size, colour);

                if self.flow_field_z[p(x, y)] > 0 {
                    let angle = self.flow_field_y[p(x, y)].atan2(self.flow_field_x[p(x, y)]);
                    let radius = (self.cell_size - self.border_width) as f32 / 2.0;
                    let offset_x = (x * self.cell_size + (self.cell_size - self.border_width) / 2) as f32;
                    let offset_y = (y * self.cell_size + (self.cell_size - self.border_width) / 2) as f32;

                    let a0 = (angle.cos() * radius + offset_x, angle.sin() * radius + offset_y);
                    let a1 = (angle.cos() * -radius + offset_x, angle.sin() * -radius + offset_y);
                    let a2 = ((angle + 0.1).cos() * radius * 0.7 + offset_x, (angle + 0.1).sin() * radius * 0.7 + offset_y);
                    let a3 = ((angle - 0.1).cos() * radius * 0.7 + offset_x, (angle - 0.1).sin() * radius * 0.7 + offset_y);

                    let a0 = (a0.0 as i32, a0.1 as i32);
                    let a1 = (a1.0 as i32, a1.1 as i32);
                    let a2 = (a2.0 as i32, a2.1 as i32);
                    let a3 = (a3.0 as i32, a3.1 as i32);

                    pge.draw_line(a0.0, a0.1, a1.0, a1.1, olc::CYAN);
                    pge.draw_line(a0.0, a0.1, a2.0, a2.1, olc::CYAN);
                    pge.draw_line(a0.0, a0.1, a3.0, a3.1, olc::CYAN);
                }
            }
        }

        let (mut ox, mut oy) = path[0];

        for a in path.iter().skip(1) {
            pge.draw_line(
                ox * self.cell_size + (self.cell_size - self.border_width) / 2,
                oy * self.cell_size + (self.cell_size - self.border_width) / 2,
                a.0 * self.cell_size + (self.cell_size - self.border_width) / 2,
                a.1 * self.cell_size + (self.cell_size - self.border_width) / 2, olc::YELLOW );
            
            ox = a.0;
            oy = a.1;

            pge.fill_circle(
                ox * self.cell_size + (self.cell_size - self.border_width) / 2,
                oy * self.cell_size + (self.cell_size - self.border_width) / 2,
                10, olc::YELLOW);
        }

        true
    }
}

fn main() {
    let app = PathFindingFlowFields::default();
    olc::PixelGameEngine::construct(app, 512, 480, 2, 2).start();
}