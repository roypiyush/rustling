use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use std::fs;
use std::thread;

pub fn println_vec(v: Vec<String>) {
    for v1 in v {
        println!("{v1}");
    }
}

fn main() {
    let n_workers = 2;

    let pool = threadpool::Builder::new()
        .num_threads(n_workers)
        .thread_stack_size(8_000_000)
        .build();

    let bind_addr = "0.0.0.0:8000";

    match TcpListener::bind(bind_addr) {
        Err(e) => {
            println!("Couldn't bind {:?}", e);
        }
        Ok(listener) => {
            println!("Bound to http://{bind_addr}");
            for stream in listener.incoming() {
                let stream = stream.unwrap();

                pool.execute(move || {
                    handle_connection(stream);
                });
            }
        }
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    println!("Running {:?}", thread::current().id());

    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if http_request.len() == 0 || !http_request[0].ends_with("HTTP/1.1") {
        println!("Some other request");
        return;
    }

    let mut http_request_header = http_request[0].split(" ");
    let (_, http_path, _) = (
        http_request_header.next(),
        http_request_header.next().unwrap(),
        http_request_header.next(),
    );

    if http_path.ends_with("sleep") {
        println!("Performing heavy task {:?}", thread::current().id());
        thread::sleep(std::time::Duration::from_millis(10 * 1000));
    }

    let status_line = "HTTP/1.1 200 OK";
    let hello_html = if std::path::Path::new("tcp_server/hello.html").exists() {
        "tcp_server/hello.html"
    } else {
        "hello.html"
    };

    match fs::read_to_string(hello_html) {
        Err(e) => {
            println!(
                "Current Dir {:?}\n{:?}",
                std::path::PathBuf::from(".").canonicalize().unwrap(),
                e
            );
        }
        Ok(contents) => {
            let length = contents.len();
            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}
