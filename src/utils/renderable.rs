use std::fmt;

use specs::prelude::*;
use specs_derive::Component;

use super::color::Color;

#[derive(Component, Copy, Clone)]
pub struct Renderable {
    pub glyph: char,
    pub fg: Color,
    pub bg: Color,
}

impl fmt::Display for Renderable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fg_code = self.fg.to_code();
        let bg_code = self.bg.to_code();

        write!(f, "\x1b[3{};4{}m{}\x1b[0m", fg_code, bg_code, self.glyph)
    }
}