use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use scheduled_thread_pool::ScheduledThreadPool;

fn main() {
    println!("Hello, world!");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ScheduledThreadPool::with_name("thread :{} ", 4);

    let mut n = 0;
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established! {}", n);
        pool.execute(|| {
            hanlde_request(stream);
        });

        n += 1;
    }

    println!("shutting down start!!");
}

fn hanlde_request(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // println!("{}", String::from_utf8_lossy(&buffer));

    let get = "GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let crash = b"GET /crash HTTP/1.1\r\n";
    let (status_line, contentfile) = if buffer.starts_with(get.as_bytes()) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(crash) {
        panic!("abc");
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(contentfile).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
