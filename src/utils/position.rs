use std::ops::Sub;

use specs::prelude::*;
use specs_derive::Component;


#[derive(Component, Debug, Copy, Clone)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}