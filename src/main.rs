mod console;
mod command;
mod agent;
mod server;
mod user;
mod global;

use console::Console;
use command::Commands;

use std::{ io, thread, time, sync::Mutex };
use crossterm::{ execute, terminal, cursor, style::Print };

use crate::global::GlobalStateStruct;

fn main() -> io::Result<()> {

    let c = Console::new();
    c.clear()?;

    // ew lol
    execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))?;
    execute!(io::stdout(), cursor::MoveTo(0, 0))?;
    execute!(io::stdout(), Print("welcome to crabnest c2...\r\n"))?;
    thread::sleep(time::Duration::from_secs(1));
    execute!(io::stdout(), Print("loading commands...\r\n"))?;
    thread::sleep(time::Duration::from_secs(1));

    // ew lol again..
    let g_state = global::GlobalState::new(Mutex::new(GlobalStateStruct::new()));
    let mut cmds = Commands::new();
    cmds.load()?;
    c.clear()?;

    println!("Welcome to CrabNest C2\r");
    println!("Type 'help' for commands\r");

    c.read(cmds, g_state)?;

    Ok(())
}
