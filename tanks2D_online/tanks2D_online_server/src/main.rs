
extern crate rustc_serialize;
extern crate bincode;
extern crate tcore;

pub mod configuration;
mod net;
mod data;

use tcore::ConfCreator;
use configuration::*;
use net::UdpSocketWrapper;
use data::SimpleTestObject;

pub const PATH_TO_CONFIG_FILE: &'static str = "server_config.json";

fn main() {
    
    let creator = ServerConfCreator::new(PATH_TO_CONFIG_FILE).unwrap();
    let conf = creator.create().unwrap();
    println!("{:?}", conf);
    let socket = UdpSocketWrapper::bind(conf);
    
}
