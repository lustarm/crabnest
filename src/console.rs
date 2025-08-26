use std::io;

use crossterm::{event, execute};

pub struct Console {
    buffer: Vec<char>
}

impl Console {
    pub fn new() -> Self {
        return Self { buffer: Vec::new() }
    }

    pub fn read(mut self) -> io::Result<()> {
        execute!(
            io::stdout(),
        )?;

        loop {
            match event::read()? {
                event::Event::Key(event) => {
                    // add to buffer
                    // println!("{:?}", event.code.as_char());
                    match event.code {
                        event::KeyCode::Char(c) => {
                            if c.is_ascii_alphabetic() {
                                self.buffer.push(c);
                            }
                        },
                        event::KeyCode::Backspace => {
                            if self.buffer.len() < 1 { continue; }
                            self.buffer.pop();
                        },
                        event::KeyCode::Enter => {
                            println!("{:?}", self.buffer);
                            self.buffer.clear();
                        },
                        // ignore
                        _ => {}
                    }
                },
                // ignore
                _ => {}
            }
        }
    }

    pub fn get_buffer(self) -> Vec<char> {
        return self.buffer;
    }
}
