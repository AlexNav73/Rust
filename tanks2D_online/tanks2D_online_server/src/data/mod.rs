
pub use rustc_serialize as rs;
pub use bincode::{ encode, decode };

mod simple_test_object;

pub use self::simple_test_object::SimpleTestObject;

const PACKET_MAX_SIZE: usize = 15;

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub enum Protocol {
    FileBatch { id: u32, count: u32, data: Vec<u8> },
    Object { type_id: u8, data: Vec<u8> },
}

pub trait Handler {
    fn handle(&mut self, data: &[u8]);
}

pub trait ToTypeId {
    fn get_type_id() -> u8;
}

struct SimpleTestObjectHandler;

impl Handler for SimpleTestObjectHandler {
    fn handle(&mut self, data: &[u8]) {
        let obj: SimpleTestObject = decode(data).unwrap();
        println!("SimpleTestObject");
    }
}

pub struct DataManager {
    handlers: Vec<Box<Handler>>
}

impl DataManager {
    fn reg_handler<E: ToTypeId, T: Handler>(&mut self, handler: T) {
        self.handlers.insert(E::get_type_id(), Box::new(handler));
    }
    
    fn handle(&self, data: &[u8]) {
        self.handlers[data[0] as usize].handle(data[1..]);
    }

    fn send<T: ToTypeId + rs::Encodable>(obj: &T) {
        let data = encode(obj).unwrap();
        socket.send(/* ... */);
    }
}

pub fn split_to_packets(data: &[u8]) -> Vec<Protocol> {
    let max_size = PACKET_MAX_SIZE - 8; // id(4 byte) + count(4 byte)
    let mut vec = Vec::new();
    let count = data.len() as u32;

    for (idx, chunk) in data.chunks(max_size).enumerate() {
        vec.push(Protocol::FileBatch { id: idx as u32, count: count, data: chunk.to_vec() });
    }
    vec
}

struct SplitToPackets {
    single: Option<Protocol>,
    multiple: Option<Map>
}

impl SplitToPackets {
    fn new(data: &[u8]) -> SplitToPackets {
        SplitToPackets {
            single: if data.len() < PACKET_MAX_SIZE {
                Some(Protocol::Object { type_id: ... , data: data })
            } else { None },
            multiple: if data.len() > PACKET_MAX_SIZE {
                let max_size = PACKET_MAX_SIZE - 8; // id(4 byte) + count(4 byte)
                let mut vec = Vec::new();
                let count = data.len() as u32;

                Some(data.chunks(max_size)
                    .enumerate()
                    .map(|(idx, chunk)| Protocol::FileBatch { id: idx as u32, count: count, data: chunk.to_vec() }))
            } else { None }
        }
    }
}

impl Iterator for SplitToPackets {
    type Item = Protocol;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(p) = self.single {
            std::mem::replace(&mut self.single, None)
        } else {
            self.multiple.next()
        }
    }
}

pub struct File(Vec<u8>);

use std::path::Path;

impl File {
    pub fn save<P: AsRef<Path>>(&self, path: P) {
        use std::fs::File;
        use std::io::Write;
        
        let mut file = File::create(path).unwrap();
        file.write_all(&self.0);
    }
}

impl From<Vec<Protocol>> for File {
    fn from(mut ps: Vec<Protocol>) -> File {
        let mut res = Vec::new();
        
        for prot in &mut ps {
            res.append(match *prot {
                Protocol::FileBatch { ref mut data, .. } => data,
                _ => unreachable!()
            });
        }
        File(res)
    }
}

