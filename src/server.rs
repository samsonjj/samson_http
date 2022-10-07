use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use crate::http::{Request, Response};
use crate::threadpool::ThreadPool;
// use crate::util::setter;

use concat_idents::concat_idents;

macro_rules! setter {
    ($name:ident, $type:ty) => {
        concat_idents!(fn_name = set_, $name {
            pub fn fn_name(&mut self, val: $type) {
                self.$name = val;
            }
        });
    }
}

pub struct Server {
    num_threads: usize,
}

impl Server {
    pub fn listen(&self, port: usize) -> std::io::Result<()> {
        let listener: TcpListener = TcpListener::bind(format!("0.0.0.0:{}", port))?;

        println!("listening on port {}", port);
        let pool = ThreadPool::build(4)?;

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            pool.execute(|| {
                handle_connection(stream);
            });
        }

        return Ok(());
    }

    pub fn new() -> Self {
        Self { num_threads: 1 }
    }

    pub fn default() -> Self {
        Self { num_threads: 10 }
    }

    setter!(num_threads, usize);
}

fn handle_connection(mut stream: TcpStream) {
    println!("Connection established!");
    if let Err(_) = stream.set_read_timeout(Some(std::time::Duration::from_secs(3))) {
        eprint!("failed to set request read timeout");
    }

    let request = Request::try_from_stream(&mut stream);
    match request {
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            eprintln!("request timed out");
            return;
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
            eprintln!("request time out");
            return;
        }
        Err(_) => {
            eprintln!("unhandled request error");
            return;
        }
        Ok(_) => {}
    }

    let response = process_request(request.unwrap());
    if let Err(_) = response {
        eprintln!("failed to process request");
        return;
    }

    stream
        .write_all(response.unwrap().as_string().as_bytes())
        .unwrap();
}

fn process_request(request: Request) -> std::io::Result<Response> {
    if request.method == "GET" {
        if request.uri.is_empty() || request.uri == "/" {
            let body = fs::read_to_string("hello.html").unwrap();
            let mut response = Response::new();
            response.set_body(body);
            return Ok(response);
        }
    }

    Response::not_found()
}
