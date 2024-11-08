#[derive(Clone)]
pub struct Header {
    pub header_key: String,
    pub header_value: String,
}

#[derive(Clone)]
pub struct Response {
    pub status_code: i32,
    pub body: Option<String>,
    pub headers: Vec<Header>,
}
pub trait Respondable {
    fn response_str(&self) -> String;
}

fn get_code_str(code: i32) -> &'static str {
    match code {
        200 => "OK",
        404 => "NOT FOUND",
        201 => "ACCEPTED",
        500 => "INTERNAL SERVER ERROR",
        401 => "UNAUTHORIZED",
        _ => "INTERNAL SERVER ERROR",
    }
}

impl Respondable for Response {
    fn response_str(&self) -> String {
        let header_str = self
            .headers
            .clone()
            .into_iter()
            .map(|x| format!("{}:{}", x.header_key, x.header_value))
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            "HTTP/1.1 {} {}\n{}\n\n{}",
            self.status_code,
            get_code_str(self.status_code),
            header_str,
            self.body.clone().unwrap_or_default()
        )
    }
}
