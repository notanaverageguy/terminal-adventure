use camera::Camera;
use components::{position::Position, renderable::Renderable};
use ctx::Ctx;
use map::Map;
use player::Player;
use specs::prelude::*;
use specs_derive::Component;

pub mod camera;
pub mod components;
pub mod ctx;
pub mod player;
pub mod utils;
pub mod map;

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
        {
            let map = self.ecs.fetch::<Map>();
            map.draw_map(ctx);
        }

        player::player_input(self, ctx);
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
    gs.ecs.register::<Player>();

    gs.ecs
        .create_entity()
        .with(Position { x: 10, y: 0 })
        .with(Renderable {
            glyph: '☺',
            fg: utils::color::Color::Green,
            bg: utils::color::Color::Default,
        })
        .with(Player {})
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

    gs.ecs.insert(Map::new_map());

    Camera::load_terminal_settings();
    context.main_loop(gs);

    Camera::deload_terminal_settings();
}
