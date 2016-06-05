
extern crate rustc_serialize;
extern crate bincode;
extern crate tcore;

pub mod configuration;
mod net;
mod data;

use configuration::*;
use net::UdpSocketWrapper;
use data::SimpleTestObject;

pub const PATH_TO_CONFIG_FILE: &'static str = "server_config.json";

fn main() {
    
    let creator = ServerConfBuilder::load(PATH_TO_CONFIG_FILE).unwrap();
    let conf = creator.create();
    println!("{:?}", conf);
    /*let socket = UdpSocketWrapper::bind(conf);*/
    
    let p = data::Protocol::Object(SimpleTestObject::new());
    
    let v = bincode::rustc_serialize::encode(&p, bincode::SizeLimit::Infinite);
    println!("{:?}", v);
    
    let mut file = std::fs::File::open("104.jpg").unwrap();
    let mut buf = Vec::new();
    
    use std::io::Read;
    file.read_to_end(&mut buf);
    
    let ps: Vec<data::Protocol<i32>> = data::Protocol::split(&buf);
    println!("{:?}", ps);
    let file: data::File = ps.into();
    file.save("test.jpg");    
    
}
