extern crate bufstream;

use std::io::prelude::*;
use std::io;
use std::net::TcpStream;
use bufstream::BufStream;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:4574").unwrap();

    let mut buf = BufStream::new(stream);

    let input = read_from_stdin();

    write_to_buf(&mut buf, input.as_bytes());

    let response = read_line_from_buf(&mut buf);

    print!("{}", response);
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

fn read_line_from_buf(buf: &mut BufStream<TcpStream>) -> String {
    let mut response = String::new();
    buf.read_line(&mut response).unwrap();
    
    response
}
