use std::net::TcpListener;
mod connection;

const PORT: &str = "7878";
const ADDRESS: &str = "127.0.0.1";

fn main() {
    let listener = TcpListener
    ::bind(format!("{}:{}", ADDRESS, PORT)).unwrap();
    println!("Starting server at {}:{} ...", ADDRESS, PORT);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        connection::handler(stream);
    }
}
