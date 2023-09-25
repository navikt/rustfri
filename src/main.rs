extern crate core;

use json::{JsonValue, object};
use std::io::{BufRead, BufReader, Error, Write};
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
        les_og_svar_httpforespørsel(stream);
    }
}

fn les_og_svar_httpforespørsel(stream: Result<TcpStream, Error>) {
    match stream {
        Ok(stream) => {
            log_info("Connection established!");
            let _request = les_httpforespørsel(&stream);
            svar_httpforespørsel(&stream)
        },
        Err(error) => panic!("Error: {:?}", error)
    }
}

fn les_httpforespørsel(stream: &TcpStream) -> Vec<String> {
    let reader = BufReader::new(stream);
    let request = reader
        .lines()
        .filter_map(|line| line.ok())
        .take_while(|line| !line.is_empty())
        .collect();
    log_info(&format!("Request: {:#?}", request));
    return request
}

fn svar_httpforespørsel(mut stream: &TcpStream) {
    let respons = lag_http_respons(200, "Velkommen til Rustfri!");
    stream.write_all(respons.as_bytes()).expect("Kunne ikke skrive respons til klient")
}

fn lag_http_respons(statuskode: u8, innhold: &str) -> String {
    let statuslinje = lag_http_statuslinje(statuskode);
    return format!("HTTP/1.1 {statuslinje}\r\n\r\n{innhold}");
}

fn lag_http_statuslinje(statuskode: u8) -> String {
    return match statuskode {
        200 => String::from("200 OK"),
        _ => panic!("Ukjent statuskode angitt: {statuskode}\n")
    }
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