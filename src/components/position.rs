use std::ops::{Add, Sub};

use specs::prelude::*;
use specs_derive::Component;
#[derive(Component, Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
