use std::{collections::HashMap, io::{stdout, Write}};

use crossterm::{cursor, style::Print, terminal, QueueableCommand};

use crate::{components::{position::Position, renderable::Renderable}, utils::color::Color};

pub struct Camera {
    pub pos: Position,
    pub buffer: Vec<(Position, Renderable)>,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            pos: Position { x: 0, y: 0 },
            buffer: vec![],
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

        // Create a HashMap for all positions with empty pixels
        let mut buffer: HashMap<Position, Renderable> = HashMap::new();

        for x in 0..terminal_size.0 {
            for y in 0..terminal_size.1 {
                let x = x as isize;
                let y = y as isize;
                let pos = Position {x, y};

                let renderable = Renderable {
                    glyph: ' ',
                    fg: Color::Default,
                    bg: Color::Default
                };

                buffer.insert(pos, renderable);

            }
        }

        for (pos, renderable) in &self.buffer {
            let mut adjusted_pos: Position = *pos - self.pos;
            adjusted_pos.y = terminal_size.1 as isize - adjusted_pos.y - 1;
            // adjusted_pos.y = 0;

            if !Self::is_visible(terminal_size, adjusted_pos) {
                continue;
            }

            buffer.insert(adjusted_pos, *renderable);

            // stdout
            //     .queue(cursor::MoveTo(adjusted_pos.x as u16, adjusted_pos.y as u16))
            //     .unwrap();
            // stdout.queue(Print(renderable.to_string())).unwrap();
            // print!("{adjusted_pos:?}");
        }
        // stdout.queue(cursor::MoveTo(0, 0));
        // print!("{:?}", self.pos);

        for (pos, renderable) in buffer {
            stdout
                .queue(cursor::MoveTo(pos.x as u16, pos.y as u16))
                .unwrap();

            stdout.queue(Print(renderable.to_string())).unwrap();

        }

        stdout.flush().unwrap();

        self.buffer.clear();
    }

    pub fn is_visible(terminal_size: (u16, u16), pos: Position) -> bool {
        pos.x >= 0
            && pos.y >= 0
            && pos.x < terminal_size.0 as isize
            && pos.y < terminal_size.1 as isize
    }
}
