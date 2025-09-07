use std::{io::{self, Write}, net::TcpStream};

use crossterm::{execute, style::Print};
use crate::cprintln;

pub struct Agent {
    stream: TcpStream
}

impl Agent {
    pub fn new(stream: TcpStream) -> Self {
        Agent { stream: stream }
    }

    pub fn handle(mut self) -> io::Result<()>{
        cprintln!("got connection from {:?}", self.stream)?;
        self.stream.write(b"test\n")?;
        Ok(())
    }

    pub fn stream(&self) -> &TcpStream {
        &self.stream
    }
}
