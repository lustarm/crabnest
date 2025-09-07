mod console;
mod command;
mod agent;
mod server;
mod user;

use console::Console;
use command::Commands;

use std::{ io, thread, time };
use crossterm::{ execute, terminal, cursor, style::Print };

fn main() -> io::Result<()> {

    let c = Console::new();
    c.clear()?;

    execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))?;
    execute!(io::stdout(), cursor::MoveTo(0, 0))?;
    execute!(io::stdout(), Print("welcome to crabnest c2...\r\n"))?;
    thread::sleep(time::Duration::from_secs(1));
    execute!(io::stdout(), Print("loading commands...\r\n"))?;
    thread::sleep(time::Duration::from_secs(1));

    let mut x = Commands::new();
    x.load()?;
    c.clear()?;

    println!("Welcome to CrabNest C2\r");
    println!("Type 'help' for commands\r");

    c.read(x)?;

    Ok(())
}
