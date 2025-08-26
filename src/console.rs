use std::io;

use crossterm::{cursor, event, execute, style::{self, Print}, terminal};
use event::{ KeyCode, KeyModifiers };

use crate::command::Commands;

// TODO: Put somewhere else
const PROMPT: &str = "crabnest ~: ";

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
        return Ok(());
    }

    pub fn move_cursor_to_prompt(&self) -> io::Result<()> {
        execute!(
            io::stdout(),
            cursor::MoveRight(
                (PROMPT.len()).try_into().unwrap()
            )
        )?;

        return Ok(());
    }

    pub fn send_prompt(&self) -> io::Result<()> {
        execute!(
            io::stdout(),
            style::Print("\r\n"),
            style::Print(PROMPT.to_owned() + "\r"),
        )?;

        self.move_cursor_to_prompt()?;

        return Ok(());
    }


    pub fn read(mut self, cmds: Commands) -> io::Result<()> {

        self.send_prompt()?;

        loop {
            match event::read()? {
                event::Event::Key(event) => {
                    match event.code {
                        KeyCode::Char(c) => {
                            if c == 'c' && event.modifiers.contains(KeyModifiers::CONTROL) {
                                execute!(
                                    io::stdout(),
                                    Print("\r\nBye!\r\n"),
                                )?;

                                break;
                            }

                            self.buffer.push(c);

                            execute!(
                                io::stdout(),
                                Print(c),
                            )?;
                        },
                        KeyCode::Backspace => {
                            if self.buffer.len() < 1 { continue; }
                            self.buffer.pop();

                            execute!(
                                io::stdout(),
                                cursor::MoveLeft(1),
                                style::Print(" "),
                                cursor::MoveLeft(1),
                            )?;
                        },
                        KeyCode::Enter => {
                            if let Some(f) = cmds.handle(String::from_iter(&self.buffer)) {
                                f();
                            } else {
                                execute!(
                                    io::stdout(),
                                    Print("\r\nCommand not found")
                                )?
                            }
                            self.buffer.clear();
                            self.send_prompt()?;
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
