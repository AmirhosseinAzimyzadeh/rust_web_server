use std::{io::Read, net::TcpStream};
use std::io::prelude::*;
use std::fs;
#[path = "./responses.rs"] mod responses;

pub fn handler(mut stream: TcpStream) {
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();

  println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

  let response = responses::create_response(
    responses::Response::OK,
    get_content(),
  );

  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}

fn get_content() -> String {
  fs::read_to_string("assets/index.html").unwrap()
}