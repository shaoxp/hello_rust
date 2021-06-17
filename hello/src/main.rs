use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::thread;
use std::time::Duration;

use hello::ThreadPool;

fn main() {
    println!("Hello, world!");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    let mut n = 0;
    for stream in listener.incoming().take(4) {
        let stream = stream.unwrap();

        println!("Connection established! {}",n);
        pool.execute(||{
            hanlde_request(stream);
        });
       
        n +=1;
    }

    println!("shutting down start!!");
}

fn hanlde_request(mut stream: TcpStream){

    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    // println!("{}", String::from_utf8_lossy(&buffer));

    let get = "GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line,contentfile) = if buffer.starts_with(get.as_bytes()) {
        ("HTTP/1.1 200 OK\r\n\r\n","res/hello.html")
    }else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n","res/hello.html")
    }else{
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n","404.html")
    
    };

    let contents = fs::read_to_string(contentfile).unwrap();
    let response = format!("{}{}",status_line,contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
