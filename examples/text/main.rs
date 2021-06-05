//#![windows_subsystem = "windows"]

use olc_pge as olc;

pub struct TextEditor {
    bg_color: olc::Pixel,
    text_color: olc::Pixel,
    format_color: olc::Pixel,
    cursor_color: olc::Pixel,

    cursor_blink_state: bool,
    cursor_blink_rate: f32,
    cursor_blink_timer: f32,

    current_line: usize,
    
    insertion_pos: usize, // position in the string to insert characters
    cursor_pos: u32,      // on screen position, in characters, on the current line
    shadow_pos: u32,      // when moving up and down, the hidden position

    tab_width: u32,

    lines: Vec<String>,

    needs_redraw: bool,
}

impl Default for TextEditor {
    fn default() -> Self {
        Self {
            bg_color:     olc::Pixel::rgb(20, 17, 23),
            text_color:   olc::Pixel::rgb(130, 220, 90),
            format_color: olc::Pixel::rgb(64, 64, 64),
            cursor_color: olc::Pixel::rgb(100, 150, 230),

            cursor_blink_state: false,
            cursor_blink_rate: 0.3,
            cursor_blink_timer: 0.0,

            current_line: 0,

            insertion_pos: 0,
            cursor_pos: 0,
            shadow_pos: 0,

            tab_width: 4,

            lines: vec!["".into()],

            needs_redraw: true,
        }
    }
}

impl TextEditor {
    fn calc_insertion_pos(&mut self) {
        let line = &self.lines[self.current_line];

        let mut pos = 0;
        self.insertion_pos = 0;

        for c in line.chars() {
            if c == '\t' {
                pos += self.tab_width - (pos % self.tab_width);
            }
            else {
                pos += 1;
            }

            if pos > self.shadow_pos {
                break;
            }

            self.insertion_pos += 1;
        }
    }

    fn calc_cursor_pos(&mut self, set_shadow: bool) {
        let line = &self.lines[self.current_line];

        self.cursor_pos = 0;

        for (i, c) in line.chars().enumerate() {
            if i >= self.insertion_pos { 
                break;
            }
            if c == '\t' {
                self.cursor_pos += self.tab_width - (self.cursor_pos % self.tab_width);
            }
            else {
                self.cursor_pos += 1;
            }
        }

        if set_shadow {
            self.shadow_pos = self.cursor_pos;
        }
    }

    fn insert_char(&mut self, c: char) {
        let line = &mut self.lines[self.current_line as usize];
        if self.insertion_pos < line.len() {
            line.insert(self.insertion_pos as usize, c);
        }
        else {
            line.push(c);
        }
        self.insertion_pos += 1;
        self.calc_cursor_pos(true);
        self.needs_redraw = true;
    }

    fn remove_char(&mut self) {
        if self.insertion_pos < self.lines[self.current_line as usize].len() {
            self.lines[self.current_line as usize].remove(self.insertion_pos as usize);
        }
        else {
            if self.current_line < self.lines.len() - 1 {
                let other = self.lines.remove(self.current_line as usize + 1);
                self.lines[self.current_line as usize].push_str(&other);
            }
        }
        self.calc_cursor_pos(true);
        self.needs_redraw = true;
    }

    fn insert_new_line(&mut self) {
        if self.insertion_pos < self.lines[self.current_line as usize].len() {
            if self.insertion_pos == 0 {
                self.lines.insert(self.current_line as usize, "".into());
                self.insertion_pos = 0;
                self.cursor_pos = 0;
            }
            else {
                let new = self.lines[self.current_line as usize].split_off(self.insertion_pos);
                self.current_line += 1;
                self.insertion_pos = 0;
                self.cursor_pos = 0;
                self.lines.insert(self.current_line as usize, new);
            }
        }
        else {
            self.current_line += 1;
            self.insertion_pos = 0;
            self.cursor_pos = 0;
            self.lines.insert(self.current_line as usize, "".into());
        }
        self.needs_redraw = true;
    }

    fn move_cursor_backward(&mut self) {
        if self.insertion_pos == 0 {
            if self.current_line != 0 {
                self.current_line -= 1;
                self.insertion_pos = self.lines[self.current_line as usize].len();
            }
        }
        else {
            self.insertion_pos -= 1;
        }
        self.calc_cursor_pos(true);
        self.reset_cursor_blink();
    }

    fn move_cursor_forward(&mut self) {
        if self.current_line < self.lines.len() {
            if self.insertion_pos < self.lines[self.current_line as usize].len() {
                self.insertion_pos += 1;
            }
            else {
                if self.current_line < self.lines.len() - 1 {
                    self.current_line += 1;
                    self.insertion_pos = 0;
                }
            }
        }
        self.calc_cursor_pos(true);
        self.reset_cursor_blink();
    }

