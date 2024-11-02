use core::str;
use std::{io::{Read, Write}, net::TcpStream};

fn main() {
    let mut client = TcpStream::connect("127.0.0.1:3000").unwrap();
    client.write("hello".as_bytes()).unwrap();

    let mut buf = [0;5];
    client.read(&mut buf).unwrap();
    println!("接收服务端响应:{:?}",str::from_utf8(&buf).unwrap());
    
}
