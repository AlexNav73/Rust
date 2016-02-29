
use http::HttpMethod;
use http::response::*;

use std::collections::HashMap;
use std::borrow::Borrow;
use std::fmt::Display;

pub struct Request {
    method: HttpMethod,
    headers: HashMap<String, String>,
    params: HashMap<String, String>,
}

impl Request {
    pub fn new() -> Request {
        Request {
            method: HttpMethod::GET,
            headers: HashMap::new(),
            params: HashMap::new(),
        }
    }

    pub fn append_header<K, V>(&mut self, name: K, value: V)
        where K: ToString,
              V: ToString
    {
        self.headers.insert(name.to_string(), value.to_string());
    }

    pub fn append_params<K, V>(&mut self, name: K, value: V)
        where K: ToString,
              V: ToString
    {
        self.params.insert(name.to_string(), value.to_string());
    }

    pub fn send<T>(&self, url: T) -> Response
        where T: AsRef<str> + Display
    {
        let status = format!("{} {} {}", self.method.to_string(), url, "HTTP/1.1");
        let headers = self.headers
                          .iter()
                          .fold(String::new(), |h, (k, v)| h + &*format!("{}: {}\n", k, v));
        let params = self.params
                          .iter()
                          .fold(String::new(), |h, (k, v)| h + &*format!("{}: {}\n", k, v));
        Response::new(format!("{}\n{}\n\n{}", status, headers, params))
    }
}
