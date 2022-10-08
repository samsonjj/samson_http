// #![feature(concat_idents)]

use samson_http::{Request, Response, Server};

fn handle_hello(req: Request) -> Response {
    println!("hello world!");
    let mut response = Response::new();
    response.status = 200;
    response.set_body(String::from("pee pee poo poo"));
    panic!("poot");
    return response;
}

fn main() -> std::io::Result<()> {
    let mut server = Server::default();
    server.set_num_threads(4);

    server.register("/hello", handle_hello);

    let port = 8080;
    server.listen(port)?;

    Ok(())
}
