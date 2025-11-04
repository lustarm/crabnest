use std::{collections::HashMap, io, thread, time};

use crossterm::{execute, style::Print, terminal, cursor};

use crate::{global::GlobalState, server::Server};

#[macro_export]
macro_rules! cprintln {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        execute!(io::stdout(), Print("\r\n"), Print(msg))
    }}
}

pub struct Commands {
    cmd: HashMap<String, fn(GlobalState) -> io::Result<()>>
}

fn _clear() -> io::Result<()> {
    execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))?;
    execute!(io::stdout(), cursor::MoveTo(0, 0))
}

fn help(_g_state: GlobalState) -> io::Result<()> {
    cprintln!("welcome to crabnest")
}

fn listen(_g_state: GlobalState) -> io::Result<()> {
    thread::spawn(move || {
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

        self.insert("agents", |_g_state| -> io::Result<()>{
            Ok(())
        })?;

        Ok(())
    }

    fn insert(&mut self, s: &str, f: fn(GlobalState) -> io::Result<()>) -> io::Result<()> {
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

    pub fn get(&self, i: &Vec<char>) -> Option<&fn(GlobalState) -> io::Result<()>> {
        self.cmd.get(&i.iter().collect::<String>())
    }
}
