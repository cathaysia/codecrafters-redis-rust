use std::net::TcpListener;

use std::io::{BufRead, BufReader, Read, Write};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buf = [0; 1024];
                let response = "+PONG\r\n".as_bytes();

                let reader = BufReader::new(stream.try_clone().unwrap());

                for line in reader.lines() {
                    let line = line.unwrap();
                    dbg!(&line);
                    if line.trim() == "ping" {
                        stream.write(response).unwrap();
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
