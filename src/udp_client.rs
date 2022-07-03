use std::net::UdpSocket;
use std::{io,str};

pub fn communicate(address: &str) -> Result<(), failure::Error> {
    // os uses an available port if specify port=0
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        socket.send_to(input.as_bytes(), address)?;

        let mut buffer = [0u8; 1024];
        // able to save data whose size is less than 1025 bytes
        // because IP layer has payload size limitation
        socket.recv_from(&mut buffer).expect("failed to receive");

        print!("{}", str::from_utf8(&buffer).expect("failed to convert to string"));
    }
}