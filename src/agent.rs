use std::{io::{self, Read, Write}, net::{Shutdown, TcpStream}};

use crossterm::{execute, style::Print};
use crate::cprintln;

pub type Agents = Vec<Agent>;

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

        match self.stream.read_exact(&mut [0u8; 1]) {
            Ok(_) => {},
            Err(_e) => self.stream.shutdown(Shutdown::Both)?,
        }

        Ok(())
    }

    pub fn stream(&self) -> &TcpStream {
        &self.stream
    }
}
