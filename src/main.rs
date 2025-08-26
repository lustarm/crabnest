mod console;
mod command;

use console::Console;

use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let l = Console::new();
    l.clear()?;

    println!("Welcome to CrabNest C2\r");
    println!("Type 'help' for commands\r");

    l.read()?;



    return Ok(());
}
