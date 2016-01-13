
use std::net::*;

const BUFF_SIZE: usize = 100;

struct Server {
    socket: UdpSocket,
    buf: [u8; BUFF_SIZE],
    user_connected: bool
}

impl Server {
    fn new(addr: &str) -> Server {
        Server {
            socket: UdpSocket::bind(addr).unwrap(),
            buf: [0; BUFF_SIZE],
            user_connected: false
        }
    }

    #[allow(unused)]
    fn start(&mut self) {
        loop {
            let (recved, from) = self.socket.recv_from(&mut self.buf).unwrap();
            println!("Log: recive {} bytes from {}", recved, from);

            if recved == 1 && self.buf[0] == 100u8 && !self.user_connected {
                println!("Log: user tries to connect");
                self.socket.send_to(&[200u8; 1], from);
                self.user_connected = true;
            }

            let mut vec = Vec::with_capacity(self.buf.len());
            for byte in self.buf[..recved].iter() {
                vec.push(*byte);
            }

            let message = String::from_utf8(vec).unwrap();
            if message.len() == 0 {
                break;
            }

            println!("{}", message);
        }
    }
}

fn main() {
    let mut ip = std::env::args();
    let to = &ip.nth(1).unwrap_or("127.0.0.1:6655".to_string());

    let mut server = Server::new(to);
    server.start();

    drop(server.socket);
}
