extern crate bufstream;

use std::io::prelude::*;
use std::io;
use std::net::TcpStream;
use bufstream::BufStream;
use std::thread;
use std::sync::mpsc::channel;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:4574").unwrap();

    let mut buf = BufStream::new(stream);

    print!("Name: ");
    io::stdout().flush().unwrap();
    let input = read_from_stdin();

    write_to_buf(&mut buf, input.as_bytes());

    let (tx, rx) = channel();

    thread::spawn(move|| {
        loop {
            let input = read_from_stdin();
            tx.send(input).unwrap();
        }
    });

    loop {
        match rx.try_recv() {
            Ok(input) => {
                buf.write(input.as_bytes()).unwrap();
                buf.flush().unwrap();
            },
            _ => (),
        }

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
