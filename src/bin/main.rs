// #![feature(concat_idents)]

use samson_http::server::Server;

fn main() -> std::io::Result<()> {
    let mut server = Server::default();
    server.set_num_threads(4);

    let port = 8080;
    server.listen(port)?;

    Ok(())
}
