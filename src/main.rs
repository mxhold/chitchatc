extern crate bufstream;

use std::io::prelude::*;
use std::io;
use std::net::TcpStream;
use bufstream::BufStream;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:4574").unwrap();

    let mut buf = BufStream::new(stream);

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let request = input.as_bytes();

    buf.write(request).unwrap();
    buf.flush().unwrap();
    let mut response = String::new();
    buf.read_line(&mut response).unwrap();

    print!("{}", response);
}
