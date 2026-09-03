use std::io::{Read, Write};
use std::net::TcpListener;
use crate::http::{handle_request, parse_request};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!("Listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buffer = [0; 1024];

        let bytes_read = stream.read(&mut buffer)?;
        let raw_request = String::from_utf8_lossy(&buffer[..bytes_read]);
        let request = parse_request(&raw_request)?;

        println!("Method: {}", request.method);
        println!("Path: {}", request.path);

        let response = handle_request(&request);
        let http_response = response.to_http();

        stream.write_all(http_response.as_bytes())?;
    }

    Ok(())
}