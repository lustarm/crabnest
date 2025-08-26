mod console;
mod command;

use console::Console;
use command::Commands;

use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut x = Commands::new();
    x.insert("test", command::test)?;

    let c = Console::new();
    c.clear()?;

    println!("Welcome to CrabNest C2\r");
    println!("Type 'help' for commands\r");

    c.read(x)?;

    return Ok(());
}
