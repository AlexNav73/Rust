
use std::fs::File;
use std::io::Error as ioError;
use std::io::Read;
use std::path::Path;
use std::result;

use super::rs::json;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct ServerConfig {
    port: u16,
    address: String,
}

pub type Result<T> = result::Result<T, ServerConfigurationError>;

impl ServerConfig {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<ServerConfig> {
        let mut buf = String::new();
        let mut file = try!(File::open(path));
        try!(file.read_to_string(&mut buf));
        json::decode(&buf).map_err(|_| ServerConfigurationError::InvalidConfigurationFormat)
    }
    
    pub fn addr(&self) -> &str { &self.address }
    
    pub fn port(&self) -> u16 { self.port }
}

#[derive(Debug)]
pub enum ServerConfigurationError {
    ProcessFileError,
    InvalidConfigurationFormat
}

impl From<ioError> for ServerConfigurationError {
    
    #[inline]
    fn from(_e: ioError) -> ServerConfigurationError {
        ServerConfigurationError::ProcessFileError
    }
}
