use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{str, thread};

use crate::util;

pub fn serve(address: &str) -> Result<(), failure::Error> {
    // listener socket which is used for waiting for connection requesting from client
    let listener = TcpListener::bind(address)?;

    loop {
        // connected socket which is used for communication with client
        let (stream, _) = listener.accept()?;

        // one thread per connection
        thread::spawn(move || {
            handler(stream).unwrap_or_else(util::err);
        });
    }
}

fn handler(mut stream: TcpStream) -> Result<(), failure::Error> {
    debug!("Handling data from {}", stream.peer_addr()?);
    let mut buffer = [0u8; 1024];
    loop {
        // read from stream, write to buffer
        let nbytes = stream.read(&mut buffer)?; // returns 0 if reach EOF
        if nbytes == 0 {
            debug!("connection closed.");
            return Ok(());
        }
        // bytestream to utf8 encoded string
        print!("{}", str::from_utf8(&buffer[..nbytes])?);
        // read from buffer, write to stream
        stream.write_all(&buffer[..nbytes]);
    }
}
