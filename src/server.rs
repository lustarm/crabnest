use std::net::{TcpListener, TcpStream};
use std::io::{self, Write};

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
            Print("crabnest server listening on "),
            Print(addr),
        ).unwrap();

        Self{ listener: l }
    }

    pub fn accept(self) -> io::Result<()> {
        for stream in self.listener.incoming() {
            self.handle(stream?)?;
        }

        Ok(())
    }

    fn handle(&self, mut stream: TcpStream) -> io::Result<()> {
        // handle stuff here
        stream.write(b"test\n")?;
        Ok(())
    }
}
