
use http::regex::Regex;
use http::HttpMethod;
use http::response::*;

use std::collections::HashMap;
use std::borrow::Borrow;
use std::fmt::Display;
use std::io::{ Error, ErrorKind };

pub struct Request {
    method: HttpMethod,
    headers: HashMap<String, String>,
    params: HashMap<String, String>,
}

#[derive(Debug)]
pub enum SendError {
    InvalidUrl,
    ConnectionError,
    UnknownError
}

impl From<Error> for SendError {
    fn from(e: Error) -> Self {
        match e.kind() {
            ErrorKind::ConnectionRefused => SendError::ConnectionError,
            ErrorKind::ConnectionReset => SendError::ConnectionError,
            ErrorKind::ConnectionAborted => SendError::ConnectionError,
            ErrorKind::NotConnected => SendError::ConnectionError,
            _ => SendError::UnknownError
        }
    }
}

impl Request {
    pub fn new() -> Request {
        Request {
            method: HttpMethod::GET,
            headers: HashMap::new(),
            params: HashMap::new(),
        }
    }

    pub fn append_header<K, V>(&mut self, name: &K, value: &V) -> &mut Self
        where K: ToString,
              V: ToString
    {
        self.headers.insert(name.to_string(), value.to_string());
        self
    }

    pub fn append_params<K, V>(&mut self, name: &K, value: &V) -> &mut Self
        where K: ToString,
              V: ToString
    {
        self.params.insert(name.to_string(), value.to_string());
        self
    }

    pub fn send<T>(&mut self, url: T) -> Result<Response, SendError>
        where T: AsRef<str> + Display
    {
        let (host, url) = try!(parse_url(url.as_ref()));
        self.append_header("Host", host.as_str());

        let status = format!("{} {} {}", self.method.to_string(), url, "HTTP/1.1");

        let headers = self.headers
                          .iter()
                          .fold(String::new(), |h, (k, v)| h + &*format!("{}: {}\n", k, v));
        let params = self.params
                          .iter()
                          .fold(String::new(), |h, (k, v)| h + &*format!("{}: {}\n", k, v));

        Ok(try!(try!(Response::new(host))
                .get_response(format!("{}\n{}\n\n{}", status, headers, params))))

    }
}

const URL_REGEX: &'static str = r"^((http.?:)//)?([A-Za-z0-9-.]+)(:[0-9]+)?(.*)$";

fn parse_url<'a>(url: &'a str) -> Result<(String, String), SendError> {
    let reg = Regex::new(URL_REGEX).unwrap();

    let host = try!(reg.captures(url)
                  .and_then(|v| v.at(3))
                  .and_then(|v| Some(v.to_string()))
                  .ok_or(SendError::InvalidUrl));

    let path = reg.captures(url)
                  .and_then(|v| v.at(5))
                  .and_then(|v| {
                      if v.is_empty() {
                          Some("/".to_string()) 
                      } else {
                          Some(v.to_string())
                      }
                  })
                  .unwrap_or("/".to_string());

    Ok((host, path))
}
