// This file manages network communication, allowing clients to connect and interact with the server.

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: &str) -> Self {
        Server {
            address: address.to_string(),
        }
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(&self.address).expect("Could not bind to address");
        println!("Server listening on {}", self.address);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        handle_client(stream);
                    });
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    match stream.read(&mut buffer) {
        Ok(size) => {
            println!("Received: {}", String::from_utf8_lossy(&buffer[..size]));
            // Echo the message back to the client
            stream.write_all(&buffer[..size]).expect("Failed to send response");
        }
        Err(e) => {
            eprintln!("Error reading from stream: {}", e);
        }
    }
}