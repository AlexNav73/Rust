
use std::net::*;

struct Server {
    socket: UdpSocket,
    buf: [u8; 100]
}

impl Server {
    fn new(addr: &str) -> Server {
        Server {
            socket: UdpSocket::bind(addr).unwrap(),
            buf: [0; 100]
        }
    }

    fn start(&mut self) {
        loop {
            let (recved, _) = self.socket.recv_from(&mut self.buf).unwrap();
            println!("Log: recive {} bytes", recved);

            let mut vec = Vec::with_capacity(self.buf.len());
            for byte in self.buf[..recved].iter() { vec.push(*byte); }
            let message = String::from_utf8(vec).unwrap();
            if message.len() == 0 { break; }
            println!("{}", message);
        }
    }
}

fn main() {
    let mut server = Server::new("127.0.0.1:6666");
    server.start();
    drop(server.socket);
}
