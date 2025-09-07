use std::{collections::HashMap, io, thread, time};

use crossterm::{execute, style::Print, terminal, cursor};

use crate::server::Server;

#[macro_export]
macro_rules! cprintln {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        execute!(io::stdout(), Print("\r\n"), Print(msg))
    }}
}

pub struct Commands {
    cmd: HashMap<String, fn() -> io::Result<()>>
}

fn clear() -> io::Result<()> {
    execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))?;
    execute!(io::stdout(), cursor::MoveTo(0, 0))
}

fn help() -> io::Result<()> {
    cprintln!("welcome to crabnest")
}

fn listen() -> io::Result<()> {
    thread::spawn(|| {
        let mut server = Server::listen("0.0.0.0:8000");
        server.accept().unwrap();
    });

    Ok(())
}

impl Commands {
    pub fn new() -> Self {
        Self {
            cmd: HashMap::new(),
        }
    }

    pub fn load(&mut self) -> io::Result<()> {
        self.insert("help", help)?;
        self.insert("listen", listen)?;

        self.insert("agents", || ->io::Result<()>{Ok(())})?;

        Ok(())
    }

    fn insert(&mut self, s: &str, f: fn() -> io::Result<()>) -> io::Result<()> {
        if let Some(_x) = self.cmd.insert(String::from(s), f) {
            execute!(
                io::stdout(),
                Print("failed to load command "),
                Print(s),
                Print("\r\n")
            )?;
        } else {
            execute!(
                io::stdout(),
                Print("loaded command "),
                Print(s),
                Print(" successfully\r\n")
            )?;
        }

        thread::sleep(time::Duration::from_secs(1));
        Ok(())
    }

    pub fn get(&self, i: &Vec<char>) -> Option<&fn() -> io::Result<()>> {
        self.cmd.get(&i.iter().collect::<String>())
    }
}
