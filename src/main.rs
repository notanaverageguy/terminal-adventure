use camera::Camera;
use ctx::Ctx;
use specs::prelude::*;
use specs_derive::Component;
use utils::renderable::Renderable;

mod camera;
mod ctx;
mod utils;

use crate::utils::position::Position;

pub trait GameState {
    fn tick(&mut self, ctx: &mut Ctx);
}

pub struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker {};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Ctx) {
        ctx.cls();

        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos, render);
        }
    }
}

#[derive(Component)]
struct LeftMover {}

#[derive(Component)]
struct LeftWalker {}

impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
        }
    }
}

fn main() {
    let mut gs: State = State { ecs: World::new() };

    let mut context: Ctx = Ctx::new();

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();

    gs.ecs
        .create_entity()
        .with(Position { x: 10, y: 0 })
        .with(Renderable {
            glyph: '☺',
            fg: utils::color::Color::Green,
            bg: utils::color::Color::Default,
        })
        .build();

    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position { x: 7 * i, y: 20 })
            .with(Renderable {
                glyph: '@',
                fg: utils::color::Color::Red,
                bg: utils::color::Color::Default,
            })
            .with(LeftMover {})
            .build();
    }

    Camera::load_terminal_settings();
    context.main_loop(gs);

    Camera::deload_terminal_settings();
}
