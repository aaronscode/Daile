use tcod::colors::*;
use tcod::console::*;

type Glyph = char;
#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub glyph: char,
    pub foreground_color: Color,
    pub background_color: Color,
}

impl Tile {
    pub fn new(glyph: Glyph, foreground_color: Color, background_color: Color) -> Self {
        Self {
            glyph,
            foreground_color,
            background_color,
        }
    }
    pub fn set_glyph(&mut self, glyph: Glyph) {
        self.glyph = glyph;
    }

    pub fn glyph(&self) -> Glyph {
        self.glyph
    }

    pub fn set_color_bkg(&mut self, c: Color) {
        self.background_color = c;
    }

    pub fn set_color_fg(&mut self, c: Color) {
        self.foreground_color = c;
    }

    pub fn set_color(&mut self, fg: Color, bg: Color) {
        self.foreground_color = fg;
        self.background_color = bg;
    }

    pub fn set_glyph_color(&mut self, g: Glyph, fg: Color, bg: Color) {
        self.glyph = g;
        self.foreground_color = fg;
        self.background_color = bg;
    }

    pub fn draw(&self, conn: &mut Offscreen, x: i32, y: i32) {
        conn.put_char_ex(
            x,
            y,
            self.glyph,
            self.foreground_color,
            self.background_color,
        );
    }
}

pub type Map = Vec<Vec<Tile>>;
