use std::ops::{Add, Div, Sub};

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

impl Div<Position> for Position {
    type Output = Position;

    fn div(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Add<isize> for Position {
    type Output = Position;

    fn add(self, rhs: isize) -> Self::Output {
        Position {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Div<isize> for Position {
    type Output = Position;

    fn div(self, rhs: isize) -> Self::Output {
        Position {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}