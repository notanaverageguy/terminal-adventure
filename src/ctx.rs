use std::{
    thread,
    time::{Duration, Instant},
};

use crossterm::terminal;

use crate::{
    camera::Camera,
    components::{position::Position, renderable::Renderable},
    utils::input_handler::InputHandler,
    GameState, State,
};

const FPS: u64 = 20; // Desired frames per second
const FRAME_DURATION: Duration = Duration::from_millis(1000 / FPS);

pub struct Ctx {
    pub cam: Camera,
    pub input_handler: InputHandler,
}

impl Ctx {
    pub fn new() -> Self {
        Ctx {
            cam: Camera::new(),
            input_handler: InputHandler::new(),
        }
    }

    pub fn main_loop(&mut self, mut gs: State) {
        let mut last_frame_time = Instant::now();

        self.input_handler.start();

        '_game_loop: loop {
            let now = Instant::now();

            gs.tick(self);

            self.cam.render();

            // Calculate elapsed time for this frame
            let elapsed = now.duration_since(last_frame_time);
            // Calculate sleep duration to maintain FPS
            let sleep_duration = FRAME_DURATION.saturating_sub(elapsed);
            // Sleep for the calculated duration
            thread::sleep(sleep_duration);

            // Update the last frame time
            last_frame_time = Instant::now();
        }
    }

    pub fn set(&mut self, pos: &Position, renderable: &Renderable) {
        self.cam.buffer.push((*pos, *renderable));
    }

    pub fn cls(&self) {
        print!("{esc}c", esc = 27 as char);
    }

    pub fn get_terminal_size() -> (u16, u16) {
        terminal::size().unwrap()
    }
}
