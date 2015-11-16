
use std::net::{ UdpSocket };

fn main() {

    let socket = UdpSocket::bind("127.0.0.1:35666").unwrap();

    let mut buf = [0; 100];
    let (res, soc) = socket.recv_from(&mut buf).unwrap();
    let message = &buf[..res];

    println!("{:?}", message);
    println!("{:?}", soc);

    drop(socket);
    
}
