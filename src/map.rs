use crate::{components::{position::Position, renderable::Renderable}, ctx::Ctx, utils::color::Color};

pub enum TileType {
    Wall,
    Floor,
}

pub struct Tile {
    pos: Position,
    r#type: TileType,
}

impl Tile {
    pub fn to_renderable(&self) -> Renderable {
        match self.r#type {
            TileType::Wall => Renderable { glyph: ' ', fg: Color::Black, bg: Color::Black },
            TileType::Floor => Renderable { glyph: '·', fg: Color::Black, bg: Color::Default },
        }
    }
}

pub struct Map {
    tiles: Vec<Tile>
}


impl Map {
    pub fn new_map() -> Self {
        let mut tiles: Vec<Tile> = vec![];

        for x in 0..100 {
            for y in 0..20 {
                let tile = Tile {
                    pos: Position{ x, y},
                    r#type: TileType::Floor
                };

                tiles.push(tile);
            }
        }

        Map {
            tiles
        }
    }

    pub fn draw_map(&self, ctx: &mut Ctx) {
        for tile in &self.tiles {
            ctx.cam.buffer.push( (tile.pos, tile.to_renderable()));
        }
    }
}