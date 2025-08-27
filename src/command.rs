use std::{collections::HashMap, io};

use crossterm::{execute, style::Print};

pub struct Commands {
    cmd: HashMap<String, fn() -> io::Result<()>>
}

pub fn test() -> io::Result<()> {
    execute!(
        io::stdout(),
        Print("this is a test\r\n")
    )?;

    return Ok(());
}

pub fn help() -> io::Result<()> {
    execute!(
        io::stdout(),
        Print("welcome to crabnest\r\n")
    )?;
    return Ok(());
}

impl Commands {
    pub fn new() -> Self {
        return Self {
            cmd: HashMap::new(),
        }
    }

    pub fn insert(&mut self, s: &str, f: fn() -> io::Result<()>) -> io::Result<()> {
        self.cmd.insert(String::from(s), f);
        return Ok(());
    }

    pub fn handle(&self, i: String) -> Option<&fn() -> io::Result<()>> {
        // is this dangerous?
        return self.cmd.get(&i);
    }
}
