mod console;

use console::Console;

use std::io;

use tokio::net;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Welcome to CrabNest C2");
    println!("Type 'help' for commands");

    let l = Console::new();
    l.read()?;


    return Ok(());
}
