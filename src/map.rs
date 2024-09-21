use std::collections::HashMap;

use rand::Rng;

use crate::{
    components::{position::Position, renderable::Renderable},
    ctx::Ctx,
    utils::{color::Color, rectangle::Rectangle},
};

#[derive(Debug, Clone, Copy)]
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

    fn apple_horizontal_line (tiles: &mut HashMap<Position, Tile>, p1: Position, x2: isize, tile :Tile) {
        for x in p1.x.min(x2)..=p1.x.max(x2) {
            let pos = Position {x, y: p1.y};
            tiles.insert(pos, tile);

            let pos = Position {x, y: p1.y + 1};
            tiles.insert(pos, tile);

        }
    }

    fn apple_vertical_line(tiles: &mut HashMap<Position, Tile>, p1: Position, y2: isize, tile :Tile) {
        for y in p1.y.min(y2)..=p1.y.max(y2) {
            let pos = Position{x: p1.x, y};
            tiles.insert(pos, tile);

            let pos = Position{x: p1.x + 1, y};
            tiles.insert(pos, tile);
        }
    }

    pub fn new_dungeon_floor() -> (Position, Self) {
        let mut tiles: HashMap<Position, Tile> = HashMap::new();

        const GENERATION_OPPORTUNITIES: i32 = 60;
        const MIN_ROOM_SIZE: isize = 6;
        const MAX_ROOM_SIZE: isize = 30;
        const DUNGEON_SIZE: (isize, isize) = (160, 100);

        {
            // Fill insides
            for x in 0..DUNGEON_SIZE.0 {
                for y in 0..DUNGEON_SIZE.1 {
                    let pos = Position { x, y };
                    tiles.insert(pos, Tile::Wall);
                }
            }
        }

        let mut rng = rand::thread_rng();
        let mut rooms: Vec<Rectangle> = Vec::new();

        'room_gen: for _ in 0..GENERATION_OPPORTUNITIES {

            let x = rng.gen_range(1..DUNGEON_SIZE.0) as isize;
            let y = rng.gen_range(1..DUNGEON_SIZE.1) as isize;
            let w = rng.gen_range(MIN_ROOM_SIZE..MAX_ROOM_SIZE + 3); // Bias towards being longer instead of taller
            let h = rng.gen_range(MIN_ROOM_SIZE..MAX_ROOM_SIZE);

            let new_room = Rectangle::new(Position { x, y }, Position { x: x + w, y: y + h });

            if new_room.p2.x >= DUNGEON_SIZE.0 || new_room.p2.y >= DUNGEON_SIZE.1 {
                continue 'room_gen;
            }

            for other_room in rooms.iter() {
                if new_room.intersect(other_room) {
                    continue 'room_gen;
                };
            }

            if !rooms.is_empty() {
                let new_center = new_room.center();
                let prev_center = rooms.last().unwrap().center();
            
                // Step 1: Draw vertical line from prev_center to new_center's Y
                Self::apple_vertical_line(&mut tiles, prev_center, new_center.y, Tile::Floor);
                
                // Step 2: Then draw horizontal line from the end of the vertical line to new_center's X
                let intermediate_position = Position { x: prev_center.x, y: new_center.y };
                Self::apple_horizontal_line(&mut tiles, intermediate_position, new_center.x, Tile::Floor);
            }


            rooms.push(new_room);
        }

        for room in rooms.iter() {
            for x in room.p1.x..room.p2.x {

                for y in room.p1.y..room.p2.y {
                    let pos = Position { x, y };
                    tiles.insert(pos, Tile::Floor);
                }
            }
        }


        (rooms[0].center(), Map { tiles })
    }
}
