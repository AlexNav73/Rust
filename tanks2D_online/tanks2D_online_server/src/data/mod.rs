
use std::mem::replace;

pub use rustc_serialize as rs;
pub use bincode::rustc_serialize::{ encode, decode };

use bincode::SizeLimit;

mod simple_test_object;
mod file;
pub use self::simple_test_object::{SimpleTestObject, SimpleTestObjectHandler};
pub use self::file::File;

const PACKET_MAX_SIZE: usize = 15;

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub enum Protocol {
    FileBatch { id: u32, count: u32, data: Vec<u8> },
    Object { type_id: u8, data: Vec<u8> },
}

pub trait Handler {
    fn handle(&self, data: &[u8]);
}

pub trait ToTypeId {
    fn get_type_id() -> u8;
}

struct SplitToPackets {
    single: Option<Protocol>,
    multiple: Option<Vec<Protocol>>
}

impl SplitToPackets {
    fn new<T: ToTypeId + rs::Encodable>(object: &T) -> SplitToPackets {
        let data = encode(object, SizeLimit::Infinite).unwrap();
        let len = data.len();

        if len <= PACKET_MAX_SIZE {
            return SplitToPackets {
                single: Some(Protocol::Object { type_id: T::get_type_id(), data: data }),
                multiple: None
            };
        }

        if len > PACKET_MAX_SIZE {
            return SplitToPackets {
                single: None,
                multiple: {
                    let max_size = PACKET_MAX_SIZE - 8; // id(4 byte) + count(4 byte)
                    let count = len as u32;

                    Some(data.chunks(max_size)
                        .enumerate()
                        .map(|(idx, chunk)| Protocol::FileBatch { id: idx as u32, count: count, data: chunk.to_vec() })
                        .collect())
                }
            };
        }

        unreachable!()
    }
}

impl Iterator for SplitToPackets {
    type Item = Protocol;

    fn next(&mut self) -> Option<Self::Item> {
        match self.single {
            Some(_) => replace(&mut self.single, None),
            None => {
                // this is safe, because we hold either single object or iterator over object butches
                let mut vec = self.multiple.as_mut().unwrap();
                let len = vec.len();
                if len == 0 {
                    None
                } else {
                    Some(vec.remove(len - 1))
                }
            }
        }
    }
}
