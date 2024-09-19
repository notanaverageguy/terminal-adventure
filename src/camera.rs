use std::io::{stdout, Write};

use crossterm::{cursor, style::Print, terminal, QueueableCommand};

use crate::utils::{position::Position, renderable::Renderable};

pub struct Camera {
    pub pos: Position,
    pub buffer: Vec<(Position, Renderable)>
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            pos: Position {x: 0, y: 0},
            buffer: vec![]
        }
    }

    pub fn load_terminal_settings() {
        let mut stdout = stdout();

        stdout.queue(terminal::EnterAlternateScreen).unwrap();
        stdout.queue(cursor::Hide).unwrap();
        stdout.queue(terminal::DisableLineWrap).unwrap();

        // Ensure all commands are executed
        stdout.flush().unwrap();
    }

    pub fn deload_terminal_settings() {
        let mut stdout = stdout();

        // Leave the alternate screen, restoring the original terminal
        stdout.queue(terminal::LeaveAlternateScreen).unwrap();

        // Ensure all commands are executed
        stdout.flush().unwrap();
    }    

    pub fn render(&mut self) {

        let mut stdout = stdout();
        let terminal_size = terminal::size().unwrap();

        for (pos, renderable) in &self.buffer {

            let mut adjusted_pos: Position = *pos - self.pos;
            adjusted_pos.y += terminal_size.1 as isize - 1;

            if !Self::is_visible(terminal_size, adjusted_pos) {
                return
            }

            stdout.queue(cursor::MoveTo(adjusted_pos.x as u16, adjusted_pos.y as u16)).unwrap();
            stdout.queue(Print(renderable.to_string())).unwrap();
        }

        stdout.flush().unwrap();

        self.buffer.clear();
    }

    pub fn is_visible(terminal_size: (u16, u16), pos: Position) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < terminal_size.0 as isize && pos.y < terminal_size.1 as isize
    }
}