use std::io::{Read, Write};
use std::net::TcpListener;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    println!("Runing on port 8888!");
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection establishdd");
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}
