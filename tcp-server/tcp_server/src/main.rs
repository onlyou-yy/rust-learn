use std::{io::{Read, Write}, net::TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("server running in 127.0.0.1:3000");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("接收到客户端链接");

        let mut buf = [0;1024];
        stream.read(&mut buf).unwrap();
        stream.write(&mut buf).unwrap();
    }
}
