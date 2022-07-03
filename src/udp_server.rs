use std::net::UdpSocket;
use std::str;

pub fn serve(address: &str) -> Result<(), failure::Error> {
    // this socket handle all data 
    let socket = UdpSocket::bind(address)?;

    loop {
        let mut buf = [0u8; 1024];
        // src to distinguish where they come from
        let (size, src) = socket.recv_from(&mut buf)?;

        debug!("Handling data from {}", src);
        print!("{}", str::from_utf8(&buf[..size])?);
        // send to addr where they come from
        socket.send_to(&buf, src)?;
    }
}
