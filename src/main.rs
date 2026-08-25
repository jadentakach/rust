use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream}
};

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // println!("Incoming Http request: {http_request:#?}"); // cool but clutters the cli

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    println!("Preparing to start TCP server\nEnter desired port:");

    let mut port: String = String::new();
    std::io::stdin().read_line(&mut port).expect("Failed to read port");

    let port = port.trim();

    let address: String = format!("127.0.0.1:{}", port);
    let listener: TcpListener = TcpListener::bind(&address).unwrap();

    println!("Server started on port {}", port);

    for stream in listener.incoming() {
        let unwrapped: TcpStream = stream.unwrap();

        println!("New connection from {}", unwrapped.peer_addr().unwrap().to_string());
        handle_connection(unwrapped);
    }
}