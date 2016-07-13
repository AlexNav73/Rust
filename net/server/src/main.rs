
use std::net::UdpSocket;

fn main() {

    let mut socket = UdpSocket::bind("127.0.0.1:4242").unwrap();
    socket.set_nonblocking(true).unwrap();

    let mut buf = [0; 10];

    loop {
        let (_, client) = socket.recv_from(&mut buf).unwrap();

        println!("{} {:?}", client, &buf[..]);
    }
    
}
