use crate::{
    abstracts::SignUpRequest, response::{Header, Respondable, Response}, Request
};
use std::str::FromStr;

pub fn general_error(request: Request) -> Response {
    let mut headers: Vec<Header> = Vec::new();
    let body_content = r#"{'msg': 'Internal Server Error'}"#.to_string();
    headers.push(Header {
        header_key: "Content-Type".to_string(),
        header_value: "application/json".to_string(),
    });
    headers.push(Header {
        header_key: "Content-Length".to_string(),
        header_value: format!("{}", body_content.len()),
    });
    return Response {
        status_code: 500,
        body: Some(body_content),
        headers,
    };
}

pub fn home(request: Request) -> Response {
    let mut headers: Vec<Header> = Vec::new();
    let body_content = r#"{'msg': 'Hello World!!'}"#.to_string();
    headers.push(Header {
        header_key: "Content-Type".to_string(),
        header_value: "application/json".to_string(),
    });
    headers.push(Header {
        header_key: "Content-Length".to_string(),
        header_value: format!("{}", body_content.len()),
    });
    return Response {
        status_code: 200,
        body: Some(body_content),
        headers,
    };
}

pub fn register_user(request: Request) -> Response {
    let mut headers: Vec<Header> = Vec::new();
    let data = request.body;
    let data_str = String::from_utf8(data.unwrap()).unwrap();
    let user = SignUpRequest::from_str(data_str.as_str()).unwrap();
    dbg!(user);
    let body_content = r#"{'msg': 'user registered'}"#.to_string();
    headers.push(Header {
        header_key: "Content-Type".to_string(),
        header_value: "application/json".to_string(),
    });
    headers.push(Header {
        header_key: "Content-Length".to_string(),
        header_value: format!("{}", body_content.len()),
    });
    return Response {
        status_code: 200,
        body: Some(body_content),
        headers,
    };
}

pub fn not_found(request: Request) -> Response {
    let mut headers: Vec<Header> = Vec::new();
    let body_content = r#"{'msg': 'Not Found'}"#.to_string();
    headers.push(Header {
        header_key: "Content-Type".to_string(),
        header_value: "application/json".to_string(),
    });
    headers.push(Header {
        header_key: "Content-Length".to_string(),
        header_value: format!("{}", body_content.len()),
    });
    return Response {
        status_code: 404,
        body: Some(body_content),
        headers,
    };
}
