use crossterm::{event::KeyCode, terminal};
use specs::prelude::*;
use specs_derive::Component;

use crate::{components::position::Position, ctx::Ctx, map::Map, State};

#[derive(Component, Debug)]
pub struct Player {}

pub fn get_camera_pos(gs: &State) -> Option<Position> {
    let positions = gs.ecs.write_storage::<Position>();
    let players = gs.ecs.write_storage::<Player>();

    let t_size = terminal::size().unwrap();

    // let a = Position {x: t_size.0 as isize / 2, y: t_size.1 as isize / 2};

    for (_player, pos) in (&players, &positions).join() {
        // return Some(*pos + Position {x: t_size.0 as isize, y: t_size.1 as isize} / 2);
        return Some(Position {x: pos.x - t_size.0 as isize / 2, y: pos.y - t_size.1 as isize / 2});
    }

    None
}

pub fn try_move_player(delta_pos: Position, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    let map = ecs.fetch::<Map>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let new_pos = *pos + delta_pos;

        if let Some(tile) = map.get_tile_at(new_pos) {
            if !tile.passable() {
                return;
            }
        }

        pos.x = new_pos.x;
        pos.y = new_pos.y;
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Ctx) {
    // Player movement
    for key in ctx.input_handler.get_key_states() {
        if key.1 == false {
            continue;
        }

        match key.0 {
            KeyCode::Left => try_move_player(Position { x: -1, y: 0 }, &mut gs.ecs),
            KeyCode::Right => try_move_player(Position { x: 1, y: 0 }, &mut gs.ecs),
            KeyCode::Up => try_move_player(Position { x: 0, y: 1 }, &mut gs.ecs),
            KeyCode::Down => try_move_player(Position { x: 0, y: -1 }, &mut gs.ecs),

            KeyCode::Char('a') => try_move_player(Position { x: -1, y: 0 }, &mut gs.ecs),
            KeyCode::Char('d') => try_move_player(Position { x: 1, y: 0 }, &mut gs.ecs),
            KeyCode::Char('w') => try_move_player(Position { x: 0, y: 1 }, &mut gs.ecs),
            KeyCode::Char('s') => try_move_player(Position { x: 0, y: -1 }, &mut gs.ecs),

            KeyCode::Char('h') => try_move_player(Position { x: -1, y: 0 }, &mut gs.ecs),
            KeyCode::Char('l') => try_move_player(Position { x: 1, y: 0 }, &mut gs.ecs),
            KeyCode::Char('k') => try_move_player(Position { x: 0, y: 1 }, &mut gs.ecs),
            KeyCode::Char('j') => try_move_player(Position { x: 0, y: -1 }, &mut gs.ecs),

            _ => {}
        }
    }
}
