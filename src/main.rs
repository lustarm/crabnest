mod console;
mod command;
mod agent;
mod server;
mod user;

use console::Console;
use command::Commands;

use std::{io, thread};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut x = Commands::new();
    x.load()?;

    let c = Console::new();
    c.clear()?;

    println!("Welcome to CrabNest C2\r");
    println!("Type 'help' for commands\r");

    c.read(x)?;

    Ok(())
}
