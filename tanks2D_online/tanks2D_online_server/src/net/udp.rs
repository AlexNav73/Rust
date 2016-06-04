
use std::vec::IntoIter;
use std::io;
use std::net::{UdpSocket, ToSocketAddrs, SocketAddr};

use super::ServerConfig;

impl<'a> ToSocketAddrs for ServerConfig<'a> {
    type Iter = IntoIter<SocketAddr>;
    
    #[inline]
    fn to_socket_addrs(&self) -> io::Result<IntoIter<SocketAddr>> {
        (self.addr(), self.port()).to_socket_addrs()
    }
}


pub struct UdpSocketWrapper {
    socket: UdpSocket
}

impl UdpSocketWrapper {
    
    #[inline]
    pub fn bind<T: ToSocketAddrs>(addr: T) -> io::Result<UdpSocketWrapper> {
        Ok(UdpSocketWrapper { 
            socket: try!(UdpSocket::bind(addr)) 
        })
    }
}
