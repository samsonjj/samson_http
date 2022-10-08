use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::panic;
use std::sync::{Arc, Mutex};

use crate::http::{http_err, Request, Response};
use crate::threadpool::ThreadPool;

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

type RouteCallback = fn(Request) -> Response; // + Send + 'static>;
type Routes = Arc<Mutex<HashMap<String, RouteCallback>>>;

pub struct Server {
    num_threads: usize,
    routes: Routes,
}

impl Server {
    pub fn listen(&self, port: usize) -> std::io::Result<()> {
        let listener: TcpListener = TcpListener::bind(format!("0.0.0.0:{}", port))?;

        println!("listening on port {}", port);
        let pool = ThreadPool::build(4)?;

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            let routes = self.routes.clone();

            pool.execute(|| {
                Self::handle_connection(stream, routes);
            });
        }

        return Ok(());
    }

    pub fn default() -> Self {
        Self {
            num_threads: 10,
            routes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Path needs to follow a specific format. Not sure what that format is yet.
    /// If your route is not working, try messing with path.
    pub fn register(&mut self, path: &str, cb: RouteCallback) {
        self.routes.lock().unwrap().insert(path.to_string(), cb);
    }

    fn handle_connection(mut stream: TcpStream, routes: Routes) {
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

        let response = Self::process_request(request.unwrap(), routes);
        if let Err(e) = response {
            eprintln!("{}", e.to_string());
            return;
        }

        // use url::Url;
        stream
            .write_all(response.unwrap().as_string().as_bytes())
            .unwrap();
    }

    fn process_request(request: Request, routes: Routes) -> std::io::Result<Response> {
        dbg!();
        // if request.method == "GET" {
        //     if request.uri.is_empty() || request.uri == "/" {
        //         let body = fs::read_to_string("hello.html").unwrap();
        //         let mut response = Response::new();
        //         response.set_body(body);
        //         return Ok(response);
        //     }
        // }

        dbg!();

        // TODO: This method of forming the full uri is probably unreliable. Use a uri builder?

        // let uri = format!("http://{}{}", host, resource_path);
        // let uri =
        //     Url::parse(&uri).map_err(|_| http_err(&format!("failed to parse uri {}", uri)))?;

        dbg!(request.uri.path());
        let path = request.uri.path();
        if let Some(cb) = routes.lock().unwrap().get(path) {
            let result = panic::catch_unwind(|| cb(Request::new()));
            return match result {
                Ok(response) => Ok(response),
                Err(_) => Err(http_err(&format!("route {} panicked", path))),
            };
        }

        Response::not_found()
    }

    setter!(num_threads, usize);
}
