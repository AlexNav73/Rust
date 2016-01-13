
use std::net::*;
use std::io::BufRead;
use std::time::Duration;
use std::sync::{Arc, Mutex};

const STATUS_ACK: u8 = 200;
const STATUS_SYN: u8 = 100;
const TIMEOUT: usize = 20;

struct Client {
    socket: UdpSocket,
}

impl Client {
    fn new(addr: &str) -> Client {
        Client { socket: UdpSocket::bind(addr).unwrap() }
    }

    fn get_message(&mut self) -> String {
        let stdin = std::io::stdin();
        let mut message = String::new();
        stdin.read_line(&mut message).unwrap();

        let mut ret_val = String::new();
        ret_val.push_str(message.trim());
        ret_val
    }

    fn start(&mut self, dest: &str) {
        loop {
            let message = self.get_message();
            let mess_len = message.len();
            match self.socket.send_to(message.as_bytes(), dest) {
                Ok(n) => println!("Log: {} bytes has been sended", n),
                Err(e) => panic!("Log: send error: {}", e),
            };
            if mess_len == 0 {
                break;
            }
        }
    }

    fn send_request_to_connect(mutex: Arc<Mutex<UdpSocket>>, dest: &str, duration: &Duration) {
        for _ in 0..TIMEOUT {
            let socket = mutex.lock().unwrap();
            socket.send_to(&[STATUS_SYN; 1], dest);
            std::thread::sleep(*duration);
        }
    }

    #[allow(unused)]
    fn try_to_connect(&mut self, dest: &str) -> bool {
        let duration = Duration::from_millis(100);
        let mut buff: [u8; 1] = [0u8];
        let socket = Arc::new(Muxtex::new(self.socket));

        let s = socket.clone();
        std::thread::spawn(|| { send_request_to_connect(s, dest, &duration) });

        let socket = mutex.lock().unwrap();
        match socket.recv_from(&mut buff) {
            Ok((readed, from)) if readed == 1 => {
                if buff[0] == STATUS_ACK {
                    return true;
                } else if buff[0] == STATUS_SYN {
                    socket.send_to(&[STATUS_ACK; 1], from);
                    return true;
                } else {
                    return false;
                }
            }
            Err(e) => {
                println!("Log: error occured {}", e);
                return false;
            }
            _ => unreachable!()
        }
    }
}

fn main() {
    let mut ips = std::env::args();
    let from = &ips.nth(1).unwrap_or("127.0.0.1:5555".to_string());
    let to = &ips.next().unwrap_or("127.0.0.1:6655".to_string());
    //let to = &ips.next().unwrap_or("192.164.1.2:6655".to_string());

    let mut client = Client::new(from);
    if client.try_to_connect(to) {
        client.start(to);
    }
    drop(client.socket);
}
