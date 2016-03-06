
extern crate regex;
pub use self::regex::*;

pub mod request;
pub mod response;

pub enum HttpMethod {
    GET,
    POST,
}

impl ToString for HttpMethod {
    fn to_string(&self) -> String {
        match *self {
            HttpMethod::GET => "GET".to_string(),
            HttpMethod::POST => "POST".to_string(),
        }
    }
}
