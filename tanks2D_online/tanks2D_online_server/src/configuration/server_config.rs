
use std::fs::File;
use std::io::Error as ioError;
use std::io::Read;
use std::path::Path;
use std::result;

use super::rs::json;
use super::tcore::ConfCreator;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct ServerConfig {
    port: u16,
    address: String,
}

impl ServerConfig {
    pub fn addr(&self) -> &str { &self.address }
    pub fn port(&self) -> u16 { self.port }
}

pub type Result<T> = result::Result<T, ServerConfigurationError>;

pub struct ServerConfCreator(String);

impl ServerConfCreator {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<ServerConfCreator> {
        let mut buf = String::new();
        let mut file = try!(File::open(path));
        try!(file.read_to_string(&mut buf));
        Ok(ServerConfCreator(buf))
    }
}

impl ConfCreator<ServerConfig> for ServerConfCreator {
    type Err = ServerConfigurationError;
    
    fn create(self) -> Result<ServerConfig> {
        json::decode(&self.0).map_err(|_| ServerConfigurationError::InvalidConfigurationFormat)
    }
}

#[derive(Debug)]
pub enum ServerConfigurationError {
    /// Error occured while reading file
    ProcessFileError,
    /// File contains invalid configuration format
    InvalidConfigurationFormat
}

impl From<ioError> for ServerConfigurationError {
    
    //
    // For now, assume that all io errors is `ProcessFileError`
    // 
    #[inline]
    fn from(_e: ioError) -> ServerConfigurationError {
        ServerConfigurationError::ProcessFileError
    }
}
