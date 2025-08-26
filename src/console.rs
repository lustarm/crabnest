use std::io::{self, Write};

use crossterm::{cursor, event, execute, terminal};
use event::{ KeyCode, KeyModifiers };

pub struct Console {
    buffer: Vec<char>
}

impl Console {
    pub fn new() -> Self {
        terminal::enable_raw_mode().unwrap();
        return Self { buffer: Vec::new() }
    }

    pub fn clear(&self) -> io::Result<()> {
        execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))?;
        execute!(io::stdout(), cursor::MoveTo(0, 0))?;
        return Ok(())
    }

    pub fn read(mut self) -> io::Result<()> {
        loop {
            match event::read()? {
                event::Event::Key(event) => {
                    match event.code {
                        // TODO: Add up and down keystrokes
                        KeyCode::Char(c) => {
                            if c == 'c' && event.modifiers.contains(KeyModifiers::CONTROL) {
                                println!("\r");
                                println!("Bye!\r");
                                break;
                            }

                            self.buffer.push(c);
                            print!("{}", c);
                            io::stdout().flush()?;
                        },
                        KeyCode::Backspace => {
                            if self.buffer.len() < 1 { continue; }
                            self.buffer.pop();
                        },
                        KeyCode::Enter => {
                            self.buffer.clear();
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        terminal::disable_raw_mode()?;
        return Ok(());
    }

    pub fn get_buffer(self) -> Vec<char> {
        return self.buffer;
    }
}
