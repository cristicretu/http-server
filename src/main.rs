use std::collections::HashMap;
use std::env::args;
use std::fs::File;
use std::thread;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

const SUCCESS_RESPONSE: &str = "HTTP/1.1 200 OK\r\n";
const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 NOT FOUND\r\n";
const INTERNAL_SERVER_ERROR_RESPONSE: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n";
const CREATED_RESPONSE: &str = "HTTP/1.1 201 CREATED\r\n";

enum Response {
    Success,
    NotFound,
    InternalServerError,
    Created,
}

impl Response {
    fn to_string(&self) -> String {
        match self {
            Response::Success => SUCCESS_RESPONSE.to_string(),
            Response::NotFound => NOT_FOUND_RESPONSE.to_string(),
            Response::InternalServerError => INTERNAL_SERVER_ERROR_RESPONSE.to_string(),
            Response::Created => CREATED_RESPONSE.to_string(),
        }
    }
}

fn parse_request(request: &str) -> (HashMap<&str, &str>, String) {
    let mut result = HashMap::new();
    let mut body = String::new();
    let mut lines = request.lines();

    let first_line = lines.next().unwrap();
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    result.insert("method", parts[0]);
    result.insert("path", parts[1]);

    let mut header_complete = false;
    for line in lines {
        if header_complete {
            body.push_str(line);
            body.push_str("\n");
            continue;
        }

        if line.is_empty() {
            header_complete = true;
            continue;
        }

        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() == 2 {
            let key = parts[0].trim();
            let value = parts[1].trim();
            result.insert(key, value);
        }
    }

    (result, body)
}

fn send_response<T: AsRef<[u8]>>(
    stream: &mut TcpStream,
    response: Response,
    content: Option<T>,
    content_type: Option<&str>,
) {
    let response_str = response.to_string();
    let full_response;

    if let Some(content_data) = content {
        let content_type_str = content_type.unwrap_or("text/plain");
        full_response = format!(
            "{}Content-Type: {}\r\nContent-Length: {}\r\n\r\n{}\r\n\r\n",
            response_str,
            content_type_str,
            content_data.as_ref().len(),
            String::from_utf8_lossy(content_data.as_ref())
        )
    } else {
        full_response = format!("{}\r\n", response_str)
    }

    stream.write_all(full_response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn save_file_path(stream: &mut TcpStream, path: &str, content: &str) {
    let dirpath = args().nth(2).unwrap_or(".".to_string());
    let file_name = path.split("/files/").collect::<Vec<&str>>()[1];

    let cleaned_content = content.replace("\0", "");

    let cleaned_content = cleaned_content.trim_end_matches('\n');

    match File::create(format!("{}/{}", dirpath, file_name)) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(cleaned_content.as_bytes()) {
                println!("Failed to write to file: {}", e);
                return send_response::<String>(stream, Response::InternalServerError, None, None);
            }
            return send_response::<String>(stream, Response::Created, None, None);
        }
        Err(e) => {
            println!("Failed to create file: {}", e);
            return send_response::<String>(stream, Response::InternalServerError, None, None);
        }
    }
}

fn get_file_path(stream: &mut TcpStream, path: &str) {
    let dirpath = args().nth(2).unwrap_or(".".to_string());
    let file_name = path.split("/files/").collect::<Vec<&str>>()[1];

    let file = std::fs::File::open(dirpath + "/" + file_name);

    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            let file_size = file.read_to_string(&mut contents);
            if file_size.is_err() {
                return send_response::<String>(stream, Response::InternalServerError, None, None);
            }

            return send_response(
                stream,
                Response::Success,
                Some(contents),
                Some("application/octet-stream"),
            );
        }
        Err(_) => return send_response::<String>(stream, Response::NotFound, None, None),
    }
}

fn user_agent_route(stream: &mut TcpStream, user_agent: Option<&str>) {
    if user_agent.is_none() {
        send_response::<String>(stream, Response::Success, None, None);
        return;
    }
    send_response(
        stream,
        Response::Success,
        user_agent.map(|s| s.to_string()),
        Some("text/plain"),
    );
}

fn index_route(stream: &mut TcpStream) {
    send_response::<String>(stream, Response::Success, None, None);
}

fn echo_route(stream: &mut TcpStream, content: Option<String>) {
    send_response(stream, Response::Success, content, Some("text/plain"));
}

fn not_found_route(stream: &mut TcpStream) {
    send_response::<String>(stream, Response::NotFound, None, None);
}

fn handle_client_connection(mut stream: std::net::TcpStream) {
    let mut buffer = [0u8; 1024];
    let n = stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[0..n]);

    let (parsed_request, body) = parse_request(&request);

    let path = parsed_request["path"];
    let method = parsed_request["method"];

    match path {
        "/" => index_route(&mut stream),
        "/user-agent" => user_agent_route(&mut stream, Some(parsed_request["User-Agent"])),
        _ if path.starts_with("/files") && method == "POST" => {
            save_file_path(&mut stream, path, &body)
        }
        p if p.starts_with("/files") => get_file_path(&mut stream, path),
        p if p.starts_with("/echo") => echo_route(
            &mut stream,
            Some(p.split("/echo/").collect::<Vec<&str>>()[1].to_string()),
        ),
        _ => not_found_route(&mut stream),
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                thread::spawn(move || {
                    handle_client_connection(_stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

/*
TODO:
- max 4 threads
- clean up functions and code - split

*/

