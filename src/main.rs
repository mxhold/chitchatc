extern crate bufstream;

use std::io::prelude::*;
use std::io;
use std::net::TcpStream;
use bufstream::BufStream;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let tcp_stream = TcpStream::connect("127.0.0.1:4574").unwrap();

    tcp_stream.set_read_timeout(Some(Duration::from_millis(30))).unwrap();

    let mut buf_stream = BufStream::new(tcp_stream);

    print!("Name: ");
    io::stdout().flush().unwrap();
    let input = read_from_stdin();

    write_to_buf(&mut buf_stream, input.as_bytes());

    let (i_tx, i_rx) = channel();

    thread::spawn(move|| {
        loop {
            let input = read_from_stdin();
            i_tx.send(input).unwrap();
        }
    });

    let (o_tx, o_rx) = channel();

    let buf_stream = Arc::new(Mutex::new(buf_stream));

    {
        let buf_stream = buf_stream.clone();
        thread::spawn(move|| {
            loop {
                let mut output = String::new();

                let mut buf_stream = buf_stream.lock().unwrap();

                while buf_stream.read_line(&mut output).unwrap_or(0) > 0 {
                    if output == ".\n" {
                        break
                    } else {
                        o_tx.send(output.clone()).unwrap();
                        output.clear()
                    }
                }
            }
        });
    }

    loop {
        match i_rx.try_recv() {
            Ok(input) => {
                let mut buf_stream = buf_stream.lock().unwrap();
                buf_stream.write(input.as_bytes()).unwrap();
                buf_stream.flush().unwrap();
            },
            _ => (),
        }

        match o_rx.try_recv() {
            Ok(output) => {
                print!("{}", output);
                io::stdout().flush().unwrap();
            },
            _ => (),
        }

        std::thread::sleep(Duration::from_millis(10));
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
