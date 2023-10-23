use std::{
    io::{self, BufRead, Write},
    net::{TcpListener, TcpStream},
};

const SUCCESS_RESPONSE: &str = "HTTP/1.1 200 OK\r\n";
const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 NOT FOUND\r\n";

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

fn send_response(stream: &mut TcpStream, response: Response, content: Option<String>) {
    let response_str = response.to_string();
    let content_str = content.unwrap_or_default();
    let full_response;

    if content_str.is_empty() {
        full_response = format!("{}\r\n", response_str);
    } else {
        full_response = format!(
            "{}Content-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n\r\n",
            response_str,
            content_str.len(),
            content_str
        );
    }

    stream.write_all(full_response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_client_connection(mut stream: TcpStream) {
    let mut reader = io::BufReader::new(&stream);
    let received: Vec<u8> = reader.fill_buf().unwrap().to_vec();

    let x = String::from_utf8(received).unwrap();

    let path = x.split(" ").collect::<Vec<&str>>()[1];

    match path {
        "/" => send_response(&mut stream, Response::Success, None),
        "/user-agent" => {
            let user_agent = reader
                .lines()
                .find(|line| line.as_ref().unwrap().starts_with("User-Agent: "))
                .unwrap()
                .unwrap()
                .replace("User-Agent: ", "");
            send_response(&mut stream, Response::Success, Some(user_agent))
        }
        _ if path.starts_with("/echo") => send_response(
            &mut stream,
            Response::Success,
            Some(path.split("/echo/").collect::<Vec<&str>>()[1].to_string()),
        ),
        _ => send_response(&mut stream, Response::NotFound, None),
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

