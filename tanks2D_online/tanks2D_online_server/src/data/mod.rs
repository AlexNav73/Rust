
pub use rustc_serialize as rs;

mod simple_test_object;

pub use self::simple_test_object::SimpleTestObject;

const PACKET_MAX_SIZE: usize = 15;

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub enum Protocol<T: rs::Decodable + rs::Encodable> {
    FileBatch { id: u32, count: u32, data: Vec<u8> },
    Object(T),
}

impl<T: rs::Decodable + rs::Encodable> Protocol<T> {
    pub fn split(data: &[u8]) -> Vec<Protocol<T>> {
        let max_size = PACKET_MAX_SIZE - 8; // id(4 byte) + count(4 byte)
        let mut vec = Vec::new();
        let count = data.len() as u32;
        
        for (idx, chunk) in data.chunks(max_size).enumerate() {
            vec.push(Protocol::FileBatch { id: idx as u32, count: count, data: chunk.to_vec() });
        }
        vec
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

impl From<Vec<Protocol<i32>>> for File {
    fn from(mut ps: Vec<Protocol<i32>>) -> File {
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

impl<'a, T: rs::Decodable + rs::Encodable + Copy> From<&'a T> for Protocol<T> {
    fn from(o: &T) -> Protocol<T> {
        Protocol::Object(o.clone())
    }
}
