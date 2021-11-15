use std::net::TcpListener;
mod connection;
mod thread_pool;
use thread_pool::ThreadPool;
use std::thread;

const PORT: &str = "7878";
const ADDRESS: &str = "127.0.0.1";

fn main() {
    let listener = TcpListener
    ::bind(format!("{}:{}", ADDRESS, PORT)).unwrap();
    println!("Starting server at {}:{} ...", ADDRESS, PORT);

    // creating thread pool
    let pool = ThreadPool::new(10);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.run(||{
            thread::sleep(std::time::Duration::from_secs(5));
            connection::handler(stream);
        });
    }
}
