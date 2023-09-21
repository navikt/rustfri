extern crate core;

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    match TcpListener::bind("127.0.0.1:7878") {
        Ok(listener) => { håndter_requests(listener) }
        Err(error) => panic!("Error: {:?}", error)
    };
}

fn håndter_requests(listener: TcpListener) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => { håndter_request(stream) }
            Err(error) => { panic!("Error: {:?}", error) }
        }
    }
}

fn håndter_request(mut stream: TcpStream) {
    println!("Connection established! 2");
    let reader = BufReader::new(&mut stream);
    let request: Vec<String> = reader.lines().filter_map(|line| line.ok()).take_while(|line| !line.is_empty()).collect();
    println!("Request: {:#?}", request);
    let response = "HTTP/1.1 200 OK\r\n\r\nRustfri!";
    stream.write_all(response.as_bytes()).unwrap();
}