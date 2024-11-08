#![allow(warnings)]
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

mod abstracts;
mod handlers;
mod macro_test;
mod pool;
mod response;
use handlers::{home, not_found, register_user};
use pool::ThreadPool;
use response::{Respondable, Response};

// lines for the resposne
const OK_LINE: &str = "HTTP/1.1 200 OK";

#[derive(Debug, Clone)]
pub struct Request<'a> {
    request_line: Option<&'a str>,
    headers: Option<Vec<&'a str>>,
    body: Option<Vec<u8>>,
}

impl<'a> Request<'a> {
    fn new() -> Self {
        Self {
            request_line: None,
            headers: None,
            body: None,
        }
    }

    fn set_request_line(&self, request_line: &'a str) -> Self {
        Self {
            request_line: Some(request_line),
            headers: self.headers.clone(),
            body: self.body.clone(),
        }
    }

    fn set_headers(&self, headers: Vec<&'a str>) -> Self {
        Self {
            request_line: self.request_line,
            headers: Some(headers),
            body: self.body.clone(),
        }
    }

    fn set_body(&self, body: Vec<u8>) -> Self {
        Self {
            request_line: self.request_line,
            headers: self.headers.clone(),
            body: Some(body),
        }
    }

    fn get_path(&self) -> String {
        if let Some(request_line) = self.request_line {
            let parts = request_line.split(" ");
            for part in parts {
                if !(part.starts_with("POST") || part.starts_with("GET")) {
                    return String::from(part);
                }
            }
        }
        return String::from("");
    }
}

const NOT_FOUND_LINE: &str = "HTTP/1.1 404 NOT FOUND";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(move || {
            primitive_router(stream);
        });
    }
}

fn primitive_router(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(stream.try_clone().unwrap());
    let mut name = String::new();

    loop {
        let r = buf_reader.read_line(&mut name).unwrap();
        if r < 3 {
            break;
        }
    }
    let mut request = Request::new();

    let mut content_length = 0;

    let linesplit = name.split("\n");

    for line in linesplit {
        if line.starts_with("GET") || line.starts_with("POST") {
            request = request.set_request_line(line)
        }
        if line.starts_with("Content-Length") {
            let inner_split = line.split(":");
            for s in inner_split {
                if !s.starts_with("Content-Length") {
                    content_length = s.trim().parse::<usize>().unwrap_or_default();
                }
            }
        }
    }
    if content_length > 0 {
        let mut body_content = vec![0; content_length];
        if let Ok(bytes_read) = buf_reader.read_exact(&mut body_content) {
            request = request.set_body(body_content);
        }
    }
    // println!("Content length was  {}", content_length);
    let req = request.clone();
    let path = req.get_path();
    let response = match path.as_str() {
        "/" => home(request),
        "/user/register" => register_user(request),
        _ => not_found(request),
    };

    stream
        .write_all(response.response_str().as_bytes())
        .unwrap();
}
