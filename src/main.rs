use camera::Camera;
use ctx::Ctx;
use specs::prelude::*;
use utils::renderable::Renderable;

mod camera;
mod utils;
mod ctx;

use crate::utils::position::Position;

pub trait GameState {
    fn tick(&mut self, ctx: &mut Ctx);
}

pub struct State {
    ecs: World
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Ctx) {

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos, render);

        }

    }
}

fn main() {

    let mut gs: State = State {
        ecs: World::new()
    };

    let mut context: Ctx = Ctx::new();

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();

    gs.ecs.create_entity()
        .with(Position{x: 0, y: 0})
        .with(Renderable{
            glyph: '☺',
            fg: utils::color::Color::Green,
            bg: utils::color::Color::Default
        })
        .build();


    Camera::load_terminal_settings();
    context.main_loop(gs);

    Camera::deload_terminal_settings();
}