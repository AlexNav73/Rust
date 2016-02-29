
use std::collections::HashMap;

#[derive(Debug)]
pub struct Response {
    status: String,
    headers: HashMap<String, String>,
    content: String,
}

impl Response {
    pub fn new(s: String) -> Response {
        Response {
            status: s,
            headers: HashMap::new(),
            content: String::new(),
        }
    }
}