    fn reset_cursor_blink(&mut self) {
        self.cursor_blink_state = true;
        self.cursor_blink_timer = 0.0;
        self.needs_redraw = true;
    }

    fn calc_draw_length(&self, text: &String) -> u32 {
        let mut pos = 0;
        for c in text.chars() {
            if c == '\t' {
                let tab_size = self.tab_width - (pos % self.tab_width);
                pos += tab_size;
            }
            else {
                pos += 1;
            }
        }
        pos
    }

    fn draw_char(pge: &mut olc::PixelGameEngine, x: u32, y: u32, c: char, color: olc::Pixel) {
        pge.draw_string(8 * x as i32, 1 + 9 * y as i32, &c.to_string(), color);
    }

    fn draw_text(&self, pge: &mut olc::PixelGameEngine, y: i32, text: &String) {
        let mut pos = 0;
        for c in text.chars() {
            if c == '\t' {
                TextEditor::draw_char(pge, pos, y as u32, '>', self.format_color);
                pos += self.tab_width - (pos % self.tab_width);
            }
            else {
                TextEditor::draw_char(pge, pos, y as u32, c, self.text_color);
                pos += 1;
            }
        }
    }

    fn draw_format_char(&self, pge: &mut olc::PixelGameEngine, x: i32, y: i32, c: char) {
        TextEditor::draw_char(pge, x as u32, y as u32, c, self.format_color);
    }

    fn draw_cursor(&self, pge: &mut olc::PixelGameEngine) {
        pge.fill_rect(8 * self.cursor_pos as i32, 9 * self.current_line as i32, 2, 9, self.cursor_color);
    }

    fn draw(&self, pge: &mut olc::PixelGameEngine) {
        pge.clear(self.bg_color);

        if self.cursor_blink_state {
            self.draw_cursor(pge);
        }

        for i in 0..30 {
            if i < self.lines.len() {
                self.draw_text(pge, i as i32, &self.lines[i]);
                self.draw_format_char(pge, self.calc_draw_length(&self.lines[i]) as i32, i as i32, '<');
            }
            else {
                self.draw_format_char(pge, 0, i as i32, '~');
            }
        }

        
    }
}

const PRINTABLE_KEYS: [olc::Key; 64] = [
    olc::Key::A, olc::Key::B, olc::Key::C, olc::Key::D, olc::Key::E, olc::Key::F, olc::Key::G, olc::Key::H,
    olc::Key::I, olc::Key::J, olc::Key::K, olc::Key::L, olc::Key::M, olc::Key::N, olc::Key::O, olc::Key::P,
    olc::Key::Q, olc::Key::R, olc::Key::S, olc::Key::T, olc::Key::U, olc::Key::V, olc::Key::W, olc::Key::X,
    olc::Key::Y, olc::Key::Z,
    olc::Key::K0, olc::Key::K1, olc::Key::K2, olc::Key::K3, olc::Key::K4, olc::Key::K5, olc::Key::K6, olc::Key::K7,
    olc::Key::K8, olc::Key::K9, olc::Key::NumPad0, olc::Key::NumPad1, olc::Key::NumPad2, olc::Key::NumPad3,
    olc::Key::NumPad4, olc::Key::NumPad5, olc::Key::NumPad6, olc::Key::NumPad7, olc::Key::NumPad8, olc::Key::NumPad9,
    olc::Key::Tab, olc::Key::Space,
    olc::Key::BackQuote, olc::Key::Minus, olc::Key::Equal, olc::Key::LeftBracket, olc::Key::RightBracket,
    olc::Key::BackSlash, olc::Key::Semicolon, olc::Key::Apostrophe, olc::Key::Comma, olc::Key::Period,
    olc::Key::Slash,
    olc::Key::NumPadDiv, olc::Key::NumPadMul, olc::Key::NumPadSub, olc::Key::NumPadAdd, olc::Key::NumPadDecimal
];

