use std::net::{TcpListener, TcpStream};

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();

        println!("Connection established!");
    }
}
