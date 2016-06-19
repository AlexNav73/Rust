
use std::collections::HashMap;
use std::io;

use rustc_serialize::Encodable;

use super::data::{ Handler, ToTypeId };
use super::configuration::ServerConfig;
use super::net::UdpSocketWrapper;

pub struct Server {
    handlers: HashMap<u8, Box<Handler>>,
    socket: UdpSocketWrapper
}

impl Server {

    pub fn new<'a>(conf: ServerConfig) -> io::Result<Server> {
        Ok(Server {
            handlers: HashMap::new(),
            socket: try!(UdpSocketWrapper::bind(conf))
        })
    }

    pub fn reg_handler<E: ToTypeId, T: Handler + 'static>(&mut self, handler: T) {
        self.handlers.insert(E::get_type_id(), Box::new(handler));
    }
    
    fn handle(&self, data: &[u8]) {
        if let Some(ref handler) = self.handlers.get(&(data[0] as u8)) {
            handler.handle(&data[1..]);
        }
    }

    pub fn send<T: ToTypeId + Encodable>(obj: &T) {
        /*let data = encode(obj).unwrap();
        socket.send(/* ... */);*/
        unimplemented!();
    }

    pub fn recv() {
    }

}
