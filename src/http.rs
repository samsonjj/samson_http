use std::collections::HashMap;
use std::fs;
use std::io::{prelude::*, BufReader, Error, ErrorKind};
use std::net::TcpStream;

use crate::util::{self, crack};

use url::Url;

pub fn http_err(msg: &str) -> Error {
    Error::new(ErrorKind::Other, msg)
}

#[derive(Clone, Debug)]
pub struct Response {
    pub version: String,
    pub status: i32,
    pub headers: HashMap<String, String>,
    body: Option<String>,
}

impl Response {
    pub fn new() -> Self {
        Response {
            version: "HTTP/1.1".to_string(),
            status: 200,
            headers: HashMap::new(),
            body: None,
        }
    }

    pub fn set_body(&mut self, body: String) {
        self.body = Some(body);
        self.set_content_length();
    }

    pub fn as_string(&self) -> String {
        format!(
            "{} {} {}\r\n{}\r\n{}",
            self.version,
            self.status,
            util::get_reason(self.status),
            self.headers
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<String>>()
                .join("\r\n"),
            match self.body {
                Some(ref body) => format!("\r\n{}", body),
                None => "".to_string(),
            },
        )
    }

    pub fn get_content_length(&self) -> usize {
        match self.body {
            Some(ref body) => body.len(),
            None => 0,
        }
    }

    fn set_content_length(&mut self) {
        self.headers.insert(
            "Content-Length".to_string(),
            format!("{}", self.get_content_length()),
        );
    }

    pub fn not_found() -> std::io::Result<Self> {
        let mut response = Self::new();
        response.status = 404;
        response.set_body(fs::read_to_string("404.html")?);
        Ok(response)
    }
}

#[derive(Clone, Debug)]
pub struct Request {
    pub method: String,
    pub uri: Url,
    pub version: String,
    pub phrase: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl Request {
    pub fn new() -> Self {
        Self {
            method: String::new(),
            uri: Url::try_from("http://test.com").unwrap(),
            version: String::new(),
            phrase: String::new(),
            headers: HashMap::new(),
            body: None,
        }
    }
    pub fn try_from_stream(stream: &mut TcpStream) -> Result<Self, std::io::Error> {
        let buf_reader = BufReader::new(stream);
        let mut lines = buf_reader.lines().map(|result| result).into_iter();

        let mut request = Request::new();

        // PARSE STATUS

        let line = crack!(lines.next(), http_err("missing status line"))?;
        let mut parts = line.split(" ");
        request.method = crack!(parts.next(), http_err("missing request method")).to_owned();

        if !util::is_http_method(&request.method) {
            return Err(http_err("invalid http method"));
        }

        let uri = crack!(parts.next(), http_err("missing request uri"));
        request.version = crack!(parts.next(), http_err("missing request version")).to_string();

        // PARSE HEADERS

        while let Some(line) = lines.next() {
            let line = line?;
            if line.is_empty() {
                break;
            }
            let mut iter = line.split(":").map(|token| token.trim());
            request.headers.insert(
                crack!(iter.next(), http_err("missing header key")).to_owned(),
                crack!(iter.next(), http_err("missing header value")).to_owned(),
            );
        }

        // supplement uri
        let host = &request
            .headers
            .get("Host")
            .expect("missing Host header in request");
        let host = Url::parse(&format!("http://{host}")).expect("failed to parse host from header");
        request.uri = host
            .join(uri)
            .map_err(|_| http_err("failed to parse uri"))?;

        // PARSE BODY

        if !["POST", "PUT", "PATCH"].contains(&request.method.as_ref()) {
            return Ok(request);
        }

        let body = lines
            .take_while(|l| {
                // TODO: potential for timeout if no more lines, this doesn't handle timeout
                return l.is_err() || !l.as_ref().unwrap().is_empty();
            })
            .collect::<Result<String, Error>>()?;

        request.body = if body.is_empty() { Some(body) } else { None };

        Ok(request)
    }

    pub fn as_abbreviated_string(&self) -> String {
        format!(
            "{} {} {}\r\n{}\r\n{}",
            self.method,
            self.uri,
            self.version,
            self.headers
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<String>>()
                .join("\r\n"),
            match self.body {
                Some(ref body) => format!("\r\n{}", body),
                None => "".to_string(),
            }
        )
    }
}
