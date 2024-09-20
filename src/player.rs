use crossterm::event::KeyCode;
use specs::prelude::*;
use specs_derive::Component;

use crate::{components::position::Position, ctx::Ctx, map::Map, State};

#[derive(Component, Debug)]
pub struct Player {}

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
            _ => {}
        }
    }
}
