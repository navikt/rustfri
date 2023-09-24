extern crate core;

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    println!("====================\nRUSTFRI\n====================\n");
    match TcpListener::bind("0.0.0.0:8080") {
        Ok(listener) => motta_tilkoblinger(&listener),
        Err(error) => panic!("Error: {:?}", error)
    };
}

fn motta_tilkoblinger(listener: &TcpListener) {
    println!("Klar for å motta requests");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => les_og_svar_httpforespørsel(stream),
            Err(error) => panic!("Error: {:?}", error)
        }
    }
}

fn les_og_svar_httpforespørsel(stream: TcpStream) {
    println!("Connection established! 2");
    let request = les_httpforespørsel(&stream);
    println!("Request: {:#?}", request);
    svar_httpforespørsel(&stream)
}

fn les_httpforespørsel(stream: &TcpStream) -> Vec<String> {
    let reader = BufReader::new(stream);
    return reader
        .lines()
        .filter_map(|line| line.ok())
        .take_while(|line| !line.is_empty())
        .collect();
}

fn svar_httpforespørsel(mut stream: &TcpStream) {
    let response = "HTTP/1.1 200 OK\r\n\r\nRustfri!";
    stream.write_all(response.as_bytes()).unwrap();
}