
use std::net::*;
use std::io::BufRead;

struct Client {
    socket: UdpSocket,
}

impl Client {
    fn new(addr: &str) -> Client {
        Client {
            socket: UdpSocket::bind(addr).unwrap()
        }
    }

    fn get_message(&mut self) -> String {
        let stdin = std::io::stdin();
        let mut input = stdin.lock();
        let mut user_in = String::new();
        input.read_line(&mut user_in).unwrap();
        user_in.pop();
        user_in
    }

    fn start(&mut self, dest: &str) {
        loop {
            let message = self.get_message();
            let mess_len = message.len();
            match self.socket.send_to(&message.into_bytes(), dest) {
                Ok(n) => println!("Log: {} bytes has been sended", n),
                Err(e) => panic!("Log: send error: {}", e)
            };
            if mess_len == 0 { break; }
        }
    }
}

fn main() {
    let mut client = Client::new("127.0.0.1:6655");
    client.start("127.0.0.1:6666");
    drop(client.socket);
}
