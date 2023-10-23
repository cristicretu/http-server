use std::{
    io::{self, BufRead, Write},
    net::{TcpListener, TcpStream},
};

const SUCCESS_RESPONSE: &str = "HTTP/1.1 200 OK\r\n\r\n";
const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

enum Response {
    Success,
    NotFound,
}

impl Response {
    fn to_string(&self) -> String {
        match self {
            Response::Success => SUCCESS_RESPONSE.to_string(),
            Response::NotFound => NOT_FOUND_RESPONSE.to_string(),
        }
    }
}

fn send_response(mut stream: TcpStream, response: Response) {
    stream.write(response.to_string().as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_client_connection(stream: TcpStream) {
    let mut reader = io::BufReader::new(&stream);
    let received: Vec<u8> = reader.fill_buf().unwrap().to_vec();

    let x = String::from_utf8(received).unwrap();

    let path = x.split(" ").collect::<Vec<&str>>()[1];

    match path {
        "/" => send_response(stream, Response::Success),
        _ => send_response(stream, Response::NotFound),
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client_connection(stream),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

