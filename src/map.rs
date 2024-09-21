use std::collections::HashMap;
use bracket_lib::prelude::{field_of_view, Algorithm2D, BaseMap, Point};
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
    revealed_tiles: HashMap<Position, Tile>,
}

impl Map {
    fn new(tiles: HashMap<Position, Tile>) -> Self {
        Map {
            tiles: tiles,
            revealed_tiles: HashMap::new()
        }
    }

    pub fn new_map() -> Self {
        let mut tiles: HashMap<Position, Tile> = HashMap::new();

        for x in 0..100 {
            for y in 0..30 {
                let pos = Position { x, y };
                let tile = Tile::Floor;

                tiles.insert(pos, tile);
            }
        }

        Map::new(tiles)
    }

    pub fn draw_map(&self, ctx: &mut Ctx) {
        for (pos, tile) in &self.revealed_tiles {
            ctx.set(pos, &tile.to_renderable());
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


        (rooms[0].center(), Map::new(tiles))
    }

    pub fn reveal_tile(&mut self, new_pos: Position) {
        let tile = self.tiles.get(&new_pos);

        if let Some(tile) = tile {
            self.revealed_tiles.insert(new_pos, *tile);
        }
    }

    pub fn reveal_fov(&mut self, player_pos: Position) {
        const VISION_RADIUS: i32 = 20;
        let player_point = Point::new(player_pos.x, player_pos.y);
        
        // Use the field_of_view function to get visible tiles
        let fov_tiles: Vec<Point> = field_of_view(player_point, VISION_RADIUS, self);
        // Reveal each tile in the FOV
        for p in fov_tiles {
            self.reveal_tile(Position { x: p.x as isize, y: p.y as isize});
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        // Return the width and height of the map. Adjust these values based on your map size.
        Point::new(160, 100) // Example: map size 160x100
    }
    fn point2d_to_index(&self, pt: Point) -> usize {
        // If using a fixed width grid, convert 2D coordinates to a 1D index
        (pt.y * 160 + pt.x) as usize // Assuming a map width of 160
    }

    fn index_to_point2d(&self, idx: usize) -> Point {
        // Convert a 1D index back into a 2D coordinate
        let x = (idx % 160) as isize;
        let y = (idx / 160) as isize;
        Point::new(x, y)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        let pos = self.index_to_point2d(idx);
        match self.get_tile_at(Position { x: pos.x as isize, y: pos.y as isize}) {
            Some(Tile::Wall) => true,  // Walls are opaque
            _ => false,                // Floors and others are transparent
        }
    }

}