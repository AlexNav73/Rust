
use std::fs::File;
use std::io::Error as ioError;
use std::io::Read;
use std::path::Path;
use std::result;
use std::borrow::Cow;
use std::marker::PhantomData;

use super::rs::json;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct ServerConfig<'a> {
    port: u16,
    address: Cow<'a, str>,
}

impl<'a> ServerConfig<'a> {
    pub fn addr(&self) -> &str { self.address.as_ref() }
    pub fn port(&self) -> u16 { self.port }
    
    fn set_add<T: Into<Cow<'a, str>>>(&mut self, addr: T) { self.address = addr.into(); }
    fn set_port(&mut self, port: u16) { self.port = port; }
}

pub type Result<T> = result::Result<T, ServerConfigurationError>;

pub struct ServerConfBuilder<'a, 'b> {
    conf: ServerConfig<'b>,
    _marker: PhantomData<&'a u8>
}

impl<'a, 'b> ServerConfBuilder<'a, 'b> {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<ServerConfBuilder<'a, 'b>> {
        let mut buf = String::new();
        let mut file = try!(File::open(path));
        try!(file.read_to_string(&mut buf));
        Ok(ServerConfBuilder {
            conf: match json::decode(&buf) {
                Ok(c) => c,
                Err(_) => return Err(ServerConfigurationError::InvalidConfigurationFormat)
            },
            _marker: PhantomData
        })
    }
    
    pub fn addr<S: Into<Cow<'b, str>>>(&mut self, addr: S) -> &mut Self {
        self.conf.set_add(addr);
        self
    }
    
    pub fn port(&mut self, port: u16) -> &mut Self {
        self.conf.set_port(port);
        self
    }
    
    pub fn create(self) -> ServerConfig<'b> {
        self.conf
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
