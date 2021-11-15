use std::{io::Read, net::TcpStream};

pub fn handler(mut stream: TcpStream) {
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();
  println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}