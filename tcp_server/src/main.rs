use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use std::thread;

pub fn println_vec(v: Vec<String>) {
    for v1 in v {
        println!("{v1}");
    }
}

fn main() {
    let n_workers = 4;

    let pool = threadpool::Builder::new()
        .num_threads(n_workers)
        .thread_stack_size(8_000_000)
        .build();

    let bind_addr = "127.0.0.1:8000";
    let listener = TcpListener::bind(bind_addr).unwrap();

    println!("Bound to http://{bind_addr}");
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(move || {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    
    let mut response = "HTTP/1.1 200 OK\r\n\r\n".to_string();
    response.push_str(http_request[0].as_str());
    response.push_str("\n");
    response.push_str(std::fmt::format(format_args!("{:?}", thread::current().id())).as_str());

    stream.write_all(response.as_bytes()).unwrap();
}
