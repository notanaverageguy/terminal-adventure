use crate::components::position::Position;

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub p1: Position,
    pub p2: Position,
}

impl Rectangle {
    pub fn new (p1: Position, p2: Position) -> Self {
        Rectangle {p1 , p2}
    }

    pub fn intersect(&self, rhs:&Self) -> bool {
        self.p1.x <= rhs.p2.x && self.p2.x >= rhs.p1.x && self.p1.y <= rhs.p2.y && self.p2.y >= rhs.p1.y
    }

    pub fn center(&self) -> Position {
        Position {
            x: ( self.p1.x + self.p2.x ) / 2,
            y: ( self.p1.y + self.p2.y ) / 2
        }
    }
}