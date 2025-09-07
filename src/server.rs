use std::net::TcpListener;
use std::io;

use crossterm::{execute, style::Print};

use crate::agent::Agent;

pub struct Server {
    listener: TcpListener,
    agents: Vec<Agent>
}

impl Server {
    pub fn listen(addr: &str) -> Self {
        // put this in seperate thread
        let l = TcpListener::bind(addr).unwrap();
        execute!(
            io::stdout(),
            Print("\r\n"),
            Print("crabnest server listening on "),
            Print(addr),
        ).unwrap();

        Self{ listener: l, agents: Vec::new() }
    }

    pub fn accept(&mut self) -> io::Result<()>{
        loop {
            match self.listener.accept() {
                Ok((stream, _addr)) => Agent::new(stream).handle()?,
                Err(_) => {}
            }
        }
    }
}
