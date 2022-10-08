use uriparse::URI;

fn main() {
    let uri = URI::try_from("example.com/my/path?my=query").unwrap();
    println!("path : {}", uri.path());
    println!("query: {:?}", uri.query());
}
