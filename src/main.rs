mod lib;
use std::{
    net::{TcpListener, TcpStream},
    io::{BufReader, BufRead, Write}
};

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(listener) => listener, 
        Err(e) => panic!("Could not start the server at port 7878.\n {:?}", e)
    };

    listener.incoming().for_each(|connection| {
        let stream = connection.unwrap();
        handle_connection(stream)
    });
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
    let status = "HTTP/1.1 200 OK\r\n\r\n";
    let body = "Hello"; 
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status, body.len(), body);

    stream.write_all(response.as_bytes()).unwrap();
}
