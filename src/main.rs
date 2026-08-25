use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream}
};

fn build_response(status_code: u16, page: &str) -> String {
    let status: String = format!("HTTP/1.1 {}", status_code);
    let contents: String = fs::read_to_string(page).unwrap();
    let length: usize = contents.len();

    let response: String = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status, length, contents);
    
    response
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let response = build_response(200, "index.html");
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let response = build_response(404, "404.html");
        stream.write_all(response.as_bytes()).unwrap();
    }
    
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