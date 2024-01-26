use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};



pub fn get_lower_case(c: char) -> char {
    let i = c as u8;
    if i < 97 {
        (c as u8 + 32) as char
    } else {
        c
    }
}


fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let mut connection_count = 0;
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, &mut connection_count);
    }
}

fn handle_connection(mut stream: TcpStream, connection_count: &mut i32) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Received {:?}", http_request);

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    *connection_count += 1;
    println!("Established Connection {connection_count}");

    stream.write_all(response.as_bytes()).unwrap();
}
