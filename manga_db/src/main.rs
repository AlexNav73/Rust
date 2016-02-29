
mod http;

use http::request::*;

fn main() {

    let mut req = Request::new();

    let a = "A";
    let b = "b".to_string();
    req.append_header(a, b);

    req.append_params("param1", "value1");

    println!("{}", req.send("rust-lang.org").status);

}
