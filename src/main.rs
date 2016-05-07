extern crate bufstream;

use std::io::prelude::*;
use std::io;
use std::net::TcpStream;
use bufstream::BufStream;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:4574").unwrap();

    let mut buf = BufStream::new(stream);

    print!("Name: ");
    io::stdout().flush().unwrap();
    let input = read_from_stdin();

    write_to_buf(&mut buf, input.as_bytes());

    loop {
        let input = read_from_stdin();

        buf.write(input.as_bytes()).unwrap();
        buf.flush().unwrap();

        let mut buffer = String::new();

        while buf.read_line(&mut buffer).unwrap() > 0 {
            if buffer == ".\n" {
                break
            } else {
                print!("{}", buffer);
                buffer.clear()
            }
        }
    }
}

fn read_from_stdin() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input
}

fn write_to_buf(buf: &mut BufStream<TcpStream>, bytes: &[u8]) {
    buf.write(bytes).unwrap();
    buf.flush().unwrap();
}
