
use std::net::UdpSocket;
use std::thread;

fn main() {

    let mut socket = UdpSocket::bind("127.0.0.1:4243").unwrap();
    let buf = [1, 2, 3, 4, 5, 6, 7, 1, 2, 3, 4, 5, 6, 7, 2, 3];

    loop {
        println!("{:?}", socket.send_to(&buf, "127.0.0.1:4242").unwrap());
        thread::sleep_ms(100);
    }

}
