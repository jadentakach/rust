use std::net::{TcpListener, TcpStream};

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
    }
}