extern crate core;

use json::{JsonValue, object};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use chrono::{DateTime, Utc};
use std::time::SystemTime;


fn main() {
    log_info("====================\nRUSTFRI\n====================\n");
    match TcpListener::bind("0.0.0.0:8080") {
        Ok(listener) => motta_tilkoblinger(&listener),
        Err(error) => panic!("Error: {:?}", error)
    };
}

fn motta_tilkoblinger(listener: &TcpListener) {
    log_info("Klar for å motta requests");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => les_og_svar_httpforespørsel(stream),
            Err(error) => panic!("Error: {:?}", error)
        }
    }
}

fn les_og_svar_httpforespørsel(stream: TcpStream) {
    log_info("Connection established!");
    let request = les_httpforespørsel(&stream);
    log_info(&format!("Request: {:#?}", request));
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

fn log_info(melding: &str) {
    println!("{}", json::stringify(lag_logstash_melding("INFO", melding)));
}

fn lag_logstash_melding(nivå: &str, melding: &str) -> JsonValue {
    return object!{
        "@timestamp": tidspunkt_som_rfc3339(),
        "@version": String::from("1.0"),
        message: String::from(melding),
        logger_name: String::from("rustfri"),
        level: String::from(nivå)
    };
}

fn tidspunkt_som_rfc3339() -> String {
    let tidspunkt_nå = SystemTime::now();
    let tidspunkt_nå: DateTime<Utc> = tidspunkt_nå.into();
    return tidspunkt_nå.to_rfc3339();
}