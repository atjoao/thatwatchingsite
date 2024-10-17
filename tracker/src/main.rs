use std::io::{BufRead, Write};
use std::{io::BufReader, net::{TcpListener, TcpStream}};

mod structs;

mod sources {
    pub mod nyaa_si;
}

// parse where the request wants needs to go and return it
// [ /search, /t?q=name&source=, /add?hash=, ] 

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("[info] Connection received from {:?}", stream.peer_addr().unwrap());
        handle_connection(stream);
    }
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{:#?}", http_request);
    println!("{:#?}", http_request[0]); // thing to parse

    let status_line = "HTTP/1.1 200 OK";
    let contents = "test";
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}