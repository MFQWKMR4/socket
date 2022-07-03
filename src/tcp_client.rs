use std::io::{ self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str;

pub fn connect(address: &str) -> Result<(), failure::Error> {
    // request a connection and connect
    let mut stream = TcpStream::connect(address)?;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        // to stream (= request)
        stream.write_all(input.as_bytes())?;

        // from stream (= response)
        let mut reader = BufReader::new(&stream);
        let mut buffer = Vec::new();
        // write to buffer
        reader.read_until(b'\n', &mut buffer)?;
        // bytestream to utf8 encoded string
        print!("{}", str::from_utf8(&buffer)?);
    }
}