fn key_to_char(key: olc::Key, shift: bool) -> Result<char, ()> {
    if !PRINTABLE_KEYS.contains(&key) {
        Err(())
    }
    else {
        use olc::Key::*;
        if shift {
            Ok(match key {
                A => 'A', B => 'B', C => 'C', D => 'D', E => 'E', F => 'F', G => 'G', H => 'H', I => 'I', J => 'J',
                K => 'K', L => 'L', M => 'M', N => 'N', O => 'O', P => 'P', Q => 'Q', R => 'R', S => 'S', T => 'T',
                U => 'U', V => 'V', W => 'W', X => 'X', Y => 'Y', Z => 'Z',
                K0 => ')', K1 => '!', K2 => '@', K3 => '#', K4 => '$',
                K5 => '%', K6 => '^', K7 => '&', K8 => '*', K9 => '(',
                NumPad0 => '0', NumPad1 => '1', NumPad2 => '2', NumPad3 => '3', NumPad4 => '4',
                NumPad5 => '5', NumPad6 => '6', NumPad7 => '7', NumPad8 => '8', NumPad9 => '9',
                Tab => '\t', Space => ' ',
                BackQuote => '~', Minus => '_', Equal => '+', LeftBracket => '{', RightBracket => '}',
                BackSlash => '|', Semicolon => ':', Apostrophe => '"', Comma => '<', Period => '>', Slash => '?',
                NumPadDiv => '/', NumPadMul => '*', NumPadSub => '-', NumPadAdd => '+', NumPadDecimal => '.',
                _ => '\0'
            })
        }
        else {
            Ok(match key {
                A => 'a', B => 'b', C => 'c', D => 'd', E => 'e', F => 'f', G => 'g', H => 'h', I => 'i', J => 'j',
                K => 'k', L => 'l', M => 'm', N => 'n', O => 'o', P => 'p', Q => 'q', R => 'r', S => 's', T => 't',
                U => 'u', V => 'v', W => 'w', X => 'x', Y => 'y', Z => 'z',
                K0 => '0', K1 => '1', K2 => '2', K3 => '3', K4 => '4',
                K5 => '5', K6 => '6', K7 => '7', K8 => '8', K9 => '9',
                NumPad0 => '0', NumPad1 => '1', NumPad2 => '2', NumPad3 => '3', NumPad4 => '4',
                NumPad5 => '5', NumPad6 => '6', NumPad7 => '7', NumPad8 => '8', NumPad9 => '9',
                Tab => '\t', Space => ' ',
                BackQuote => '`', Minus => '-', Equal => '=', LeftBracket => '[', RightBracket => ']',
                BackSlash => '\\', Semicolon => ';', Apostrophe => '\'', Comma => ',', Period => '.', Slash => '/',
                NumPadDiv => '/', NumPadMul => '*', NumPadSub => '-', NumPadAdd => '+', NumPadDecimal => '.',
                _ => '\0'
            })
        }
    }
}

impl olc::PGEApplication for TextEditor {
    const APP_NAME: &'static str = "Text Editor - Rust";

    fn on_user_update(&mut self, pge: &mut olc::PixelGameEngine, elapsed_time: f32) -> bool {
        self.cursor_blink_timer += elapsed_time;
        if self.cursor_blink_timer >= self.cursor_blink_rate {
            while self.cursor_blink_timer >= self.cursor_blink_rate {
                self.cursor_blink_timer -= self.cursor_blink_rate;
            }
            self.cursor_blink_state = !self.cursor_blink_state;
            self.needs_redraw = true;
        }

        if pge.get_key(olc::Key::Up).pressed {
            if self.current_line == 0 {
                // if we're at the top, set insertion at the beginning, update cursor and shadow
                self.insertion_pos = 0;
                self.calc_cursor_pos(true);
            }
            else {
                // if we're not, calc the insertion and cursor pos from the shadow
                self.current_line -= 1;
                self.calc_insertion_pos();
                self.calc_cursor_pos(false);
            }
            self.reset_cursor_blink();
        }

        if pge.get_key(olc::Key::Down).pressed {
            if self.current_line < self.lines.len() {
                if self.current_line == self.lines.len() - 1 {
                    // if we're at the last line, set insertion at the end, update cursor and shadow
                    self.insertion_pos = self.lines[self.current_line as usize].len();
                    self.calc_cursor_pos(true);
                }
                else {
                    // if we're not, calc the insertion and cursor pos from the shadow
                    self.current_line += 1;
                    self.calc_insertion_pos();
                    self.calc_cursor_pos(false);
                }
                self.reset_cursor_blink();
            }
        }

        if pge.get_key(olc::Key::Left).pressed {
            self.move_cursor_backward();
        }

        if pge.get_key(olc::Key::Right).pressed {
            self.move_cursor_forward();
        }

        if pge.get_key(olc::Key::Delete).pressed {
            self.remove_char();
        }

        if pge.get_key(olc::Key::Back).pressed {
            if !(self.current_line == 0 && self.insertion_pos == 0) {
                self.move_cursor_backward();
                self.remove_char();
            }
        }

        if pge.get_key(olc::Key::Enter).pressed {
            self.insert_new_line();
        }

        let shift = pge.get_key(olc::Key::Shift);

        for key in PRINTABLE_KEYS.iter() {
            let key = *key;
            if pge.get_key(key).pressed {
                self.insert_char(key_to_char(key, shift.pressed | shift.held).unwrap());
            }
        }

        if self.needs_redraw { self.draw(pge); self.needs_redraw = false; }

        true
    }
}

fn main() {
    olc::PixelGameEngine::construct(TextEditor::default(), 8 * 60, 9 * 30, 2, 2).start();
}