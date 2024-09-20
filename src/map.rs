use std::collections::HashMap;

use crate::{
    components::{position::Position, renderable::Renderable},
    ctx::Ctx,
    utils::color::Color,
};

pub enum Tile {
    Wall,
    Floor,
}

impl Tile {
    pub fn to_renderable(&self) -> Renderable {
        match self {
            Tile::Wall => Renderable {
                glyph: '#',
                fg: Color::Black,
                bg: Color::Default,
            },
            Tile::Floor => Renderable {
                glyph: '·',
                fg: Color::Black,
                bg: Color::Default,
            },
        }
    }

    pub fn passable(&self) -> bool {
        match self {
            Tile::Wall => false,
            Tile::Floor => true,
        }
    }
}

pub struct Map {
    tiles: HashMap<Position, Tile>,
}

impl Map {
    pub fn new_map() -> Self {
        let mut tiles: HashMap<Position, Tile> = HashMap::new();

        for x in 0..100 {
            for y in 0..30 {
                let pos = Position { x, y };
                let tile = Tile::Floor;

                tiles.insert(pos, tile);
            }
        }

        Map { tiles }
    }

    pub fn draw_map(&self, ctx: &mut Ctx) {
        for (pos, tile) in &self.tiles {
            ctx.cam.buffer.push((*pos, tile.to_renderable()));
        }
    }

    pub fn get_tile_at(&self, pos: Position) -> Option<&Tile> {
        self.tiles.get(&pos)
    }
}
