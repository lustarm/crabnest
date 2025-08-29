use std::{collections::HashMap, io, thread, time};

use crossterm::{execute, style::Print};

use crate::server::Server;

pub struct Commands {
    cmd: HashMap<String, fn() -> io::Result<()>>
}

pub fn test() -> io::Result<()> {
    return execute!(
        io::stdout(),
        Print("\r\n"),
        Print("this is a test\r\n")
    );
}

pub fn help() -> io::Result<()> {
    return execute!(
        io::stdout(),
        Print("\r\n"),
        Print("welcome to crabnest\r\n")
    );
}

pub fn listen() -> io::Result<()> {
    // create server
    let server = Server::listen("0.0.0.0:8000");
    server.accept()?;

    Ok(())
}

impl Commands {
    pub fn new() -> Self {
        Self {
            cmd: HashMap::new(),
        }
    }

    pub fn load(&mut self) -> io::Result<()> {
        self.insert("test", test)?;
        self.insert("help", help)?;
        self.insert("listen", listen)?;

        Ok(())
    }

    fn insert(&mut self, s: &str, f: fn() -> io::Result<()>) -> io::Result<()> {
        if let Some(_x) = self.cmd.insert(String::from(s), f) {
            println!("loaded command {} successfully", s);
        } else {
            println!("failed to load command {}", s);
        }

        thread::sleep(time::Duration::from_millis(1000));
        Ok(())
    }

    pub fn get(&self, i: String) -> Option<&fn() -> io::Result<()>> {
        self.cmd.get(&i)
    }
}
