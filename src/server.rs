use std::net::{TcpListener, TcpStream};
use std::io;

use crossterm::{execute, style::Print};

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn listen(addr: &str) -> Self {
        // put this in seperate thread
        let l = TcpListener::bind(addr).unwrap();
        execute!(
            io::stdout(),
            Print("\r\n"),
            Print("crabnest server listening on {}"),
            Print(addr),
            Print("\r\n")
        ).unwrap();

        Self{ listener: l }
    }

    pub fn accept(self) -> io::Result<()> {
        for stream in self.listener.incoming() {
            self.handle(stream?)?;
        }

        Ok(())
    }

    fn handle(&self, stream: TcpStream) -> io::Result<()> {
        // handle stuff here
        Ok(())
    }
}
